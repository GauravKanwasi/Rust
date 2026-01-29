use std::io;

fn main() {
    loop {
        println!("\n=== Unit Converter ===");
        println!("1. Meters to Kilometers");
        println!("2. Kilometers to Meters");
        println!("3. Kilograms to Grams");
        println!("4. Grams to Kilograms");
        println!("5. Celsius to Fahrenheit");
        println!("6. Fahrenheit to Celsius");
        println!("7. Exit");
        print!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid choice.");
                continue;
            }
        };

        if choice == 7 {
            println!("Exiting...");
            break;
        }

        print!("Enter value to convert: ");
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
        let value: f64 = match value.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid number.");
                continue;
            }
        };

        let result = match choice {
            1 => value / 1000.0,
            2 => value * 1000.0,
            3 => value * 1000.0,
            4 => value / 1000.0,
            5 => (value * 9.0 / 5.0) + 32.0,
            6 => (value - 32.0) * 5.0 / 9.0,
            _ => {
                println!("Invalid option.");
                continue;
            }
        };

        println!("Converted value: {:.2}", result);
    }
}
