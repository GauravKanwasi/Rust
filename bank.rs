use std::io::{self, Write};

struct Account {
    name: String,
    balance: f64,
}

impl Account {
    fn new(name: String) -> Self {
        Self {
            name,
            balance: 0.0,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("✔ Deposit successful.");
        } else {
            println!("✘ Invalid amount.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= 0.0 {
            println!("✘ Invalid amount.");
        } else if amount > self.balance {
            println!("✘ Insufficient balance.");
        } else {
            self.balance -= amount;
            println!("✔ Withdrawal successful.");
        }
    }

    fn show_balance(&self) {
        println!("💰 Current Balance: {:.2}", self.balance);
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt);
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("✘ Please enter a valid number."),
        }
    }
}

fn show_menu() {
    println!("\n==============================");
    println!("        BANKING SYSTEM        ");
    println!("==============================");
    println!("1) Deposit");
    println!("2) Withdraw");
    println!("3) Check Balance");
    println!("4) Exit");
    println!("==============================");
}

fn main() {
    println!("Welcome to the Banking System");
    let name = read_input("Enter your name: ");

    let mut account = Account::new(name);

    loop {
        show_menu();
        let choice = read_input("Choose an option: ");

        match choice.as_str() {
            "1" => {
                let amount = read_f64("Enter amount to deposit: ");
                account.deposit(amount);
            }
            "2" => {
                let amount = read_f64("Enter amount to withdraw: ");
                account.withdraw(amount);
            }
            "3" => {
                account.show_balance();
            }
            "4" => {
                println!("Thank you for using the banking system. Goodbye!");
                break;
            }
            _ => println!("✘ Invalid choice. Please select 1–4."),
        }
    }
}
