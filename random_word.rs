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

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn gen_range(&mut self, max: usize) -> usize {
        (self.next_u32() as usize) % max
    }
}

fn generate_random_word(rng: &mut SimpleRng, length: usize) -> String {
    let charset = b"abcdefghijklmnopqrstuvwxyz";
    let mut word = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.gen_range(charset.len());
        word.push(charset[idx] as char);
    }

    word
}

fn main() {
    let mut rng = SimpleRng::new();
    let word_count = 10;
    let word_length = 6;

    for _ in 0..word_count {
        println!("{}", generate_random_word(&mut rng, word_length));
    }
}
