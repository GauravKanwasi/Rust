use std::io::{self, Write};

fn main() {
    println!("==================================");
    println!(" Interactive Rust Program");
    println!(" Type 'help' to see available commands");
    println!(" Type 'exit' to quit");
    println!("==================================");

    loop {
        // Prompt
        print!("\n> ");
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let command = input.trim().to_lowercase();

        match command.as_str() {
            "help" => show_help(),
            "greet" => greet_user(),
            "add" => add_numbers(),
            "exit" => {
                println!("Goodbye! 👋");
                break;
            }
            "" => {}
            _ => println!("Unknown command. Type 'help' for options."),
        }
    }
}

fn show_help() {
    println!("\nAvailable commands:");
    println!("  help  - Show this help menu");
    println!("  greet - Enter your name and get a greeting");
    println!("  add   - Add two numbers");
    println!("  exit  - Quit the program");
}

fn greet_user() {
    let mut name = String::new();
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("Hello, {}! Welcome to Rust 🚀", name.trim());
}

fn add_numbers() {
    let a = read_number("Enter first number: ");
    let b = read_number("Enter second number: ");

    println!("Result: {} + {} = {}", a, b, a + b);
}

fn read_number(prompt: &str) -> i32 {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read number");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer."),
        }
    }
}
