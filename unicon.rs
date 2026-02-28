use std::io;

fn main() {
    println!("=== Unit Converter ===");
    println!("1. Length");
    println!("2. Weight");
    println!("3. Temperature");

    let choice = read_number("Choose conversion type (1-3): ");

    match choice {
        1 => length_converter(),
        2 => weight_converter(),
        3 => temperature_converter(),
        _ => println!("Invalid choice."),
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_number(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let input = read_line();
        if let Ok(num) = input.parse::<i32>() {
            return num;
        }
        println!("Invalid input. Try again.");
    }
}

fn read_float(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let input = read_line();
        if let Ok(num) = input.parse::<f64>() {
            return num;
        }
        println!("Invalid input. Try again.");
    }
}

fn length_converter() {
    println!("Length units: m, km, mi, ft");

    println!("From unit:");
    let from = read_line();

    println!("To unit:");
    let to = read_line();

    let value = read_float("Enter value:");

    let meters = match from.as_str() {
        "m" => value,
        "km" => value * 1000.0,
        "mi" => value * 1609.34,
        "ft" => value * 0.3048,
        _ => {
            println!("Invalid unit.");
            return;
        }
    };

    let result = match to.as_str() {
        "m" => meters,
        "km" => meters / 1000.0,
        "mi" => meters / 1609.34,
        "ft" => meters / 0.3048,
        _ => {
            println!("Invalid unit.");
            return;
        }
    };

    println!("Result: {}", result);
}

fn weight_converter() {
    println!("Weight units: kg, g, lb");

    println!("From unit:");
    let from = read_line();

    println!("To unit:");
    let to = read_line();

    let value = read_float("Enter value:");

    let kg = match from.as_str() {
        "kg" => value,
        "g" => value / 1000.0,
        "lb" => value * 0.453592,
        _ => {
            println!("Invalid unit.");
            return;
        }
    };

    let result = match to.as_str() {
        "kg" => kg,
        "g" => kg * 1000.0,
        "lb" => kg / 0.453592,
        _ => {
            println!("Invalid unit.");
            return;
        }
    };

    println!("Result: {}", result);
}

fn temperature_converter() {
    println!("Temperature units: C, F, K");

    println!("From unit:");
    let from = read_line();

    println!("To unit:");
    let to = read_line();

    let value = read_float("Enter value:");

    let celsius = match from.as_str() {
        "C" => value,
        "F" => (value - 32.0) * 5.0 / 9.0,
        "K" => value - 273.15,
        _ => {
            println!("Invalid unit.");
            return;
        }
    };

    let result = match to.as_str() {
        "C" => celsius,
        "F" => (celsius * 9.0 / 5.0) + 32.0,
        "K" => celsius + 273.15,
        _ => {
            println!("Invalid unit.");
            return;
        }
    };

    println!("Result: {}", result);
}
