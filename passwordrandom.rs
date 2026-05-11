use std::io::{self, Write};

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?";

fn lcg_rand(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *seed
}

fn get_seed() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(12345678901234567890)
}

fn generate_password(length: usize, use_upper: bool, use_digits: bool, use_symbols: bool) -> String {
    let mut charset: Vec<u8> = LOWERCASE.to_vec();
    let mut guaranteed: Vec<u8> = vec![LOWERCASE[0]];

    if use_upper {
        charset.extend_from_slice(UPPERCASE);
        guaranteed.push(UPPERCASE[0]);
    }
    if use_digits {
        charset.extend_from_slice(DIGITS);
        guaranteed.push(DIGITS[0]);
    }
    if use_symbols {
        charset.extend_from_slice(SYMBOLS);
        guaranteed.push(SYMBOLS[0]);
    }

    let mut seed = get_seed();
    let mut password: Vec<u8> = Vec::with_capacity(length);

    for g in &guaranteed {
        let idx = (lcg_rand(&mut seed) as usize) % charset.len();
        let _ = g;
        password.push(charset[idx]);
    }

    while password.len() < length {
        let idx = (lcg_rand(&mut seed) as usize) % charset.len();
        password.push(charset[idx]);
    }

    for i in (1..password.len()).rev() {
        let j = (lcg_rand(&mut seed) as usize) % (i + 1);
        password.swap(i, j);
    }

    for (i, slot) in guaranteed.iter().enumerate() {
        let _ = slot;
        let pool: &[u8] = match i {
            0 => LOWERCASE,
            1 if use_upper => UPPERCASE,
            _ if use_digits && (!use_upper || i >= 2) => DIGITS,
            _ => SYMBOLS,
        };
        let idx = (lcg_rand(&mut seed) as usize) % pool.len();
        let pos = (lcg_rand(&mut seed) as usize) % password.len();
        password[pos] = pool[idx];
    }

    String::from_utf8(password).unwrap()
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn ask_yes_no(msg: &str) -> bool {
    loop {
        let ans = prompt(msg).to_lowercase();
        match ans.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter y or n."),
        }
    }
}

fn main() {
    println!("╔══════════════════════════════════╗");
    println!("║      🔐 Password Generator        ║");
    println!("╚══════════════════════════════════╝\n");

    let length: usize = loop {
        let input = prompt("Password length (8–128): ");
        match input.parse::<usize>() {
            Ok(n) if (8..=128).contains(&n) => break n,
            _ => println!("Enter a number between 8 and 128."),
        }
    };

    let use_upper = ask_yes_no("Include uppercase letters? (y/n): ");
    let use_digits = ask_yes_no("Include digits? (y/n): ");
    let use_symbols = ask_yes_no("Include symbols? (y/n): ");

    let count: usize = loop {
        let input = prompt("How many passwords to generate? (1–20): ");
        match input.parse::<usize>() {
            Ok(n) if (1..=20).contains(&n) => break n,
            _ => println!("Enter a number between 1 and 20."),
        }
    };

    println!("\n Generated Passwords:\n");
    for i in 1..=count {
        let pwd = generate_password(length, use_upper, use_digits, use_symbols);
        println!("  {}. {}", i, pwd);
    }

    println!("\n Tip: Store passwords in a secure password manager.");
}
