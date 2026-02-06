use std::io;

fn main() {
    println!("Interactive Calculator (Rust)");
    println!("Type 'q' at any prompt to exit.\n");

    loop {
        println!("Enter the first number:");
        let first = match read_number_or_quit() {
            Some(n) => n,
            None => break,
        };

        println!("Enter the second number:");
        let second = match read_number_or_quit() {
            Some(n) => n,
            None => break,
        };

        println!("Enter operation (+, -, *, /):");
        let operation = match read_operation_or_quit() {
            Some(op) => op,
            None => break,
        };

        let result = match operation {
            '+' => first + second,
            '-' => first - second,
            '*' => first * second,
            '/' => {
                if second == 0.0 {
                    println!("Error: Division by zero is not allowed.\n");
                    continue;
                }
                first / second
            }
            _ => {
                println!("Invalid operation.\n");
                continue;
            }
        };

        println!("Result: {}\n", result);
    }

    println!("Calculator closed.");
}

fn read_number_or_quit() -> Option<f64> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("q") {
            return None;
        }

        match trimmed.parse::<f64>() {
            Ok(num) => return Some(num),
            Err(_) => println!("Invalid number. Enter a valid numeric value:"),
        }
    }
}

fn read_operation_or_quit() -> Option<char> {
    loop {
        let mut op = String::new();
        io::stdin().read_line(&mut op).ok()?;

        let trimmed = op.trim();

        if trimmed.eq_ignore_ascii_case("q") {
            return None;
        }

        if trimmed.len() == 1 {
            let ch = trimmed.chars().next().unwrap();
            if "+-*/".contains(ch) {
                return Some(ch);
            }
        }

        println!("Invalid operator. Enter one of (+, -, *, /):");
    }
}
