use std::io::{self, Write};

struct Conversion {
    label: &'static str,
    from: &'static str,
    to: &'static str,
    apply: fn(f64) -> f64,
}

const fn conv(label: &'static str, from: &'static str, to: &'static str, apply: fn(f64) -> f64) -> Conversion {
    Conversion { label, from, to, apply }
}

const MI_KM: f64 = 1.609_344;
const LB_KG: f64 = 0.453_592_37;

const GROUPS: &[(&str, &[Conversion])] = &[
    ("Length", &[
        conv("Meters → Kilometers", "m", "km", |v| v / 1_000.0),
        conv("Kilometers → Meters", "km", "m", |v| v * 1_000.0),
        conv("Miles → Kilometers", "mi", "km", |v| v * MI_KM),
        conv("Kilometers → Miles", "km", "mi", |v| v / MI_KM),
    ]),
    ("Mass", &[
        conv("Kilograms → Grams", "kg", "g", |v| v * 1_000.0),
        conv("Grams → Kilograms", "g", "kg", |v| v / 1_000.0),
        conv("Pounds → Kilograms", "lb", "kg", |v| v * LB_KG),
        conv("Kilograms → Pounds", "kg", "lb", |v| v / LB_KG),
    ]),
    ("Temperature", &[
        conv("Celsius → Fahrenheit", "°C", "°F", |v| v * 9.0 / 5.0 + 32.0),
        conv("Fahrenheit → Celsius", "°F", "°C", |v| (v - 32.0) * 5.0 / 9.0),
    ]),
];

/// Prints `msg`, reads a line, and exits cleanly on EOF (Ctrl-D / closed pipe).
fn prompt(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().ok();

    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(n) if n > 0 => buf.trim().to_string(),
        _ => {
            println!();
            std::process::exit(0);
        }
    }
}

fn print_menu() {
    println!("\n=== Unit Converter ===");
    let mut n = 1;
    for (group, items) in GROUPS {
        println!("\n {group}");
        for item in *items {
            println!("  {n:>2}. {}", item.label);
            n += 1;
        }
    }
    println!("\n  {n:>2}. Exit");
}

fn main() {
    let conversions: Vec<&Conversion> = GROUPS.iter().flat_map(|(_, c)| c.iter()).collect();
    let exit_option = conversions.len() + 1;

    loop {
        print_menu();

        let Ok(choice) = prompt("\n  Select option: ").parse::<usize>() else {
            println!("  ⚠️  Please enter a number from the menu.");
            continue;
        };
        if choice == exit_option {
            println!("\n  👋  Goodbye!");
            return;
        }
        let Some(conv) = choice.checked_sub(1).and_then(|i| conversions.get(i)) else {
            println!("  ⚠️  Option out of range.");
            continue;
        };

        let input = prompt(&format!("  Enter value in {}: ", conv.from));
        let Some(value) = input.parse::<f64>().ok().filter(|v| v.is_finite()) else {
            println!("  ⚠️  Please enter a valid number.");
            continue;
        };

        println!("\n  ✅  {input} {} = {:.4} {}", conv.from, (conv.apply)(value), conv.to);
    }
}
