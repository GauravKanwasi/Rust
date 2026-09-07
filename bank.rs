use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const DATA_FILE: &str = "rustbank.dat";
const MAX_BALANCE: f64 = 1_000_000_000.0;
const MAX_HISTORY_DISPLAY: usize = 20;

mod color {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const CYAN: &str = "\x1b[36m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const BLUE: &str = "\x1b[34m";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransactionKind {
    Deposit,
    Withdrawal,
}

impl fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deposit => write!(f, "Deposit"),
            Self::Withdrawal => write!(f, "Withdrawal"),
        }
    }
}

#[derive(Debug, Clone)]
struct Transaction {
    kind: TransactionKind,
    amount: f64,
    balance_after: f64,
    timestamp: u64,
}

#[derive(Debug)]
enum BankError {
    InsufficientFunds { available: f64, requested: f64 },
    InvalidAmount(f64),
    Overflow,
    InvalidInitialBalance(f64),
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientFunds {
                available,
                requested,
            } => write!(
                f,
                "Insufficient funds: balance is {:.2}, requested {:.2}",
                available, requested
            ),
            Self::InvalidAmount(amount) => write!(f, "Invalid amount: {:.2}", amount),
            Self::Overflow => write!(f, "Transaction would exceed maximum balance"),
            Self::InvalidInitialBalance(balance) => {
                write!(f, "Invalid initial balance: {:.2}", balance)
            }
        }
    }
}

impl std::error::Error for BankError {}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn format_timestamp(secs: u64) -> String {
    const DAYS_PER_400Y: i64 = 146097;
    let days_since_epoch = (secs / 86400) as i64;
    let secs_of_day = secs % 86400;
    let (h, m, s) = (
        secs_of_day / 3600,
        (secs_of_day % 3600) / 60,
        secs_of_day % 60,
    );

    let z = days_since_epoch + 719468;
    let era = if z >= 0 {
        z
    } else {
        z - DAYS_PER_400Y + 1
    } / DAYS_PER_400Y;
    let doe = (z - era * DAYS_PER_400Y) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let year = if month <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, d, h, m, s
    )
}

fn format_money(amount: f64) -> String {
    format!("{:.2}", amount)
}

struct Account {
    balance: f64,
    history: Vec<Transaction>,
}

impl Account {
    fn new(initial_balance: f64) -> Result<Self, BankError> {
        if !initial_balance.is_finite() || initial_balance < 0.0 || initial_balance > MAX_BALANCE {
            return Err(BankError::InvalidInitialBalance(initial_balance));
        }

        Ok(Self {
            balance: initial_balance,
            history: Vec::new(),
        })
    }

    fn record(&mut self, kind: TransactionKind, amount: f64) {
        self.history.push(Transaction {
            kind,
            amount,
            balance_after: self.balance,
            timestamp: now_secs(),
        });
    }

    fn deposit(&mut self, amount: f64) -> Result<f64, BankError> {
        validate_amount(amount)?;

        let new_balance = self.balance + amount;
        if !new_balance.is_finite() || new_balance > MAX_BALANCE {
            return Err(BankError::Overflow);
        }

        self.balance = new_balance;
        self.record(TransactionKind::Deposit, amount);
        Ok(self.balance)
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, BankError> {
        validate_amount(amount)?;

        if amount > self.balance {
            return Err(BankError::InsufficientFunds {
                available: self.balance,
                requested: amount,
            });
        }

        self.balance -= amount;
        // Avoid tiny floating-point residue such as 99.999999999.
        if self.balance.abs() < 0.00000001 {
            self.balance = 0.0;
        }

        self.record(TransactionKind::Withdrawal, amount);
        Ok(self.balance)
    }

    fn balance(&self) -> f64 {
        self.balance
    }

    fn history(&self) -> &[Transaction] {
        &self.history
    }

    fn total_deposited(&self) -> f64 {
        self.history
            .iter()
            .filter(|tx| tx.kind == TransactionKind::Deposit)
            .map(|tx| tx.amount)
            .sum()
    }

    fn total_withdrawn(&self) -> f64 {
        self.history
            .iter()
            .filter(|tx| tx.kind == TransactionKind::Withdrawal)
            .map(|tx| tx.amount)
            .sum()
    }

    fn save(&self, path: &str) -> io::Result<()> {
        let mut out = String::new();
        out.push_str(&format!("VERSION|2\n"));
        out.push_str(&format!("BALANCE|{}\n", self.balance));

        for tx in &self.history {
            out.push_str(&format!(
                "{}|{}|{}|{}\n",
                tx.kind, tx.amount, tx.balance_after, tx.timestamp
            ));
        }

        let temp_path = format!("{}.tmp", path);
        {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&temp_path)?;
            file.write_all(out.as_bytes())?;
            file.flush()?;
        }

        // Replace the previous file only after the temporary file was written.
        fs::rename(&temp_path, path)?;
        Ok(())
    }

    fn load(path: &str) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut lines = contents.lines();

        let first = lines.next().unwrap_or_default();

        let balance_line = if first.starts_with("VERSION|") {
            lines.next().unwrap_or_default()
        } else {
            first
        };

        let balance = balance_line
            .strip_prefix("BALANCE|")
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|v| v.is_finite() && *v >= 0.0 && *v <= MAX_BALANCE)
            .unwrap_or(0.0);

        let mut history = Vec::new();

        for line in lines {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 4 {
                continue;
            }

            let kind = match parts[0] {
                "Deposit" => TransactionKind::Deposit,
                "Withdrawal" => TransactionKind::Withdrawal,
                _ => continue,
            };

            let (Ok(amount), Ok(balance_after), Ok(timestamp)) = (
                parts[1].parse::<f64>(),
                parts[2].parse::<f64>(),
                parts[3].parse::<u64>(),
            ) else {
                continue;
            };

            if !amount.is_finite()
                || amount <= 0.0
                || !balance_after.is_finite()
                || balance_after < 0.0
                || balance_after > MAX_BALANCE
            {
                continue;
            }

            history.push(Transaction {
                kind,
                amount,
                balance_after,
                timestamp,
            });
        }

        Ok(Self { balance, history })
    }
}

fn validate_amount(amount: f64) -> Result<(), BankError> {
    if amount <= 0.0 || !amount.is_finite() {
        Err(BankError::InvalidAmount(amount))
    } else {
        Ok(())
    }
}

struct Cli {
    stdin: io::StdinLock<'static>,
}

impl Cli {
    fn new() -> Self {
        Self {
            stdin: io::stdin().lock(),
        }
    }

    fn prompt(&self, msg: &str) {
        print!("{}", msg);
        let _ = io::stdout().flush();
    }

    fn read_line(&mut self) -> Option<String> {
        let mut buffer = String::new();

        match self.stdin.read_line(&mut buffer) {
            Ok(0) => None,
            Ok(_) => Some(buffer.trim().to_owned()),
            Err(_) => None,
        }
    }

    fn read_choice(&mut self, prompt: &str, min: u8, max: u8) -> Option<u8> {
        loop {
            self.prompt(prompt);
            let line = self.read_line()?;

            match line.parse::<u8>() {
                Ok(value) if (min..=max).contains(&value) => return Some(value),
                _ => println!(
                    "  {}✗ Enter a number from {} to {}.{}",
                    color::RED,
                    min,
                    max,
                    color::RESET
                ),
            }
        }
    }

    fn read_amount(&mut self, prompt: &str) -> Option<f64> {
        loop {
            self.prompt(prompt);
            let line = self.read_line()?;

            match line.parse::<f64>() {
                Ok(value) if value.is_finite() && value > 0.0 => return Some(value),
                _ => println!(
                    "  {}✗ Enter a valid positive amount.{}",
                    color::RED,
                    color::RESET
                ),
            }
        }
    }

    fn confirm(&mut self, prompt: &str) -> Option<bool> {
        loop {
            self.prompt(prompt);
            let answer = self.read_line()?.to_lowercase();

            match answer.as_str() {
                "y" | "yes" => return Some(true),
                "n" | "no" => return Some(false),
                _ => println!(
                    "  {}✗ Please enter y/yes or n/no.{}",
                    color::RED,
                    color::RESET
                ),
            }
        }
    }
}

fn clear_screen() {
    // ANSI works in modern Windows terminals and Unix-like terminals.
    print!("\x1b[2J\x1b[H");
}

fn print_header() {
    println!();
    println!(
        "{}{}╔════════════════════════════════════════════════════╗{}",
        color::BOLD,
        color::CYAN,
        color::RESET
    );
    println!(
        "{}║{}{:^52}{}{}║{}",
        color::BOLD,
        color::CYAN,
        "RUSTBANK",
        color::RESET,
        color::CYAN,
        color::RESET
    );
    println!(
        "{}║{}{:^52}{}{}║{}",
        color::CYAN,
        color::DIM,
        "Secure • Simple • Dependency-Free",
        color::RESET,
        color::CYAN,
        color::RESET
    );
    println!(
        "{}╚════════════════════════════════════════════════════╝{}",
        color::CYAN,
        color::RESET
    );
}

fn print_separator() {
    println!("{}{}{}", color::DIM, "─".repeat(54), color::RESET);
}

fn print_menu(account: &Account) {
    print_separator();
    println!(
        "  {}CURRENT BALANCE{}  {}₹ {}{}",
        color::BOLD,
        color::RESET,
        color::GREEN,
        format_money(account.balance()),
        color::RESET
    );
    println!(
        "  {}Transactions:{} {}",
        color::DIM,
        color::RESET,
        account.history().len()
    );
    print_separator();

    println!("  {}1){} Deposit", color::CYAN, color::RESET);
    println!("  {}2){} Withdraw", color::CYAN, color::RESET);
    println!("  {}3){} Balance & Statistics", color::CYAN, color::RESET);
    println!("  {}4){} Transaction History", color::CYAN, color::RESET);
    println!("  {}5){} Save Account", color::CYAN, color::RESET);
    println!("  {}6){} Exit", color::CYAN, color::RESET);

    print_separator();
}

fn print_stats(account: &Account) {
    let deposits = account.total_deposited();
    let withdrawals = account.total_withdrawn();
    let transactions = account.history().len();

    println!(
        "\n{}{} ACCOUNT OVERVIEW {}{}",
        color::BOLD,
        color::BLUE,
        color::RESET,
        color::DIM
    );
    print_separator();
    println!("  Current balance : ₹{}", format_money(account.balance()));
    println!("  Total deposited : ₹{}", format_money(deposits));
    println!("  Total withdrawn : ₹{}", format_money(withdrawals));
    println!("  Transactions    : {}", transactions);
    println!(
        "  Available limit : ₹{}",
        format_money(MAX_BALANCE - account.balance())
    );
    print_separator();
}

fn print_history(history: &[Transaction]) {
    if history.is_empty() {
        println!(
            "\n  {}No transactions yet.{}",
            color::YELLOW,
            color::RESET
        );
        return;
    }

    println!(
        "\n{}{} TRANSACTION HISTORY {}{}",
        color::BOLD,
        color::MAGENTA,
        color::RESET,
        color::DIM
    );
    print_separator();

    println!(
        "  {:<19} {:<12} {:>12} {:>12}",
        "WHEN", "TYPE", "AMOUNT", "BALANCE"
    );
    print_separator();

    for tx in history.iter().rev().take(MAX_HISTORY_DISPLAY) {
        let signed_amount = match tx.kind {
            TransactionKind::Deposit => tx.amount,
            TransactionKind::Withdrawal => -tx.amount,
        };

        let amount_color = if signed_amount >= 0.0 {
            color::GREEN
        } else {
            color::RED
        };

        println!(
            "  {:<19} {:<12} {}{:>+12.2}{} {:>12.2}",
            format_timestamp(tx.timestamp),
            tx.kind,
            amount_color,
            signed_amount,
            color::RESET,
            tx.balance_after
        );
    }

    if history.len() > MAX_HISTORY_DISPLAY {
        println!(
            "\n  {}Showing the latest {} of {} transactions.{}",
            color::DIM,
            MAX_HISTORY_DISPLAY,
            history.len(),
            color::RESET
        );
    }

    print_separator();
}

fn save_with_message(account: &Account) {
    match account.save(DATA_FILE) {
        Ok(()) => println!(
            "  {}✓ Account saved to {}{}",
            color::GREEN,
            DATA_FILE,
            color::RESET
        ),
        Err(error) => println!(
            "  {}✗ Save failed: {}{}",
            color::RED,
            error,
            color::RESET
        ),
    }
}

fn main() {
    clear_screen();
    print_header();

    let mut account = if Path::new(DATA_FILE).exists() {
        match Account::load(DATA_FILE) {
            Ok(account) => {
                println!(
                    "\n  {}✓ Existing account loaded.{}",
                    color::GREEN,
                    color::RESET
                );
                account
            }
            Err(error) => {
                println!(
                    "\n  {}⚠ Could not load account: {}{}",
                    color::YELLOW,
                    error,
                    color::RESET
                );
                Account::new(0.0).expect("zero is a valid balance")
            }
        }
    } else {
        println!(
            "\n  {}New account created.{}",
            color::YELLOW,
            color::RESET
        );
        Account::new(0.0).expect("zero is a valid balance")
    };

    let mut cli = Cli::new();

    loop {
        println!();
        print_menu(&account);

        let Some(choice) = cli.read_choice("  Choice: ", 1, 6) else {
            println!("\n  Goodbye!");
            break;
        };

        match choice {
            1 => {
                let Some(amount) = cli.read_amount("  Deposit amount: ₹") else {
                    break;
                };

                match account.deposit(amount) {
                    Ok(balance) => {
                        println!(
                            "  {}✓ Deposited ₹{:.2}{}",
                            color::GREEN,
                            amount,
                            color::RESET
                        );
                        println!("  New balance: ₹{:.2}", balance);
                        save_with_message(&account);
                    }
                    Err(error) => {
                        println!("  {}✗ {}{}", color::RED, error, color::RESET);
                    }
                }
            }

            2 => {
                let Some(amount) = cli.read_amount("  Withdrawal amount: ₹") else {
                    break;
                };

                match account.withdraw(amount) {
                    Ok(balance) => {
                        println!(
                            "  {}✓ Withdrew ₹{:.2}{}",
                            color::GREEN,
                            amount,
                            color::RESET
                        );
                        println!("  New balance: ₹{:.2}", balance);
                        save_with_message(&account);
                    }
                    Err(error) => {
                        println!("  {}✗ {}{}", color::RED, error, color::RESET);
                    }
                }
            }

            3 => print_stats(&account),

            4 => print_history(account.history()),

            5 => save_with_message(&account),

            6 => {
                println!();
                if account.history().is_empty() {
                    println!("  No transactions were made.");
                } else if cli.confirm("  Save before exiting? [Y/n]: ").unwrap_or(true) {
                    save_with_message(&account);
                }

                println!(
                    "\n  {}Thank you for using RustBank. Goodbye!{}",
                    color::BOLD,
                    color::RESET
                );
                break;
            }

            _ => unreachable!(),
        }

        println!();
        let _ = cli.confirm("  Press y to continue: ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_account_starts_at_zero() {
        let account = Account::new(0.0).unwrap();
        assert_eq!(account.balance(), 0.0);
        assert!(account.history().is_empty());
    }

    #[test]
    fn invalid_initial_balance_is_rejected() {
        assert!(Account::new(-1.0).is_err());
        assert!(Account::new(f64::NAN).is_err());
        assert!(Account::new(MAX_BALANCE + 1.0).is_err());
    }

    #[test]
    fn deposit_increases_balance_and_records_history() {
        let mut account = Account::new(0.0).unwrap();

        let balance = account.deposit(100.0).unwrap();

        assert_eq!(balance, 100.0);
        assert_eq!(account.history().len(), 1);
        assert_eq!(account.history()[0].kind, TransactionKind::Deposit);
        assert_eq!(account.history()[0].amount, 100.0);
    }

    #[test]
    fn withdrawal_decreases_balance() {
        let mut account = Account::new(50.0).unwrap();

        let balance = account.withdraw(20.0).unwrap();

        assert_eq!(balance, 30.0);
        assert_eq!(account.history().len(), 1);
    }

    #[test]
    fn withdrawal_more_than_balance_fails_without_mutation() {
        let mut account = Account::new(10.0).unwrap();

        let error = account.withdraw(20.0).unwrap_err();

        assert!(matches!(
            error,
            BankError::InsufficientFunds {
                available: 10.0,
                requested: 20.0
            }
        ));
        assert_eq!(account.balance(), 10.0);
        assert!(account.history().is_empty());
    }

    #[test]
    fn invalid_amounts_are_rejected() {
        let mut account = Account::new(10.0).unwrap();

        assert!(account.deposit(-5.0).is_err());
        assert!(account.deposit(0.0).is_err());
        assert!(account.deposit(f64::NAN).is_err());
        assert!(account.deposit(f64::INFINITY).is_err());

        assert!(account.withdraw(-5.0).is_err());
        assert!(account.withdraw(0.0).is_err());
        assert!(account.withdraw(f64::NAN).is_err());
    }

    #[test]
    fn deposit_beyond_max_balance_is_rejected() {
        let mut account = Account::new(MAX_BALANCE - 1.0).unwrap();

        let error = account.deposit(10.0).unwrap_err();

        assert!(matches!(error, BankError::Overflow));
        assert_eq!(account.balance(), MAX_BALANCE - 1.0);
    }

    #[test]
    fn statistics_are_calculated_correctly() {
        let mut account = Account::new(0.0).unwrap();

        account.deposit(100.0).unwrap();
        account.deposit(50.0).unwrap();
        account.withdraw(30.0).unwrap();

        assert_eq!(account.total_deposited(), 150.0);
        assert_eq!(account.total_withdrawn(), 30.0);
        assert_eq!(account.balance(), 120.0);
    }

    #[test]
    fn save_and_load_round_trip() {
        let mut account = Account::new(0.0).unwrap();
        account.deposit(100.0).unwrap();
        account.withdraw(30.0).unwrap();

        let path = "test_rustbank_roundtrip.dat";
        account.save(path).unwrap();

        let loaded = Account::load(path).unwrap();

        assert_eq!(loaded.balance(), account.balance());
        assert_eq!(loaded.history().len(), account.history().len());
        assert_eq!(loaded.total_deposited(), account.total_deposited());
        assert_eq!(loaded.total_withdrawn(), account.total_withdrawn());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn legacy_file_without_version_still_loads() {
        let path = "test_rustbank_legacy.dat";

        fs::write(
            path,
            "BALANCE|70\nDeposit|100|100|1000\nWithdrawal|30|70|1001\n",
        )
        .unwrap();

        let account = Account::load(path).unwrap();

        assert_eq!(account.balance(), 70.0);
        assert_eq!(account.history().len(), 2);

        let _ = fs::remove_file(path);
    }
}
