use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const DATA_FILE: &str = "rustbank.dat";

// ---------- ANSI color helpers (no external deps) ----------
mod color {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const CYAN: &str = "\x1b[36m";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransactionKind {
    Deposit,
    Withdrawal,
}

impl fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionKind::Deposit => write!(f, "Deposit"),
            TransactionKind::Withdrawal => write!(f, "Withdrawal"),
        }
    }
}

#[derive(Debug, Clone)]
struct Transaction {
    kind: TransactionKind,
    amount: f64,
    balance_after: f64,
    timestamp: u64, // unix seconds
}

#[derive(Debug)]
enum BankError {
    InsufficientFunds { available: f64, requested: f64 },
    InvalidAmount(f64),
    Overflow,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::InsufficientFunds { available, requested } => write!(
                f,
                "Insufficient funds: balance is {:.2}, requested {:.2}",
                available, requested
            ),
            BankError::InvalidAmount(a) => write!(f, "Invalid amount: {:.2}", a),
            BankError::Overflow => write!(f, "Transaction would exceed maximum balance"),
        }
    }
}

impl std::error::Error for BankError {}

const MAX_BALANCE: f64 = 1_000_000_000.0;

#[derive(Debug)]
struct Account {
    balance: f64,
    history: Vec<Transaction>,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// Minimal, dependency-free "date" formatting (UTC, good enough for a CLI log).
fn format_timestamp(secs: u64) -> String {
    const DAYS_PER_400Y: i64 = 146097;
    let days_since_epoch = (secs / 86400) as i64;
    let secs_of_day = secs % 86400;
    let (h, m, s) = (secs_of_day / 3600, (secs_of_day % 3600) / 60, secs_of_day % 60);

    // Civil-from-days algorithm (Howard Hinnant), proleptic Gregorian.
    let z = days_since_epoch + 719468;
    let era = if z >= 0 { z } else { z - DAYS_PER_400Y + 1 } / DAYS_PER_400Y;
    let doe = (z - era * DAYS_PER_400Y) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m2 = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let y = if m2 <= 2 { y + 1 } else { y };

    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m2, d, h, m, s)
}

impl Account {
    fn new(initial_balance: f64) -> Self {
        Self {
            balance: initial_balance,
            history: Vec::new(),
        }
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
        if amount <= 0.0 || !amount.is_finite() {
            return Err(BankError::InvalidAmount(amount));
        }
        let new_balance = self.balance + amount;
        if !new_balance.is_finite() || new_balance > MAX_BALANCE {
            return Err(BankError::Overflow);
        }
        self.balance = new_balance;
        self.record(TransactionKind::Deposit, amount);
        Ok(self.balance)
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, BankError> {
        if amount <= 0.0 || !amount.is_finite() {
            return Err(BankError::InvalidAmount(amount));
        }
        if amount > self.balance {
            return Err(BankError::InsufficientFunds {
                available: self.balance,
                requested: amount,
            });
        }
        self.balance -= amount;
        self.record(TransactionKind::Withdrawal, amount);
        Ok(self.balance)
    }

    fn balance(&self) -> f64 {
        self.balance
    }

    fn history(&self) -> &[Transaction] {
        &self.history
    }

    /// Serialize to a tiny pipe-delimited format: one line per transaction,
    /// plus a leading balance line. Dependency-free (no serde required).
    fn save(&self, path: &str) -> io::Result<()> {
        let mut out = String::new();
        out.push_str(&format!("BALANCE|{}\n", self.balance));
        for tx in &self.history {
            out.push_str(&format!(
                "{}|{}|{}|{}\n",
                tx.kind, tx.amount, tx.balance_after, tx.timestamp
            ));
        }
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        f.write_all(out.as_bytes())
    }

    fn load(path: &str) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut lines = contents.lines();
        let balance = lines
            .next()
            .and_then(|l| l.strip_prefix("BALANCE|"))
            .and_then(|v| v.parse::<f64>().ok())
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
            let (Ok(amount), Ok(balance_after), Ok(timestamp)) =
                (parts[1].parse(), parts[2].parse(), parts[3].parse())
            else {
                continue;
            };
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
        io::stdout().flush().expect("flush failed");
    }

    fn read_line(&mut self) -> Option<String> {
        let mut buf = String::new();
        match self.stdin.read_line(&mut buf) {
            Ok(0) => None, // EOF
            Ok(_) => Some(buf.trim().to_owned()),
            Err(_) => None,
        }
    }

    /// Keeps asking until the user enters a valid u8, or EOF/quit ends the loop.
    fn read_u8(&mut self, prompt: &str) -> Option<u8> {
        loop {
            self.prompt(prompt);
            let line = self.read_line()?;
            match line.parse::<u8>() {
                Ok(v) => return Some(v),
                Err(_) => println!(
                    "  {}Please enter a whole number.{}",
                    color::RED,
                    color::RESET
                ),
            }
        }
    }

    /// Keeps asking until the user enters a valid positive finite f64, or EOF ends the loop.
    fn read_f64(&mut self, prompt: &str) -> Option<f64> {
        loop {
            self.prompt(prompt);
            let line = self.read_line()?;
            match line.parse::<f64>() {
                Ok(v) if v.is_finite() => return Some(v),
                _ => println!(
                    "  {}Please enter a valid number.{}",
                    color::RED,
                    color::RESET
                ),
            }
        }
    }
}

fn print_separator() {
    println!("{}{}{}", color::DIM, "─".repeat(42), color::RESET);
}

fn print_menu(balance: f64) {
    print_separator();
    println!(
        "  {}{}BANK ACCOUNT{}   {}Balance: {:.2}{}",
        color::BOLD,
        color::CYAN,
        color::RESET,
        color::GREEN,
        balance,
        color::RESET
    );
    print_separator();
    println!("  1) Deposit");
    println!("  2) Withdraw");
    println!("  3) Balance");
    println!("  4) History");
    println!("  5) Save");
    println!("  6) Exit");
    print_separator();
}

fn print_history(history: &[Transaction]) {
    if history.is_empty() {
        println!("  No transactions yet.");
        return;
    }
    print_separator();
    println!(
        "  {:<19} {:<12} {:>10}  {:>10}",
        "WHEN", "TYPE", "AMOUNT", "BALANCE"
    );
    print_separator();
    for tx in history.iter().rev().take(20) {
        let signed_amount = match tx.kind {
            TransactionKind::Deposit => tx.amount,
            TransactionKind::Withdrawal => -tx.amount,
        };
        let line_color = if signed_amount >= 0.0 {
            color::GREEN
        } else {
            color::RED
        };
        println!(
            "  {:<19} {:<12} {}{:>+10.2}{}  {:>10.2}",
            format_timestamp(tx.timestamp),
            tx.kind.to_string(),
            line_color,
            signed_amount,
            color::RESET,
            tx.balance_after
        );
    }
    print_separator();
}

fn main() {
    let mut account = match Account::load(DATA_FILE) {
        Ok(acc) => {
            println!(
                "\n{}Loaded existing account from {} (balance: {:.2}){}",
                color::YELLOW,
                DATA_FILE,
                acc.balance(),
                color::RESET
            );
            acc
        }
        Err(_) => Account::new(0.0),
    };

    let mut cli = Cli::new();
    println!("\n{}Welcome to RustBank CLI{}", color::BOLD, color::RESET);

    loop {
        println!();
        print_menu(account.balance());
        let choice = match cli.read_u8("  Choice: ") {
            Some(c) => c,
            None => break, // EOF (e.g. Ctrl-D) — exit cleanly
        };
        println!();

        match choice {
            1 => match cli.read_f64("  Amount to deposit: ") {
                Some(amount) => match account.deposit(amount) {
                    Ok(bal) => println!(
                        "  {}✓ Deposited {:.2}. Balance: {:.2}{}",
                        color::GREEN,
                        amount,
                        bal,
                        color::RESET
                    ),
                    Err(e) => println!("  {}✗ {}{}", color::RED, e, color::RESET),
                },
                None => break,
            },

            2 => match cli.read_f64("  Amount to withdraw: ") {
                Some(amount) => match account.withdraw(amount) {
                    Ok(bal) => println!(
                        "  {}✓ Withdrew {:.2}. Balance: {:.2}{}",
                        color::GREEN,
                        amount,
                        bal,
                        color::RESET
                    ),
                    Err(e) => println!("  {}✗ {}{}", color::RED, e, color::RESET),
                },
                None => break,
            },

            3 => println!("  Current balance: {:.2}", account.balance()),

            4 => print_history(account.history()),

            5 => match account.save(DATA_FILE) {
                Ok(()) => println!(
                    "  {}✓ Saved to {}{}",
                    color::GREEN,
                    DATA_FILE,
                    color::RESET
                ),
                Err(e) => println!("  {}✗ Save failed: {}{}", color::RED, e, color::RESET),
            },

            6 => {
                if let Err(e) = account.save(DATA_FILE) {
                    println!("  {}✗ Save failed: {}{}", color::RED, e, color::RESET);
                }
                println!("  Goodbye!");
                break;
            }

            _ => println!(
                "  {}✗ Unknown option. Choose 1–6.{}",
                color::RED,
                color::RESET
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_increases_balance_and_records_history() {
        let mut acc = Account::new(0.0);
        let bal = acc.deposit(100.0).unwrap();
        assert_eq!(bal, 100.0);
        assert_eq!(acc.history().len(), 1);
        assert_eq!(acc.history()[0].kind, TransactionKind::Deposit);
    }

    #[test]
    fn withdraw_decreases_balance() {
        let mut acc = Account::new(50.0);
        let bal = acc.withdraw(20.0).unwrap();
        assert_eq!(bal, 30.0);
    }

    #[test]
    fn withdraw_more_than_balance_fails() {
        let mut acc = Account::new(10.0);
        let err = acc.withdraw(20.0).unwrap_err();
        assert!(matches!(err, BankError::InsufficientFunds { .. }));
        assert_eq!(acc.balance(), 10.0); // unchanged
    }

    #[test]
    fn negative_or_nan_amounts_are_rejected() {
        let mut acc = Account::new(10.0);
        assert!(acc.deposit(-5.0).is_err());
        assert!(acc.deposit(0.0).is_err());
        assert!(acc.deposit(f64::NAN).is_err());
        assert!(acc.withdraw(-5.0).is_err());
    }

    #[test]
    fn deposit_beyond_max_balance_overflows() {
        let mut acc = Account::new(MAX_BALANCE - 1.0);
        let err = acc.deposit(10.0).unwrap_err();
        assert!(matches!(err, BankError::Overflow));
    }

    #[test]
    fn save_and_load_round_trip() {
        let mut acc = Account::new(0.0);
        acc.deposit(100.0).unwrap();
        acc.withdraw(30.0).unwrap();

        let path = "test_rustbank_roundtrip.dat";
        acc.save(path).unwrap();
        let loaded = Account::load(path).unwrap();

        assert_eq!(loaded.balance(), acc.balance());
        assert_eq!(loaded.history().len(), acc.history().len());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn history_signed_amount_is_correct_not_string_roundtrip() {
        // Regression test: the original code built the signed amount by
        // formatting into a string and re-parsing it back to f64, which was
        // fragile. We now compute it directly.
        let mut acc = Account::new(0.0);
        acc.deposit(12.5).unwrap();
        acc.withdraw(4.25).unwrap();

        let deposit_tx = &acc.history()[0];
        let withdrawal_tx = &acc.history()[1];

        let deposit_signed = match deposit_tx.kind {
            TransactionKind::Deposit => deposit_tx.amount,
            TransactionKind::Withdrawal => -deposit_tx.amount,
        };
        let withdrawal_signed = match withdrawal_tx.kind {
            TransactionKind::Deposit => withdrawal_tx.amount,
            TransactionKind::Withdrawal => -withdrawal_tx.amount,
        };

        assert_eq!(deposit_signed, 12.5);
        assert_eq!(withdrawal_signed, -4.25);
    }
}
