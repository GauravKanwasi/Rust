use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn parse_number(input: &str) -> Result<f64, String> {
    input.parse::<f64>().map_err(|_| format!("'{}' is not a valid number", input))
}

fn calculate(num1: f64, num2: f64, op: &str) -> Result<f64, String> {
    match op {
        "add" | "+" => Ok(num1 + num2),
        "sub" | "-" => Ok(num1 - num2),
        "mul" | "multiply" | "*" => Ok(num1 * num2),
        "div" | "divide" | "/" => {
            if num2 == 0.0 {
                Err("Division by zero is not allowed".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("Unknown operation '{}'. Use: add, sub, multiply, divide", op)),
    }
}

fn main() {
    let num1 = loop {
        match parse_number(&read_input("Enter the first number: ")) {
            Ok(n) => break n,
            Err(e) => eprintln!("Error: {}", e),
        }
    };

    let num2 = loop {
        match parse_number(&read_input("Enter the second number: ")) {
            Ok(n) => break n,
            Err(e) => eprintln!("Error: {}", e),
        }
    };

    let op = read_input("Operation (add/sub/multiply/divide or +/-/*/÷): ").to_lowercase();

    match calculate(num1, num2, &op) {
        Ok(result) => println!("{} {} {} = {}", num1, op, num2, result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
