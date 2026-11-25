use std::fmt::Write as _;
use std::io::{self, BufRead, Write};

fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

fn divisors(factors: &[(u64, u32)]) -> Vec<u64> {
    let mut divs = vec![1u64];
    for &(p, e) in factors {
        let len = divs.len();
        let mut pow = 1u64;
        for _ in 0..e {
            pow *= p;
            for i in 0..len {
                divs.push(divs[i] * pow);
            }
        }
    }
    divs.sort_unstable();
    divs
}

fn format_factors(factors: &[(u64, u32)]) -> String {
    let mut s = String::new();
    for (i, &(p, e)) in factors.iter().enumerate() {
        if i > 0 {
            s.push_str(" × ");
        }
        if e == 1 {
            write!(s, "{p}").unwrap();
        } else {
            write!(s, "{p}^{e}").unwrap();
        }
    }
    s
}

fn analyze(num: i64) {
    let abs = num.unsigned_abs();
    let parity = if abs % 2 == 0 { "even" } else { "odd" };
    let sign = match num.signum() {
        -1 => "negative",
        0 => "zero",
        _ => "positive",
    };

    let factors = if abs > 1 { prime_factors(abs) } else { Vec::new() };
    let is_prime = factors.len() == 1 && factors[0].1 == 1;

    println!("\n{num} is {parity}  |  {sign}  |  prime: {is_prime}");

    let prefix = if num < 0 { "-" } else { "" };
    println!("binary:   {prefix}{abs:b}");
    println!("hex:      {prefix}{abs:x}");

    if abs > 1 {
        if !is_prime {
            println!("factors:  {}", format_factors(&factors));
        }
        println!("divisors: {:?}", divisors(&factors));
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("Enter an integer (or 'q' to quit): ");
        io::stdout().flush().ok();

        let line = match lines.next() {
            Some(Ok(l)) => l,
            _ => break,
        };
        let trimmed = line.trim();

        if trimmed.eq_ignore_ascii_case("q") || trimmed.eq_ignore_ascii_case("quit") {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }

        match trimmed.replace('_', "").parse::<i64>() {
            Ok(n) => analyze(n),
            Err(e) => eprintln!("error: {trimmed:?} is not a valid i64 ({e})\n"),
        }
    }
}
