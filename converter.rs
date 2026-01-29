use std::io::{self, Write};

fn read_f64() -> Option<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<f64>().ok()
}

fn read_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse::<u32>().ok()
}

fn main() {
    loop {
        println!("\n=== Fast Unit Converter ===");
        println!("1. Meters → Kilometers");
        println!("2. Kilometers → Meters");
        println!("3. Kilograms → Grams");
        println!("4. Grams → Kilograms");
        println!("5. Celsius → Fahrenheit");
        println!("6. Fahrenheit → Celsius");
        println!("7. Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        let choice = match read_u32() {
            Some(n) => n,
            None => {
                println!("Invalid choice.");
                continue;
            }
        };

        if choice == 7 {
            println!("Goodbye.");
            break;
        }

        print!("Enter value: ");
        io::stdout().flush().unwrap();

        let value = match read_f64() {
            Some(v) => v,
            None => {
                println!("Invalid number.");
                continue;
            }
        };

        let result = match choice {
            1 => value * 0.001,
            2 => value * 1000.0,
            3 => value * 1000.0,
            4 => value * 0.001,
            5 => value * 9.0 / 5.0 + 32.0,
            6 => (value - 32.0) * 5.0 / 9.0,
            _ => {
                println!("Invalid option.");
                continue;
            }
        };

        println!("Result: {:.3}", result);
    }
}
