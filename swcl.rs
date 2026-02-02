use std::io;

fn main() {
    println!("--- Rust Match Calculator ---");
    
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    // Input first number
    println!("Enter first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please type a number!");

    // Input operator
    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let operator = operator.trim();

    // Input second number
    println!("Enter second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please type a number!");

    // The "Switch Case" equivalent in Rust
    match operator {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2);
            } else {
                println!("Error: Cannot divide by zero.");
            }
        },
        _ => println!("Error: Invalid operator!"), // The default/fallback case
    }
}
