use std::io::{self, Write};

enum Conversion {
    MToKm,
    KmToM,
    KgToG,
    GToKg,
    CToF,
    FToC,
    Exit,
}

fn read_line() -> Option<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    Some(buf)
}

fn read_f64() -> Option<f64> {
    read_line()?.trim().parse().ok()
}

fn read_choice() -> Option<Conversion> {
    match read_line()?.trim().parse::<u8>().ok()? {
        1 => Some(Conversion::MToKm),
        2 => Some(Conversion::KmToM),
        3 => Some(Conversion::KgToG),
        4 => Some(Conversion::GToKg),
        5 => Some(Conversion::CToF),
        6 => Some(Conversion::FToC),
        7 => Some(Conversion::Exit),
        _ => None,
    }
}

fn convert(c: Conversion, v: f64) -> f64 {
    match c {
        Conversion::MToKm => v * 0.001,
        Conversion::KmToM => v * 1000.0,
        Conversion::KgToG => v * 1000.0,
        Conversion::GToKg => v * 0.001,
        Conversion::CToF => v * 9.0 / 5.0 + 32.0,
        Conversion::FToC => (v - 32.0) * 5.0 / 9.0,
        Conversion::Exit => v,
    }
}

fn print_menu() {
    println!("\n=== High-Performance Unit Converter ===");
    println!("1. Meters → Kilometers");
    println!("2. Kilometers → Meters");
    println!("3. Kilograms → Grams");
    println!("4. Grams → Kilograms");
    println!("5. Celsius → Fahrenheit");
    println!("6. Fahrenheit → Celsius");
    println!("7. Exit");
    print!("Select option: ");
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        print_menu();

        let choice = match read_choice() {
            Some(c) => c,
            None => {
                println!("Invalid selection.");
                continue;
            }
        };

        if let Conversion::Exit = choice {
            println!("Exiting.");
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

        let result = convert(choice, value);
        println!("Result: {:.4}", result);
    }
}
