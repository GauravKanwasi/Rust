use std::io::{self, Write};

#[derive(Debug)]
enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Unit {
    fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "c" | "celsius" => Some(Unit::Celsius),
            "f" | "fahrenheit" => Some(Unit::Fahrenheit),
            "k" | "kelvin" => Some(Unit::Kelvin),
            _ => None,
        }
    }

    fn to_celsius(&self, value: f64) -> f64 {
        match self {
            Unit::Celsius => value,
            Unit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
            Unit::Kelvin => value - 273.15,
        }
    }

    fn symbol(&self) -> &str {
        match self {
            Unit::Celsius => "°C",
            Unit::Fahrenheit => "°F",
            Unit::Kelvin => "K",
        }
    }
}

struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn new(value: f64, unit: &Unit) -> Result<Self, String> {
        let celsius = unit.to_celsius(value);
        if celsius < -273.15 {
            return Err("Temperature below absolute zero is physically impossible.".to_string());
        }
        Ok(Self { celsius })
    }

    fn to_fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    fn to_kelvin(&self) -> f64 {
        self.celsius + 273.15
    }

    fn condition(&self) -> &str {
        match self.celsius {
            t if t < -30.0 => "☠️  Deadly cold — life-threatening exposure risk",
            t if t < 0.0   => "🥶 Freezing — ice and frost likely",
            t if t < 10.0  => "🧊 Very cold — heavy clothing required",
            t if t < 20.0  => "🌬️  Cool — light jacket recommended",
            t if t < 25.0  => "😊 Comfortable — pleasant weather",
            t if t < 30.0  => "🌤️  Warm — light clothing ideal",
            t if t < 38.0  => "🌡️  Hot — stay hydrated",
            t if t < 45.0  => "🔥 Very hot — avoid prolonged sun exposure",
            _              => "☀️  Extreme heat — dangerous, seek shelter",
        }
    }

    fn humidity_advice(&self) -> &str {
        match self.celsius {
            t if t < 0.0  => "Watch for black ice on roads and surfaces.",
            t if t < 15.0 => "Low humidity typical; skin may feel dry.",
            t if t < 30.0 => "Moderate conditions — generally comfortable.",
            _             => "High heat index likely; limit outdoor activity.",
        }
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    buf.trim().to_string()
}

fn read_temperature() -> Option<(f64, Unit)> {
    let unit_input = prompt("Enter unit (C / F / K): ");
    let unit = Unit::from_str(&unit_input).or_else(|| {
        println!("❌ Unknown unit '{}'. Defaulting to Celsius.", unit_input);
        Some(Unit::Celsius)
    })?;

    let value_input = prompt(&format!("Enter temperature in {}: ", unit.symbol()));
    let value: f64 = match value_input.parse() {
        Ok(v) => v,
        Err(_) => {
            println!("❌ Invalid number: '{}'", value_input);
            return None;
        }
    };

    Some((value, unit))
}

fn main() {
    println!("╔══════════════════════════════╗");
    println!("║   🌡️  Temperature Analyzer    ║");
    println!("╚══════════════════════════════╝\n");

    let (value, unit) = match read_temperature() {
        Some(pair) => pair,
        None => return,
    };

    let temp = match Temperature::new(value, &unit) {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Error: {}", e);
            return;
        }
    };

    println!("\n─────────────────────────────────");
    println!("  Conversions:");
    println!("    Celsius    : {:.2}°C", temp.celsius);
    println!("    Fahrenheit : {:.2}°F", temp.to_fahrenheit());
    println!("    Kelvin     : {:.2} K", temp.to_kelvin());
    println!("─────────────────────────────────");
    println!("  Condition  : {}", temp.condition());
    println!("  Advisory   : {}", temp.humidity_advice());
    println!("─────────────────────────────────\n");
}
