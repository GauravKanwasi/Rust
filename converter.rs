use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
enum Conversion {
    MToKm,
    KmToM,
    KgToG,
    GToKg,
    CToF,
    FToC,
    MiToKm,
    KmToMi,
    LbToKg,
    KgToLb,
    Exit,
}

impl Conversion {
    fn label(&self) -> &str {
        match self {
            Conversion::MToKm  => "Meters → Kilometers",
            Conversion::KmToM  => "Kilometers → Meters",
            Conversion::KgToG  => "Kilograms → Grams",
            Conversion::GToKg  => "Grams → Kilograms",
            Conversion::CToF   => "Celsius → Fahrenheit",
            Conversion::FToC   => "Fahrenheit → Celsius",
            Conversion::MiToKm => "Miles → Kilometers",
            Conversion::KmToMi => "Kilometers → Miles",
            Conversion::LbToKg => "Pounds → Kilograms",
            Conversion::KgToLb => "Kilograms → Pounds",
            Conversion::Exit   => "Exit",
        }
    }

    fn unit_in(&self) -> &str {
        match self {
            Conversion::MToKm  => "m",
            Conversion::KmToM  => "km",
            Conversion::KgToG  => "kg",
            Conversion::GToKg  => "g",
            Conversion::CToF   => "°C",
            Conversion::FToC   => "°F",
            Conversion::MiToKm => "mi",
            Conversion::KmToMi => "km",
            Conversion::LbToKg => "lb",
            Conversion::KgToLb => "kg",
            Conversion::Exit   => "",
        }
    }

    fn unit_out(&self) -> &str {
        match self {
            Conversion::MToKm  => "km",
            Conversion::KmToM  => "m",
            Conversion::KgToG  => "g",
            Conversion::GToKg  => "kg",
            Conversion::CToF   => "°F",
            Conversion::FToC   => "°C",
            Conversion::MiToKm => "km",
            Conversion::KmToMi => "mi",
            Conversion::LbToKg => "kg",
            Conversion::KgToLb => "lb",
            Conversion::Exit   => "",
        }
    }

    fn convert(&self, v: f64) -> f64 {
        match self {
            Conversion::MToKm  => v * 0.001,
            Conversion::KmToM  => v * 1_000.0,
            Conversion::KgToG  => v * 1_000.0,
            Conversion::GToKg  => v * 0.001,
            Conversion::CToF   => v * 9.0 / 5.0 + 32.0,
            Conversion::FToC   => (v - 32.0) * 5.0 / 9.0,
            Conversion::MiToKm => v * 1.609_344,
            Conversion::KmToMi => v / 1.609_344,
            Conversion::LbToKg => v * 0.453_592,
            Conversion::KgToLb => v / 0.453_592,
            Conversion::Exit   => v,
        }
    }

    fn from_index(n: u8) -> Option<Self> {
        match n {
            1  => Some(Conversion::MToKm),
            2  => Some(Conversion::KmToM),
            3  => Some(Conversion::KgToG),
            4  => Some(Conversion::GToKg),
            5  => Some(Conversion::CToF),
            6  => Some(Conversion::FToC),
            7  => Some(Conversion::MiToKm),
            8  => Some(Conversion::KmToMi),
            9  => Some(Conversion::LbToKg),
            10 => Some(Conversion::KgToLb),
            11 => Some(Conversion::Exit),
            _  => None,
        }
    }
}

impl fmt::Display for Conversion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

const MENU_ITEMS: &[Conversion] = &[
    Conversion::MToKm,
    Conversion::KmToM,
    Conversion::KgToG,
    Conversion::GToKg,
    Conversion::CToF,
    Conversion::FToC,
    Conversion::MiToKm,
    Conversion::KmToMi,
    Conversion::LbToKg,
    Conversion::KgToLb,
    Conversion::Exit,
];

fn read_line() -> Option<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    Some(buf.trim().to_string())
}

fn prompt(msg: &str) -> Option<String> {
    print!("{}", msg);
    io::stdout().flush().ok()?;
    read_line()
}

fn read_f64(msg: &str) -> Option<f64> {
    prompt(msg)?.parse().ok()
}

fn read_choice() -> Option<Conversion> {
    Conversion::from_index(prompt("  Select option: ")?.parse::<u8>().ok()?)
}

fn print_menu() {
    println!("\n╔══════════════════════════════════════╗");
    println!("║       🔁  Unit Converter  🔁          ║");
    println!("╠══════════════════════════════════════╣");

    let groups = [
        ("📏 Distance", &MENU_ITEMS[0..4]),
        ("🌡  Temperature", &MENU_ITEMS[4..6]),
        ("🛣  Extended", &MENU_ITEMS[6..10]),
    ];

    let mut idx = 1u8;
    for (group_name, items) in &groups {
        println!("║  {}                        ", group_name);
        for item in *items {
            println!("║   {:>2}. {:<32}", idx, item.label());
            idx += 1;
        }
        println!("║                                      ║");
    }

    println!("║  {:>2}. {:<32}║", idx, "Exit");
    println!("╚══════════════════════════════════════╝");
}

fn format_result(conv: &Conversion, input: f64, output: f64) -> String {
    format!(
        "\n  ✅  {:.6} {} = {:.6} {}",
        input,
        conv.unit_in(),
        output,
        conv.unit_out()
    )
}

fn main() {
    println!("\nWelcome to the Enhanced Unit Converter!");

    loop {
        print_menu();

        let choice = match read_choice() {
            Some(c) => c,
            None => {
                println!("  ⚠️  Invalid selection. Please enter a number from the menu.");
                continue;
            }
        };

        if let Conversion::Exit = choice {
            println!("\n  👋  Goodbye!\n");
            break;
        }

        let msg = format!("  Enter value in {}: ", choice.unit_in());
        let value = match read_f64(&msg) {
            Some(v) => v,
            None => {
                println!("  ⚠️  Invalid number. Please enter a numeric value.");
                continue;
            }
        };

        let result = choice.convert(value);
        println!("{}", format_result(&choice, value, result));
    }
}
