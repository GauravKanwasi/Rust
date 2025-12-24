use std::io::{self, Write};

fn count_vowels_and_consonants(text: &str) -> (usize, usize) {
    let vowels = "aeiouAEIOU";
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for ch in text.chars() {
        if ch.is_alphabetic() {
            if vowels.contains(ch) {
                vowel_count += 1;
            } else {
                consonant_count += 1;
            }
        }
    }

    (vowel_count, consonant_count)
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let paragraph_count: usize = read_input("Enter number of paragraphs: ")
        .parse()
        .expect("Please enter a valid number");

    println!("\n--- Paragraph Analysis ---\n");

    for i in 1..=paragraph_count {
        println!("Paragraph {}", i);

        let name = read_input("Enter paragraph name: ");
        let content = read_input("Enter paragraph text: ");

        let (vowels, consonants) = count_vowels_and_consonants(&content);

        println!("\nResults for '{}':", name);
        println!("Vowels     : {}", vowels);
        println!("Consonants : {}", consonants);
        println!("--------------------------------\n");
    }
}
