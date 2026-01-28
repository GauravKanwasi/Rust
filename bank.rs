use std::io::{self, Read, Write};

struct Account {
    balance: f64,
}

impl Account {
    fn new() -> Self {
        Self { balance: 0.0 }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposit successful.");
        } else {
            println!("Invalid amount.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Insufficient balance.");
        } else if amount > 0.0 {
            self.balance -= amount;
            println!("Withdrawal successful.");
        } else {
            println!("Invalid amount.");
        }
    }
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f64>().unwrap_or(0.0)
}

fn read_choice() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u8>().unwrap_or(0)
}

fn main() {
    let mut account = Account::new();

    loop {
        print!(
            "\n1) Deposit\n2) Withdraw\n3) Balance\n4) Exit\nChoice: "
        );
        io::stdout().flush().unwrap();

        match read_choice() {
            1 => {
                print!("Amount: ");
                io::stdout().flush().unwrap();
                let amt = read_number();
                account.deposit(amt);
            }
            2 => {
                print!("Amount: ");
                io::stdout().flush().unwrap();
                let amt = read_number();
                account.withdraw(amt);
            }
            3 => {
                println!("Balance: {:.2}", account.balance);
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option."),
        }
    }
}
