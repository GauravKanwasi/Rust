use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::process::ExitCode;

/// Prompts the user via stdin if no CLI argument is provided.
fn read_name_interactive() -> Result<String, Box<dyn Error>> {
    print!("Enter your name: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let name = input.trim();
    if name.is_empty() {
        return Err("Name cannot be empty".into());
    }

    Ok(name.to_string())
}

/// Retrieves the name from CLI arguments, falling back to interactive input.
fn get_name() -> Result<String, Box<dyn Error>> {
    if let Some(arg) = env::args().nth(1) {
        let trimmed = arg.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
    
    read_name_interactive()
}

fn main() -> ExitCode {
    match get_name() {
        Ok(name) => {
            println!("Hello, {name}! Welcome to Rust.");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
