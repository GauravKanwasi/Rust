use std::io::{self, Write};

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const TITLE: &str = "NUMBER CLASSIFICATION REPORT";

// ---------- number theory helpers ----------

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1 % m;
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

/// Deterministic Miller-Rabin, valid for every u64.
fn is_prime(n: u64) -> bool {
    const WITNESSES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
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

/// Euclid–Euler: an even n is perfect iff n = 2^(k-1) * (2^k - 1) with 2^k - 1 prime.
/// No odd perfect number is known (none exist below 10^1500), so odd => false.
fn is_perfect(n: u64) -> bool {
    if n < 6 || n % 2 == 1 {
        return false;
    }
    let k = n.trailing_zeros() + 1;
    let m = n >> (k - 1);
    k < 64 && m == (1u64 << k) - 1 && is_prime(m)
}

fn digits_of(n: u64) -> Vec<u32> {
    n.to_string().bytes().map(|b| (b - b'0') as u32).collect()
}

fn is_armstrong(digits: &[u32], n: u64) -> bool {
    let len = digits.len() as u32;
    let sum: u128 = digits.iter().map(|&d| (d as u128).pow(len)).sum();
    sum == n as u128
}

fn is_palindrome(digits: &[u32]) -> bool {
    digits.iter().eq(digits.iter().rev())
}

fn is_perfect_square(n: u64) -> bool {
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r * r == n
}

fn yes_no(b: bool) -> String {
    if b { "Yes" } else { "No" }.to_string()
}

// ---------- classification ----------

fn classify_number(n: f64) -> Vec<(&'static str, String)> {
    let nature = if n > 0.0 {
        "Positive"
    } else if n < 0.0 {
        "Negative"
    } else {
        "Zero"
    };

    let is_integer = n.fract() == 0.0;
    let mut props = vec![
        ("Nature", nature.to_string()),
        ("Type", if is_integer { "Integer" } else { "Decimal (floating-point)" }.to_string()),
    ];

    if is_integer {
        // 2^64 as f64 is exact, so anything strictly below it fits in a u64.
        if n.abs() >= 18_446_744_073_709_551_616.0 {
            props.push(("Note", "Too large for detailed analysis".to_string()));
            return props;
        }

        let abs = n.abs() as u64;
        let digits = digits_of(abs);

        props.push(("Parity", if abs % 2 == 0 { "Even" } else { "Odd" }.to_string()));
        props.push(("Absolute", abs.to_string()));
        props.push(("Prime", yes_no(is_prime(abs))));
        props.push(("Perfect Number", yes_no(is_perfect(abs))));
        props.push(("Perfect Square", yes_no(is_perfect_square(abs))));
        props.push(("Armstrong", yes_no(is_armstrong(&digits, abs))));
        props.push(("Palindrome", yes_no(is_palindrome(&digits))));
        props.push(("Digit Sum", digits.iter().sum::<u32>().to_string()));
        props.push(("Digits", digits.len().to_string()));
        props.push(("Binary", format!("{:b}", abs)));
    } else {
        props.push(("Absolute", format!("{:.6}", n.abs())));
        props.push(("Fractional Part", format!("{:.6}", n.fract().abs())));
        props.push(("Floor", n.floor().to_string()));
        props.push(("Ceil", n.ceil().to_string()));
        props.push(("Rounded", n.round().to_string()));
    }

    props
}

// ---------- I/O ----------

/// Returns Ok(None) on EOF (e.g. Ctrl-D / closed pipe).
fn prompt(msg: &str) -> io::Result<Option<String>> {
    print!("{msg}");
    io::stdout().flush()?;
    let mut s = String::new();
    if io::stdin().read_line(&mut s)? == 0 {
        return Ok(None);
    }
    Ok(Some(s.trim().to_string()))
}

fn render(rows: &[(&str, String)]) {
    let label_w = rows.iter().map(|(l, _)| l.chars().count()).max().unwrap_or(0);
    let value_w = rows.iter().map(|(_, v)| v.chars().count()).max().unwrap_or(0);
    let inner = (label_w + 3 + value_w).max(TITLE.len()) + 2;
    let pad = inner - label_w - 5;
    let bar = "─".repeat(inner);

    println!("\n┌{bar}┐");
    println!("│{TITLE:^inner$}│");
    println!("├{bar}┤");
    for (label, value) in rows {
        println!("│ {label:<label_w$} : {value:<pad$} │");
    }
    println!("└{bar}┘\n");
}

fn main() {
    loop {
        let input = match prompt("Enter a number (or 'exit' to quit): ") {
            Ok(Some(s)) => s,
            Ok(None) => {
                println!();
                break;
            }
            Err(e) => {
                eprintln!("Error reading input: {e}");
                break;
            }
        };

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        // Allow digit separators like 1_000_000.
        let number = match input.replace('_', "").parse::<f64>() {
            Ok(n) if n.is_finite() => n,
            Ok(_) => {
                eprintln!("{RED}NaN and infinity can't be classified.{RESET}\n");
                continue;
            }
            Err(_) => {
                eprintln!("{RED}Invalid input. Please enter a valid number.{RESET}\n");
                continue;
            }
        };

        render(&classify_number(number));
    }
}
