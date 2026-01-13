use std::io;

fn main() {
    println!("Simple Calculator in Rust");
    println!("Enter the first number:");

    let first = read_number();
    println!("Enter the second number:");

    let second = read_number();
    println!("Enter operation (+, -, *, /):");

    let operation = read_operation();

    let result = match operation {
        '+' => first + second,
        '-' => first - second,
        '*' => first * second,
        '/' => {
            if second == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            first / second
        }
        _ => {
            println!("Invalid operation.");
            return;
        }
    };

    println!("Result: {}", result);
}

fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid number. Please enter a valid numeric value:");
            }
        }
    }
}

fn read_operation() -> char {
    loop {
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Failed to read input");

        let trimmed = op.trim();
        if trimmed.len() == 1 {
            let ch = trimmed.chars().next().unwrap();
            if "+-*/".contains(ch) {
                return ch;
            }
        }

        println!("Invalid operator. Please enter one of (+, -, *, /):");
    }
}
