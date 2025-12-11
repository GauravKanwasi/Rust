use std::io;

/// The main function for the multivision program.
fn main() {
    println!("--- Multivision: Multiplication and Division Program ---");
    println!("This program will multiply and divide two numbers you provide.");

    // --- First Number Input ---
    let num1 = loop {
        match get_number("Please enter the **first** number (a float):") {
            Ok(n) => break n,
            Err(e) => {
                println!("Error: {}", e);
                // Continue the loop to ask again
            }
        }
    };

    // --- Second Number Input ---
    let num2 = loop {
        match get_number("Please enter the **second** number (a float):") {
            Ok(n) => break n,
            Err(e) => {
                println!("Error: {}", e);
                // Continue the loop to ask again
            }
        }
    };

    println!("\n--- Results ---");
    
    // Perform Multiplication
    let product = num1 * num2;
    println!("Multiplication ({} * {}): {}", num1, num2, product);

    // Perform Division
    if num2 == 0.0 {
        println!("Division ({} / {}): Cannot divide by zero.", num1, num2);
    } else {
        let quotient = num1 / num2;
        println!("Division ({} / {}): {}", num1, num2, quotient);
    }
    
    println!("-------------------");
}

/// Helper function to prompt the user for input and parse it as a float (f64).
fn get_number(prompt: &str) -> Result<f64, String> {
    println!("{}", prompt);
    let mut input = String::new();

    // Read a line from standard input
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim whitespace and attempt to parse the string as an f64
            match input.trim().parse::<f64>() {
                Ok(number) => Ok(number),
                Err(_) => Err(String::from("Invalid input. Please enter a valid number (e.g., 5, 3.14).")),
            }
        }
        Err(error) => Err(format!("Failed to read line: {}", error)),
    }
}
