use std::io::{self, Write};

fn read_number<T: std::str::FromStr>() -> T {
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(value) = input.trim().parse::<T>() {
            return value;
        }
        print!("Invalid input, try again: ");
        io::stdout().flush().unwrap();
    }
}

fn mcin() {
    println!("--- Addition ---");
    print!("Enter first integer: ");
    io::stdout().flush().unwrap();
    let num1: i32 = read_number();

    print!("Enter second integer: ");
    io::stdout().flush().unwrap();
    let num2: i32 = read_number();

    let sum = num1 + num2;
    println!("Result: {}", sum);

    println!("\n---------------------\n");

    println!("--- Subtraction ---");
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let num3: f64 = read_number();

    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let num4: f64 = read_number();

    let difference = num3 - num4;
    println!("Result: {:.2}", difference);

    println!("\n---------------------\n");

    println!("--- Multiplication ---");
    let product = num3 * num4;
    println!("Result: {:.2}", product);

    println!("\n---------------------\n");

    println!("--- Division ---");
    if num4 != 0.0 {
        let quotient = num3 / num4;
        println!("Result: {:.2}", quotient);
    } else {
        println!("Division by zero is not allowed");
    }
}

fn main() {
    mcin();
}
