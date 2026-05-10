use std::io::{self, Write};

fn main() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("      Advanced Calculator     ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Type 'q' anytime to quit\n");

    loop {
        let first = match read_number("Enter first number") {
            Some(v) => v,
            None => break,
        };

        let operation = match read_operation() {
            Some(v) => v,
            None => break,
        };

        let second = match read_number("Enter second number") {
            Some(v) => v,
            None => break,
        };

        let result = match operation {
            '+' => Ok(first + second),
            '-' => Ok(first - second),
            '*' => Ok(first * second),
            '/' if second != 0.0 => Ok(first / second),
            '/' => Err("Division by zero is not allowed"),
            '%' if second != 0.0 => Ok(first % second),
            '%' => Err("Modulo by zero is not allowed"),
            '^' => Ok(first.powf(second)),
            _ => Err("Unsupported operation"),
        };

        match result {
            Ok(value) => {
                println!("\nResult");
                println!("{} {} {} = {}\n", first, operation, second, value);
            }
            Err(message) => {
                println!("\nError: {}\n", message);
            }
        }
    }

    println!("\nCalculator closed");
}

fn read_number(prompt: &str) -> Option<f64> {
    loop {
        print!("{}: ", prompt);
        io::stdout().flush().ok()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;

        let value = input.trim();

        if value.eq_ignore_ascii_case("q") {
            return None;
        }

        match value.parse::<f64>() {
            Ok(num) => return Some(num),
            Err(_) => println!("Invalid number\n"),
        }
    }
}

fn read_operation() -> Option<char> {
    loop {
        print!("Operation (+, -, *, /, %, ^): ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;

        let value = input.trim();

        if value.eq_ignore_ascii_case("q") {
            return None;
        }

        if value.len() == 1 {
            let op = value.chars().next()?;
            if "+-*/%^".contains(op) {
                return Some(op);
            }
        }

        println!("Invalid operation\n");
    }
}
