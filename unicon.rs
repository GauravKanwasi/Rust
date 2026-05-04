use std::io;

fn main() {
    loop {
        println!("\n╔══════════════════════════╗");
        println!("║     UNIT CONVERTER       ║");
        println!("╠══════════════════════════╣");
        println!("║  1. Length               ║");
        println!("║  2. Weight               ║");
        println!("║  3. Temperature          ║");
        println!("║  4. Speed                ║");
        println!("║  5. Area                 ║");
        println!("║  6. Volume               ║");
        println!("║  0. Exit                 ║");
        println!("╚══════════════════════════╝");

        match read_number("Choose (0-6): ") {
            0 => {
                println!("Goodbye!");
                break;
            }
            1 => length_converter(),
            2 => weight_converter(),
            3 => temperature_converter(),
            4 => speed_converter(),
            5 => area_converter(),
            6 => volume_converter(),
            _ => println!("Invalid choice."),
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

fn read_number(prompt: &str) -> i32 {
    loop {
        print!("{} ", prompt);
        let input = read_line();
        match input.parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("  Invalid input. Enter a number."),
        }
    }
}

fn read_float(prompt: &str) -> f64 {
    loop {
        print!("{} ", prompt);
        let input = read_line();
        match input.parse::<f64>() {
            Ok(n) => return n,
            Err(_) => println!("  Invalid input. Enter a number."),
        }
    }
}

fn read_unit(prompt: &str, valid: &[&str]) -> String {
    loop {
        print!("{} ", prompt);
        let input = read_line();
        if valid.contains(&input.as_str()) {
            return input;
        }
        println!("  Invalid unit. Options: {}", valid.join(", "));
    }
}

fn print_result(value: f64, from_unit: &str, result: f64, to_unit: &str) {
    println!("\n  ✓ {:.6} {} = {:.6} {}\n", value, from_unit, result, to_unit);
}

fn length_converter() {
    let units = ["m", "km", "cm", "mm", "mi", "ft", "in", "yd", "nm"];
    println!("\n  Units: {}", units.join(", "));
    let from = read_unit("From:", &units);
    let to = read_unit("To:  ", &units);
    let value = read_float("Value:");

    let meters = match from.as_str() {
        "m"  => value,
        "km" => value * 1_000.0,
        "cm" => value / 100.0,
        "mm" => value / 1_000.0,
        "mi" => value * 1_609.344,
        "ft" => value * 0.3048,
        "in" => value * 0.0254,
        "yd" => value * 0.9144,
        "nm" => value * 1_852.0,
        _    => unreachable!(),
    };

    let result = match to.as_str() {
        "m"  => meters,
        "km" => meters / 1_000.0,
        "cm" => meters * 100.0,
        "mm" => meters * 1_000.0,
        "mi" => meters / 1_609.344,
        "ft" => meters / 0.3048,
        "in" => meters / 0.0254,
        "yd" => meters / 0.9144,
        "nm" => meters / 1_852.0,
        _    => unreachable!(),
    };

    print_result(value, &from, result, &to);
}

fn weight_converter() {
    let units = ["kg", "g", "mg", "lb", "oz", "t", "st"];
    println!("\n  Units: {}", units.join(", "));
    let from = read_unit("From:", &units);
    let to = read_unit("To:  ", &units);
    let value = read_float("Value:");

    let kg = match from.as_str() {
        "kg" => value,
        "g"  => value / 1_000.0,
        "mg" => value / 1_000_000.0,
        "lb" => value * 0.453592,
        "oz" => value * 0.0283495,
        "t"  => value * 1_000.0,
        "st" => value * 6.35029,
        _    => unreachable!(),
    };

    let result = match to.as_str() {
        "kg" => kg,
        "g"  => kg * 1_000.0,
        "mg" => kg * 1_000_000.0,
        "lb" => kg / 0.453592,
        "oz" => kg / 0.0283495,
        "t"  => kg / 1_000.0,
        "st" => kg / 6.35029,
        _    => unreachable!(),
    };

    print_result(value, &from, result, &to);
}

fn temperature_converter() {
    let units = ["c", "f", "k"];
    println!("\n  Units: C, F, K");
    let from = read_unit("From:", &units);
    let to = read_unit("To:  ", &units);
    let value = read_float("Value:");

    let celsius = match from.as_str() {
        "c" => value,
        "f" => (value - 32.0) * 5.0 / 9.0,
        "k" => value - 273.15,
        _   => unreachable!(),
    };

    let result = match to.as_str() {
        "c" => celsius,
        "f" => celsius * 9.0 / 5.0 + 32.0,
        "k" => celsius + 273.15,
        _   => unreachable!(),
    };

    print_result(value, &from.to_uppercase(), result, &to.to_uppercase());
}

fn speed_converter() {
    let units = ["mps", "kph", "mph", "fps", "kn"];
    println!("\n  Units: {}", units.join(", "));
    println!("  (mps=m/s, kph=km/h, mph=mi/h, fps=ft/s, kn=knots)");
    let from = read_unit("From:", &units);
    let to = read_unit("To:  ", &units);
    let value = read_float("Value:");

    let mps = match from.as_str() {
        "mps" => value,
        "kph" => value / 3.6,
        "mph" => value * 0.44704,
        "fps" => value * 0.3048,
        "kn"  => value * 0.514444,
        _     => unreachable!(),
    };

    let result = match to.as_str() {
        "mps" => mps,
        "kph" => mps * 3.6,
        "mph" => mps / 0.44704,
        "fps" => mps / 0.3048,
        "kn"  => mps / 0.514444,
        _     => unreachable!(),
    };

    print_result(value, &from, result, &to);
}

fn area_converter() {
    let units = ["m2", "km2", "cm2", "ft2", "in2", "ac", "ha"];
    println!("\n  Units: {}", units.join(", "));
    let from = read_unit("From:", &units);
    let to = read_unit("To:  ", &units);
    let value = read_float("Value:");

    let m2 = match from.as_str() {
        "m2"  => value,
        "km2" => value * 1_000_000.0,
        "cm2" => value / 10_000.0,
        "ft2" => value * 0.092903,
        "in2" => value * 0.00064516,
        "ac"  => value * 4_046.86,
        "ha"  => value * 10_000.0,
        _     => unreachable!(),
    };

    let result = match to.as_str() {
        "m2"  => m2,
        "km2" => m2 / 1_000_000.0,
        "cm2" => m2 * 10_000.0,
        "ft2" => m2 / 0.092903,
        "in2" => m2 / 0.00064516,
        "ac"  => m2 / 4_046.86,
        "ha"  => m2 / 10_000.0,
        _     => unreachable!(),
    };

    print_result(value, &from, result, &to);
}

fn volume_converter() {
    let units = ["l", "ml", "m3", "gal", "qt", "pt", "cup", "floz", "tbsp", "tsp"];
    println!("\n  Units: {}", units.join(", "));
    let from = read_unit("From:", &units);
    let to = read_unit("To:  ", &units);
    let value = read_float("Value:");

    let liters = match from.as_str() {
        "l"    => value,
        "ml"   => value / 1_000.0,
        "m3"   => value * 1_000.0,
        "gal"  => value * 3.78541,
        "qt"   => value * 0.946353,
        "pt"   => value * 0.473176,
        "cup"  => value * 0.236588,
        "floz" => value * 0.0295735,
        "tbsp" => value * 0.0147868,
        "tsp"  => value * 0.00492892,
        _      => unreachable!(),
    };

    let result = match to.as_str() {
        "l"    => liters,
        "ml"   => liters * 1_000.0,
        "m3"   => liters / 1_000.0,
        "gal"  => liters / 3.78541,
        "qt"   => liters / 0.946353,
        "pt"   => liters / 0.473176,
        "cup"  => liters / 0.236588,
        "floz" => liters / 0.0295735,
        "tbsp" => liters / 0.0147868,
        "tsp"  => liters / 0.00492892,
        _      => unreachable!(),
    };

    print_result(value, &from, result, &to);
}
