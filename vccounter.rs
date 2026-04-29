use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let (vowels, consonants) = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold((0usize, 0usize), |(v, c), ch| {
            match ch.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => (v + 1, c),
                _ => (v, c + 1),
            }
        });

    println!("{}", input.trim());
    println!("{}", vowels);
    println!("{}", consonants);
}
