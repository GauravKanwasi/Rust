use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?";

/// Which character classes to draw from. Lowercase is always included.
#[derive(Clone, Copy)]
struct CharsetOptions {
    upper: bool,
    digits: bool,
    symbols: bool,
}

impl CharsetOptions {
    /// The full pool of allowed bytes given the selected options.
    fn full_pool(&self) -> Vec<u8> {
        let mut pool = LOWERCASE.to_vec();
        if self.upper {
            pool.extend_from_slice(UPPERCASE);
        }
        if self.digits {
            pool.extend_from_slice(DIGITS);
        }
        if self.symbols {
            pool.extend_from_slice(SYMBOLS);
        }
        pool
    }

    /// One sub-pool per *selected* category, used to guarantee at least
    /// one character from each chosen class appears in the password.
    fn required_pools(&self) -> Vec<&'static [u8]> {
        let mut pools: Vec<&'static [u8]> = vec![LOWERCASE];
        if self.upper {
            pools.push(UPPERCASE);
        }
        if self.digits {
            pools.push(DIGITS);
        }
        if self.symbols {
            pools.push(SYMBOLS);
        }
        pools
    }
}

/// Generates a password using the OS-backed CSPRNG (via `rand::thread_rng`,
/// which on modern `rand` is a ChaCha-based generator seeded from the OS).
///
/// Approach:
/// 1. Place exactly one random character from each *selected* class
///    (this guarantees the classes are represented without biasing
///    their positions).
/// 2. Fill the remaining slots from the full combined pool.
/// 3. Shuffle the whole buffer with a Fisher-Yates shuffle driven by
///    the CSPRNG, so guaranteed characters aren't predictably placed
///    at the front.
fn generate_password(length: usize, opts: CharsetOptions) -> String {
    let mut rng = rand::thread_rng();
    let full_pool = opts.full_pool();
    let required = opts.required_pools();

    debug_assert!(length >= required.len(), "length must fit all required classes");

    let mut password: Vec<u8> = Vec::with_capacity(length);

    for pool in &required {
        let ch = pool[rng.gen_range(0..pool.len())];
        password.push(ch);
    }

    while password.len() < length {
        let ch = full_pool[rng.gen_range(0..full_pool.len())];
        password.push(ch);
    }

    password.shuffle(&mut rng);

    // Safety: every byte comes from ASCII charset slices above.
    String::from_utf8(password).expect("charset is pure ASCII")
}

/// Shannon entropy in bits, assuming uniform random selection from `pool_size`
/// possible characters at each of `length` positions: log2(pool_size^length).
fn entropy_bits(length: usize, pool_size: usize) -> f64 {
    (length as f64) * (pool_size as f64).log2()
}

fn strength_label(bits: f64) -> &'static str {
    match bits {
        b if b < 40.0 => "Weak",
        b if b < 60.0 => "Fair",
        b if b < 80.0 => "Strong",
        _ => "Very strong",
    }
}

fn prompt(msg: &str) -> io::Result<String> {
    print!("{}", msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_range(msg: &str, min: usize, max: usize) -> io::Result<usize> {
    loop {
        let input = prompt(msg)?;
        match input.parse::<usize>() {
            Ok(n) if (min..=max).contains(&n) => return Ok(n),
            _ => println!("Enter a number between {} and {}.", min, max),
        }
    }
}

fn ask_yes_no(msg: &str) -> io::Result<bool> {
    loop {
        let ans = prompt(msg)?.to_lowercase();
        match ans.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            "" => return Ok(true), // sensible default: Enter = yes
            _ => println!("Please enter y or n."),
        }
    }
}

fn run() -> io::Result<()> {
    println!("╔══════════════════════════════════╗");
    println!("║      🔐 Password Generator        ║");
    println!("╚══════════════════════════════════╝\n");

    // Minimum length must be able to fit lowercase + every enabled class,
    // so ask for classes before length and validate accordingly.
    let use_upper = ask_yes_no("Include uppercase letters? (Y/n): ")?;
    let use_digits = ask_yes_no("Include digits? (Y/n): ")?;
    let use_symbols = ask_yes_no("Include symbols? (Y/n): ")?;

    let opts = CharsetOptions {
        upper: use_upper,
        digits: use_digits,
        symbols: use_symbols,
    };
    let min_length = opts.required_pools().len().max(8);

    let length = prompt_range(
        &format!("Password length ({min_length}-128): "),
        min_length,
        128,
    )?;

    let count = prompt_range("How many passwords to generate? (1-20): ", 1, 20)?;

    let pool_size = opts.full_pool().len();
    let bits = entropy_bits(length, pool_size);

    println!("\nGenerated Passwords:\n");
    for i in 1..=count {
        let pwd = generate_password(length, opts);
        println!("  {}. {}", i, pwd);
    }

    println!(
        "\nEstimated entropy: {:.1} bits  ({}, alphabet size {})",
        bits,
        strength_label(bits),
        pool_size
    );
    println!("Tip: Store passwords in a secure password manager, and use a unique one per account.");

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respects_requested_length() {
        let opts = CharsetOptions { upper: true, digits: true, symbols: true };
        let pwd = generate_password(20, opts);
        assert_eq!(pwd.len(), 20);
    }

    #[test]
    fn contains_each_selected_class() {
        let opts = CharsetOptions { upper: true, digits: true, symbols: true };
        // Run a few times since generation is random.
        for _ in 0..50 {
            let pwd = generate_password(16, opts);
            assert!(pwd.bytes().any(|b| LOWERCASE.contains(&b)));
            assert!(pwd.bytes().any(|b| UPPERCASE.contains(&b)));
            assert!(pwd.bytes().any(|b| DIGITS.contains(&b)));
            assert!(pwd.bytes().any(|b| SYMBOLS.contains(&b)));
        }
    }

    #[test]
    fn lowercase_only_still_valid() {
        let opts = CharsetOptions { upper: false, digits: false, symbols: false };
        let pwd = generate_password(10, opts);
        assert!(pwd.bytes().all(|b| LOWERCASE.contains(&b)));
    }

    #[test]
    fn entropy_matches_formula() {
        // 4 lowercase-only chars: log2(26^4) ≈ 18.8 bits
        let bits = entropy_bits(4, 26);
        assert!((bits - 18.8).abs() < 0.1);
    }
}
