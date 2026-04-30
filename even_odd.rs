use std::io;

fn is_prime(n: i64) -> bool {
    let n = n.unsigned_abs();
    if n < 2 { return false; }
    (2..=((n as f64).sqrt() as u64)).all(|i| n % i != 0)
}

fn factors_of(n: i64) -> Vec<i64> {
    let n = n.unsigned_abs() as i64;
    if n <= 1 { return vec![]; }
    (1..=n).filter(|i| n % i == 0).collect()
}

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter a number (or 'q' to quit):");

        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed == "q" { break; }

        let num: i64 = match trimmed.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("error[E0308]: expected i32, found \"{}\"", trimmed);
                continue;
            }
        };

        let parity = if num % 2 == 0 { "even" } else { "odd" };
        let sign   = if num < 0 { "negative" } else { "non-negative" };
        let prime  = is_prime(num);
        let factors = factors_of(num);

        println!("\n{} is {}  |  {}  |  prime: {}", num, parity, sign, prime);
        println!("binary:   {:b}", num.unsigned_abs());
        if !factors.is_empty() {
            println!("divisors: {:?}", factors);
        }
        println!();
    }
}
