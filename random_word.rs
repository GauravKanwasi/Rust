use std::time::{SystemTime, UNIX_EPOCH};

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self { state: seed }
    }

    fn new_with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.state >> 33) ^ self.state) as u32
    }

    fn gen_range(&mut self, max: usize) -> usize {
        (self.next_u32() as usize) % max
    }

    fn gen_range_between(&mut self, min: usize, max: usize) -> usize {
        assert!(min < max, "min must be less than max");
        min + self.gen_range(max - min)
    }
}

#[derive(Debug, Clone, Copy)]
enum CharSet {
    Lowercase,
    Uppercase,
    Digits,
    Alphanumeric,
    Symbols,
    All,
    Custom(&'static [u8]),
}

impl CharSet {
    fn bytes(self) -> &'static [u8] {
        match self {
            CharSet::Lowercase    => b"abcdefghijklmnopqrstuvwxyz",
            CharSet::Uppercase    => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            CharSet::Digits       => b"0123456789",
            CharSet::Alphanumeric => b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            CharSet::Symbols      => b"!@#$%^&*()-_=+[]{}|;:,.<>?",
            CharSet::All          => b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:,.<>?",
            CharSet::Custom(b)    => b,
        }
    }
}

struct WordGenerator {
    rng: SimpleRng,
    charset: CharSet,
}

impl WordGenerator {
    fn new(charset: CharSet) -> Self {
        Self { rng: SimpleRng::new(), charset }
    }

    fn with_seed(seed: u64, charset: CharSet) -> Self {
        Self { rng: SimpleRng::new_with_seed(seed), charset }
    }

    fn generate(&mut self, length: usize) -> String {
        let bytes = self.charset.bytes();
        (0..length)
            .map(|_| bytes[self.rng.gen_range(bytes.len())] as char)
            .collect()
    }

    fn generate_variable(&mut self, min_len: usize, max_len: usize) -> String {
        let length = self.rng.gen_range_between(min_len, max_len + 1);
        self.generate(length)
    }

    fn generate_batch(&mut self, count: usize, length: usize) -> Vec<String> {
        (0..count).map(|_| self.generate(length)).collect()
    }

    fn generate_batch_variable(&mut self, count: usize, min_len: usize, max_len: usize) -> Vec<String> {
        (0..count).map(|_| self.generate_variable(min_len, max_len)).collect()
    }
}

fn main() {
    println!("=== Fixed Length (Lowercase) ===");
    let mut gen = WordGenerator::new(CharSet::Lowercase);
    for word in gen.generate_batch(6, 6) {
        println!("{}", word);
    }

    println!("\n=== Variable Length (Alphanumeric) ===");
    let mut gen = WordGenerator::new(CharSet::Alphanumeric);
    for word in gen.generate_batch_variable(6, 4, 10) {
        println!("{}", word);
    }

    println!("\n=== Fixed Seed / Reproducible (All chars) ===");
    let mut gen = WordGenerator::with_seed(42, CharSet::All);
    for word in gen.generate_batch(6, 8) {
        println!("{}", word);
    }

    println!("\n=== Custom Charset ===");
    let mut gen = WordGenerator::new(CharSet::Custom(b"AEIOU01"));
    for word in gen.generate_batch(6, 6) {
        println!("{}", word);
    }
}
