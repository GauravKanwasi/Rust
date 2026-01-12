use std::io;

fn main() {
    // Read first number
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");

    // Read second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    // Read operation
    println!("Choose an operation (add, sub, multiply, divide):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let op = operation.trim().to_lowercase();

    // Perform the selected operation
    let result = match op.as_str() {
        "add" => num1 + num2,
        "sub" => num1 - num2,
        "multiply" => num1 * num2,
        "divide" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operation selected.");
            return;
        }
    };

    // Output result
    println!("Result: {}", result);
}
