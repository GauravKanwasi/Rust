use std::io::{self, Read};

fn main() {
    let mut text = String::new();
    if io::stdin().read_to_string(&mut text).is_err() {
        return;
    }
    let text = text.trim();

    println!("Original: \"{}\"\n", text);

    let words: Vec<&str> = text.split_whitespace().collect();

    for (i, word) in words.iter().enumerate() {
        println!("  [{:>2}] {}", i + 1, word);
    }

    println!("\nTotal words: {}", words.len());

    let (vowels, consonants) = text
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold((0usize, 0usize), |(v, c), ch| {
            match ch.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => (v + 1, c),
                _ => (v, c + 1),
            }
        });

    println!("Vowels: {}", vowels);
    println!("Consonants: {}", consonants);
}
