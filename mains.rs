use std::env;
use std::io::{self, Write};

fn read_name_interactive() -> Result<String, String> {
    print!("Enter your name: ");
    io::stdout().flush().map_err(|_| "Output error")?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Input error")?;

    let name = input.trim().to_string();
    if name.is_empty() {
        return Err("Name cannot be empty".into());
    }

    Ok(name)
}

fn main() {
    let name = env::args()
        .nth(1)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(Ok)
        .unwrap_or_else(read_name_interactive);

    match name {
        Ok(n) => println!("Hello, {}! Welcome to Rust.", n),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
