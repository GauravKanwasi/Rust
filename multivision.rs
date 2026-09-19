use std::io::{self, Write};

fn main() {
    println!("=== Multivision — Multiply & Divide ===\n");

    loop {
        let num1 = read_number("Enter first number:");
        let num2 = read_number("Enter second number:");

        let product = num1 * num2;
        println!("\n{} * {} = {}", fmt(num1), fmt(num2), fmt(product));

        if num2 == 0.0 {
            println!("{} / {} = undefined (division by zero)", fmt(num1), fmt(num2));
        } else {
            let quotient = num1 / num2;
            println!("{} / {} = {}", fmt(num1), fmt(num2), fmt(quotient));
        }

        if !ask_yes_no("\nRun again? (y/n):") {
            println!("Goodbye!");
            break;
        }
        println!();
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{} ", prompt);
        io::stdout().flush().ok();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nNo more input. Exiting.");
                std::process::exit(0);
            }
            Ok(_) => {
                let trimmed = input.trim();
                match trimmed.parse::<f64>() {
                    Ok(value) if value.is_finite() => return value,
                    Ok(_) => println!("Please enter a finite number (no infinity/NaN)."),
                    Err(_) => println!("Invalid number: \"{}\". Try again.", trimmed),
                }
            }
            Err(e) => println!("Error reading input: {e}. Try again."),
        }
    }
}

fn ask_yes_no(prompt: &str) -> bool {
    print!("{} ", prompt);
    io::stdout().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    } else {
        false
    }
}

/// Formats a number without trailing zeros (e.g. 6 instead of 6.0000).
fn fmt(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{:.4}", n).trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
