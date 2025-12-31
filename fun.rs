use std::io;
use std::time::Duration;
use std::thread;
use rand::Rng;

fn slow_print(message: &str) {
    for c in message.chars() {
        print!("{}", c);
        io::Write::flush(&mut io::stdout()).unwrap();
        thread::sleep(Duration::from_millis(25));
    }
    println!();
}

fn main() {
    slow_print("🎉 Welcome to FUN.RS — the most serious fun you’ll ever have in Rust.");
    slow_print("I am thinking of a number between 1 and 100.");
    slow_print("Can you guess it? Let’s find out.\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Enter your guess (or type 'quit' to escape reality):");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess = guess.trim();

        if guess.eq_ignore_ascii_case("quit") {
            println!("You gave up. The number was {}. I forgive you.", secret_number);
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That’s not a number. Rust is disappointed.");
                continue;
            }
        };

        attempts += 1;

        if guess < secret_number {
            println!("Too small. Think bigger.");
        } else if guess > secret_number {
            println!("Too big. Humble yourself.");
        } else {
            println!("\n🎯 Correct!");
            println!("You guessed the number in {} attempts.", attempts);

            match attempts {
                1 => println!("Legendary. Are you cheating?"),
                2..=5 => println!("Impressive. Rust approves."),
                6..=10 => println!("Not bad. Acceptable performance."),
                _ => println!("Eventually… success."),
            }
            break;
        }
    }

    println!("\nThanks for playing FUN.RS. May your code compile on the first try.");
}
