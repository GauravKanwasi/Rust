use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter text to count words: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // trim() removes the trailing newline from the Enter key
    let trimmed_input = input.trim();
    let word_count = trimmed_input.split_whitespace().count();

    println!("--------------------------");
    println!("Words found: {}", word_count);
}
