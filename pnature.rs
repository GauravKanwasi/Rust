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
const WITNESSES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

type Row = (&'static str, String);

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    result
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &WITNESSES {
        if n % p == 0 {
            return n == p;
        }
    }

    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;

    'witness: for &a in &WITNESSES {
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

fn yes_no(b: bool) -> String {
    (if b { "Yes" } else { "No" }).to_string()
}

fn nature(negative: bool, zero: bool) -> String {
    (if zero {
        "Zero"
    } else if negative {
        "Negative"
    } else {
        "Positive"
    })
    .to_string()
}

fn classify_integer(n: i128) -> Vec<Row> {
    let negative = n < 0;
    let mut rows: Vec<Row> = vec![
        ("Nature", nature(negative, n == 0)),
        ("Type", "Integer".to_string()),
    ];

    let Ok(abs) = u64::try_from(n.unsigned_abs()) else {
        rows.push(("Note", "Too large for detailed analysis".to_string()));
        return rows;
    };

    let text = abs.to_string();
    let digits: Vec<u32> = text.bytes().map(|b| u32::from(b - b'0')).collect();
    let len = digits.len() as u32;
    let armstrong: u128 = digits.iter().map(|&d| u128::from(d).pow(len)).sum();
    let root = abs.isqrt();
    let sign = if negative { "-" } else { "" };

    rows.push(("Parity", (if abs % 2 == 0 { "Even" } else { "Odd" }).to_string()));
    rows.push(("Prime", yes_no(!negative && is_prime(abs))));
    rows.push(("Perfect Number", yes_no(!negative && PERFECT.contains(&abs))));
    rows.push(("Perfect Square", yes_no(!negative && root * root == abs)));
    rows.push(("Armstrong", yes_no(!negative && armstrong == u128::from(abs))));
    rows.push(("Palindrome", yes_no(!negative && text.bytes().eq(text.bytes().rev()))));
    rows.push(("Digit Sum", digits.iter().sum::<u32>().to_string()));
    rows.push(("Digits", len.to_string()));
    rows.push(("Binary", format!("{sign}{abs:b}")));
    rows.push(("Hexadecimal", format!("{sign}{abs:X}")));
    rows
}

fn classify_decimal(n: f64) -> Vec<Row> {
    let clean = |x: f64| x + 0.0;
    let kind = if n.fract() == 0.0 { "Decimal (whole value)" } else { "Decimal" };

    vec![
        ("Nature", nature(n < 0.0, n == 0.0)),
        ("Type", kind.to_string()),
        ("Fractional Part", format!("{:.6}", n.fract().abs())),
        ("Floor", clean(n.floor()).to_string()),
        ("Ceil", clean(n.ceil()).to_string()),
        ("Rounded", clean(n.round()).to_string()),
        ("Truncated", clean(n.trunc()).to_string()),
    ]
}

fn render(rows: &[Row]) {
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
        if input.is_empty() {
            continue;
        }
        if ["exit", "quit", "q"].iter().any(|w| input.eq_ignore_ascii_case(w)) {
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
