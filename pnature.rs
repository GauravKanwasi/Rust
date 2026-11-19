use std::io::{self, Write};

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const PERFECT: [u64; 8] = [
    6,
    28,
    496,
    8128,
    33_550_336,
    8_589_869_056,
    137_438_691_328,
    2_305_843_008_139_952_128,
];

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut i = 3;
    while i <= n / i {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn yes_no(b: bool) -> String {
    if b { "Yes" } else { "No" }.to_string()
}

fn nature(negative: bool, zero: bool) -> String {
    if zero {
        "Zero"
    } else if negative {
        "Negative"
    } else {
        "Positive"
    }
    .to_string()
}

fn classify_integer(n: i128) -> Vec<(&'static str, String)> {
    let mut rows = vec![
        ("Nature", nature(n < 0, n == 0)),
        ("Type", "Integer".to_string()),
    ];

    let Ok(abs) = u64::try_from(n.unsigned_abs()) else {
        rows.push(("Note", "Too large for detailed analysis".to_string()));
        return rows;
    };

    let text = abs.to_string();
    let digits: Vec<u32> = text.chars().filter_map(|c| c.to_digit(10)).collect();
    let len = digits.len() as u32;
    let armstrong: u128 = digits.iter().map(|&d| (d as u128).pow(len)).sum();
    let root = abs.isqrt();

    rows.push(("Parity", if abs % 2 == 0 { "Even" } else { "Odd" }.to_string()));
    rows.push(("Prime", yes_no(is_prime(abs))));
    rows.push(("Perfect Number", yes_no(PERFECT.contains(&abs))));
    rows.push(("Perfect Square", yes_no(root * root == abs)));
    rows.push(("Armstrong", yes_no(armstrong == abs as u128)));
    rows.push(("Palindrome", yes_no(text.chars().eq(text.chars().rev()))));
    rows.push(("Digit Sum", digits.iter().sum::<u32>().to_string()));
    rows.push(("Digits", len.to_string()));
    rows.push(("Binary", format!("{abs:b}")));
    rows
}

fn classify_decimal(n: f64) -> Vec<(&'static str, String)> {
    vec![
        ("Nature", nature(n < 0.0, n == 0.0)),
        ("Type", "Decimal".to_string()),
        ("Fractional Part", format!("{:.6}", n.fract().abs())),
        ("Floor", n.floor().to_string()),
        ("Ceil", n.ceil().to_string()),
        ("Rounded", n.round().to_string()),
    ]
}

fn render(rows: &[(&str, String)]) {
    let label_w = rows.iter().map(|(l, _)| l.len()).max().unwrap_or(0);
    println!("\n=== NUMBER CLASSIFICATION REPORT ===");
    for (label, value) in rows {
        println!("{label:<label_w$} : {value}");
    }
    println!();
}

fn main() {
    loop {
        print!("Enter a number (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            println!();
            break;
        }

        let input = line.trim().replace('_', "");
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        if let Ok(n) = input.parse::<i128>() {
            render(&classify_integer(n));
        } else if let Ok(n) = input.parse::<f64>() {
            if n.is_finite() {
                render(&classify_decimal(n));
            } else {
                eprintln!("{RED}NaN and infinity can't be classified.{RESET}\n");
            }
        } else {
            eprintln!("{RED}Invalid input. Please enter a valid number.{RESET}\n");
        }
    }
}
