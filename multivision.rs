use std::io::{self, Write};

fn main() {
    println!("Multivision — Multiply & Divide");

    let num1 = read_number("Enter first number:");
    let num2 = read_number("Enter second number:");

    let product = num1 * num2;
    println!("{} * {} = {}", num1, num2, product);

    match num2 {
        0.0 => println!("{} / {} = undefined", num1, num2),
        _ => println!("{} / {} = {}", num1, num2, num1 / num2),
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{} ", prompt);
        io::stdout().flush().ok();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(value) = input.trim().parse::<f64>() {
                return value;
            }
        }

        println!("Invalid number, try again.");
    }
}
