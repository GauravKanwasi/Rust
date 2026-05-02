use std::io;

fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn convert(choice: u32, temp: f64) -> Option<(f64, &'static str)> {
    match choice {
        1 => Some(((temp * 9.0 / 5.0) + 32.0, "°C → °F")),
        2 => Some(((temp - 32.0) * 5.0 / 9.0, "°F → °C")),
        3 => Some((temp + 273.15, "°C → K")),
        4 => Some((temp - 273.15, "K → °C")),
        5 => Some(((temp - 32.0) * 5.0 / 9.0 + 273.15, "°F → K")),
        6 => Some(((temp - 273.15) * 9.0 / 5.0 + 32.0, "K → °F")),
        _ => None,
    }
}

fn validate_temp(choice: u32, temp: f64) -> Result<(), &'static str> {
    let kelvin = match choice {
        2 | 5 => (temp - 32.0) * 5.0 / 9.0 + 273.15,
        3 | 1 => temp + 273.15,
        4 | 6 => temp,
        _ => return Ok(()),
    };
    if kelvin < 0.0 {
        Err("Temperature is below absolute zero.")
    } else {
        Ok(())
    }
}

fn main() {
    println!("╔══════════════════════════╗");
    println!("║   Temperature Converter  ║");
    println!("╠══════════════════════════╣");
    println!("║ 1. Celsius → Fahrenheit  ║");
    println!("║ 2. Fahrenheit → Celsius  ║");
    println!("║ 3. Celsius → Kelvin      ║");
    println!("║ 4. Kelvin → Celsius      ║");
    println!("║ 5. Fahrenheit → Kelvin   ║");
    println!("║ 6. Kelvin → Fahrenheit   ║");
    println!("╚══════════════════════════╝");

    let choice: u32 = match read_line("Enter your choice (1-6):").parse() {
        Ok(n) => n,
        Err(_) => { println!("Invalid choice."); return; }
    };

    let temp: f64 = match read_line("Enter temperature value:").parse() {
        Ok(t) => t,
        Err(_) => { println!("Invalid number."); return; }
    };

    if let Err(e) = validate_temp(choice, temp) {
        println!("Error: {}", e);
        return;
    }

    match convert(choice, temp) {
        Some((result, label)) => println!("\n[{}] Result: {:.2}", label, result),
        None => println!("Invalid choice."),
    }
}
