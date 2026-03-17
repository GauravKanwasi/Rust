use std::fmt;
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
struct Account {
    balance: f64,
    history: Vec<Transaction>,
}

impl Account {
    fn new(initial_balance: f64) -> Self {
        Self {
            balance: initial_balance,
            history: Vec::new(),
        }
    }

    fn deposit(&mut self, amount: f64) -> Result<f64, BankError> {
        if amount <= 0.0 || !amount.is_finite() {
            return Err(BankError::InvalidAmount(amount));
        }
        let new_balance = self.balance + amount;
        if !new_balance.is_finite() {
            return Err(BankError::Overflow);
        }
        self.balance = new_balance;
        self.history.push(Transaction {
            kind: TransactionKind::Deposit,
            amount,
            balance_after: self.balance,
        });
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
        self.history.push(Transaction {
            kind: TransactionKind::Withdrawal,
            amount,
            balance_after: self.balance,
        });
        Ok(self.balance)
    }

    fn balance(&self) -> f64 {
        self.balance
    }

    fn history(&self) -> &[Transaction] {
        &self.history
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

    fn read_line(&mut self) -> String {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap_or(0);
        buf.trim().to_owned()
    }

    fn read_u8(&mut self, prompt: &str) -> u8 {
        self.prompt(prompt);
        self.read_line().parse::<u8>().unwrap_or(0)
    }

    fn read_f64(&mut self, prompt: &str) -> Option<f64> {
        self.prompt(prompt);
        self.read_line().parse::<f64>().ok()
    }
}

fn print_separator() {
    println!("{}", "─".repeat(38));
}

fn print_menu() {
    print_separator();
    println!("  BANK ACCOUNT");
    print_separator();
    println!("  1) Deposit");
    println!("  2) Withdraw");
    println!("  3) Balance");
    println!("  4) History");
    println!("  5) Exit");
    print_separator();
}

fn print_history(history: &[Transaction]) {
    if history.is_empty() {
        println!("No transactions yet.");
        return;
    }
    print_separator();
    println!("  {:<12} {:>10}  {:>10}", "TYPE", "AMOUNT", "BALANCE");
    print_separator();
    for tx in history.iter().rev().take(20) {
        let sign = if tx.kind == TransactionKind::Deposit { "+" } else { "-" };
        println!(
            "  {:<12} {:>+10.2}  {:>10.2}",
            tx.kind.to_string(),
            format!("{}{:.2}", sign, tx.amount).parse::<f64>().unwrap_or(0.0),
            tx.balance_after
        );
    }
    print_separator();
}

fn main() {
    let mut account = Account::new(0.0);
    let mut cli = Cli::new();

    println!("\nWelcome to RustBank CLI");

    loop {
        println!();
        print_menu();
        let choice = cli.read_u8("  Choice: ");
        println!();

        match choice {
            1 => match cli.read_f64("  Amount to deposit: ") {
                Some(amount) => match account.deposit(amount) {
                    Ok(bal) => println!("  ✓ Deposited {:.2}. Balance: {:.2}", amount, bal),
                    Err(e) => println!("  ✗ {}", e),
                },
                None => println!("  ✗ Please enter a valid number."),
            },

            2 => match cli.read_f64("  Amount to withdraw: ") {
                Some(amount) => match account.withdraw(amount) {
                    Ok(bal) => println!("  ✓ Withdrew {:.2}. Balance: {:.2}", amount, bal),
                    Err(e) => println!("  ✗ {}", e),
                },
                None => println!("  ✗ Please enter a valid number."),
            },

            3 => println!("  Current balance: {:.2}", account.balance()),

            4 => print_history(account.history()),

            5 => {
                println!("  Goodbye!");
                break;
            }

            _ => println!("  ✗ Unknown option. Choose 1–5."),
        }
    }
}
