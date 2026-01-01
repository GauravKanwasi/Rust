use std::io::{self, Read};

fn main() {
    // Read entire input from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    // Array to store counts for letters A-Z
    let mut counts = [0u32; 26];

    // Count alphabet occurrences (case-insensitive)
    for ch in input.chars() {
        if ch.is_ascii_alphabetic() {
            let index = (ch.to_ascii_lowercase() as u8 - b'a') as usize;
            counts[index] += 1;
        }
    }

    // Display results
    println!("Alphabet repetition count:");
    for (i, count) in counts.iter().enumerate() {
        let letter = (b'A' + i as u8) as char;
        println!("{} : {}", letter, count);
    }
}
