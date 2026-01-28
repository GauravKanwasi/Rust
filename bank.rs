use std::io;

struct Account {
    name: String,
    balance: f64,
}

impl Account {
    fn new(name: String) -> Account {
        Account {
            name,
            balance: 0.0,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited successfully.");
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Insufficient balance.");
        } else {
            self.balance -= amount;
            println!("Withdrawal successful.");
        }
    }

    fn display_balance(&self) {
        println!("Current Balance: {:.2}", self.balance);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("===== Simple Banking System (Rust) =====");
    println!("Enter your name:");
    let name = read_input();

    let mut account = Account::new(name);

    loop {
        println!("\n1. Deposit");
        println!("2. Withdraw");
        println!("3. Check Balance");
        println!("4. Exit");
        println!("Enter your choice:");

        let choice = read_input();

        match choice.as_str() {
            "1" => {
                println!("Enter amount to deposit:");
                let amount: f64 = read_input().parse().unwrap_or(0.0);
                account.deposit(amount);
            }
            "2" => {
                println!("Enter amount to withdraw:");
                let amount: f64 = read_input().parse().unwrap_or(0.0);
                account.withdraw(amount);
            }
            "3" => {
                account.display_balance();
            }
            "4" => {
                println!("Thank you for using the banking system.");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}
