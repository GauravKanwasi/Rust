use std::io::{self, Read};

fn main() {
    // Prompt the user
    println!("Enter a word or sentence:");

    // Read input from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    // Remove trailing newline characters
    let input = input.trim_end();

    // Display ASCII codes
    println!("\nCharacter -> ASCII Code");
    println!("------------------------");

    for ch in input.chars() {
        if ch.is_ascii() {
            println!("'{}' -> {}", ch, ch as u8);
        } else {
            println!("'{}' -> Not an ASCII character", ch);
        }
    }
}
