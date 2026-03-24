use std::io::{self, Write};

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn count_chars(s: &str) -> usize {
    s.chars().filter(|c| !c.is_whitespace()).count()
}

fn count_sentences(s: &str) -> usize {
    s.chars().filter(|&c| c == '.' || c == '!' || c == '?').count().max(if s.trim().is_empty() { 0 } else { 1 })
}

fn print_separator() {
    println!("{}", "─".repeat(34));
}

fn main() {
    print!("Enter text: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let text = input.trim();

    if text.is_empty() {
        eprintln!("No input provided.");
        std::process::exit(1);
    }

    let words = count_words(text);
    let chars = count_chars(text);
    let sentences = count_sentences(text);
    let lines = text.lines().count();
    let avg_word_len = if words > 0 { chars as f64 / words as f64 } else { 0.0 };

    print_separator();
    println!(" {:<20} {:>10}", "Words",     words);
    println!(" {:<20} {:>10}", "Characters", chars);
    println!(" {:<20} {:>10}", "Sentences",  sentences);
    println!(" {:<20} {:>10}", "Lines",      lines);
    println!(" {:<20} {:>10.2}", "Avg word length", avg_word_len);
    print_separator();
}
