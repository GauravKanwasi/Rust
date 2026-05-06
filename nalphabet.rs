use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut counts = [0u32; 26];

    for ch in input.chars() {
        if ch.is_ascii_alphabetic() {
            counts[(ch.to_ascii_lowercase() as u8 - b'a') as usize] += 1;
        }
    }

    println!("Alphabet repetition count:");
    counts.iter().enumerate().for_each(|(i, count)| {
        println!("{} : {}", (b'A' + i as u8) as char, count);
    });
}
