use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("3. Celsius to Kelvin");
    println!("4. Kelvin to Celsius");
    println!("5. Fahrenheit to Kelvin");
    println!("6. Kelvin to Fahrenheit");
    println!("Enter your choice (1-6): ");

    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read input");
    let choice: u32 = choice_input.trim().parse().expect("Invalid choice");

    println!("Enter temperature value:");
    let mut temp_input = String::new();
    io::stdin().read_line(&mut temp_input).expect("Failed to read input");
    let temp: f64 = temp_input.trim().parse().expect("Invalid number");

    let result = match choice {
        1 => (temp * 9.0 / 5.0) + 32.0,            // C → F
        2 => (temp - 32.0) * 5.0 / 9.0,            // F → C
        3 => temp + 273.15,                        // C → K
        4 => temp - 273.15,                        // K → C
        5 => (temp - 32.0) * 5.0 / 9.0 + 273.15,    // F → K
        6 => (temp - 273.15) * 9.0 / 5.0 + 32.0,    // K → F
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    println!("Converted Temperature: {:.2}", result);
}
