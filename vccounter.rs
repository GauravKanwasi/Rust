use std::io::{self, BufRead};

fn main() {
    println!("Please enter a string:");

    let stdin = io::stdin();
    // Read the line from standard input
    let mut input = String::new();
    match stdin.lock().read_line(&mut input) {
        Ok(_) => {
            // Processing happens here
        },
        Err(error) => {
            eprintln!("Error reading input: {}", error);
            return;
        }
    }

    // Trim whitespace (like the newline character '\n') from the end
    let s = input.trim();

    // Initialize counters
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    // Iterate over each character in the string
    for c in s.chars() {
        // Convert the character to lowercase for case-insensitive comparison
        let lower_c = c.to_ascii_lowercase();

        // Check if the character is an English letter
        if lower_c.is_ascii_alphabetic() {
            // Check if it's a vowel using a match statement
            match lower_c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowel_count += 1;
                }
                // If it's a letter but not a vowel, it must be a consonant
                _ => {
                    consonant_count += 1;
                }
            }
        }
        // If the character is not an alphabetic character, it is ignored
    }

    // Print the final counts
    println!("\n--- Results ---");
    println!("Input String: \"{}\"", s);
    println!("Vowels: {}", vowel_count);
    println!("Consonants: {}", consonant_count);
    println!("-----------------");
}
