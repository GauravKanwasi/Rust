use std::env;
use std::io::{self, Write};

fn main() {
    // Try to read the name from command-line arguments
    let name = env::args().nth(1).unwrap_or_else(|| {
        // Fallback to interactive input
        print!("Enter your name: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input.trim().to_string()
    });

    if name.is_empty() {
        eprintln!("Error: Name cannot be empty.");
        std::process::exit(1);
    }

    println!("Hello, {}! Welcome to Rust.", name);
}
