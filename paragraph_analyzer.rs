use std::io::{self, Write};
use std::collections::HashMap;

fn count_vowels_and_consonants(text: &str) -> (usize, usize) {
    let vowels = "aeiouAEIOU";
    text.chars().filter(|c| c.is_alphabetic()).fold((0, 0), |(v, c), ch| {
        if vowels.contains(ch) { (v + 1, c) } else { (v, c + 1) }
    })
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_sentences(text: &str) -> usize {
    text.chars().filter(|&c| c == '.' || c == '!' || c == '?').count().max(1)
}

fn most_frequent_char(text: &str) -> Option<char> {
    let freq = text.chars().filter(|c| c.is_alphabetic()).fold(HashMap::new(), |mut map, c| {
        *map.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
        map
    });
    freq.into_iter().max_by_key(|&(_, count)| count).map(|(ch, _)| ch)
}

fn average_word_length(text: &str) -> f64 {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() { return 0.0; }
    let total_chars: usize = words.iter().map(|w| w.chars().filter(|c| c.is_alphabetic()).count()).sum();
    total_chars as f64 / words.len() as f64
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_separator(ch: char, count: usize) {
    println!("{}", ch.to_string().repeat(count));
}

fn main() {
    print_separator('=', 40);
    println!("     PARAGRAPH ANALYSIS TOOL");
    print_separator('=', 40);

    let paragraph_count: usize = loop {
        match read_input("\nEnter number of paragraphs: ").parse() {
            Ok(n) if n > 0 => break n,
            _ => println!("Please enter a valid positive number."),
        }
    };

    let mut total_vowels = 0;
    let mut total_consonants = 0;
    let mut total_words = 0;

    println!("\n");

    for i in 1..=paragraph_count {
        print_separator('-', 40);
        println!("PARAGRAPH {}", i);
        print_separator('-', 40);

        let name = read_input("Enter paragraph name : ");
        let content = read_input("Enter paragraph text : ");

        if content.is_empty() {
            println!("No content provided. Skipping...\n");
            continue;
        }

        let (vowels, consonants) = count_vowels_and_consonants(&content);
        let words = count_words(&content);
        let sentences = count_sentences(&content);
        let letters = vowels + consonants;
        let avg_word_len = average_word_length(&content);
        let freq_char = most_frequent_char(&content);
        let vowel_ratio = if letters > 0 { (vowels as f64 / letters as f64) * 100.0 } else { 0.0 };

        total_vowels += vowels;
        total_consonants += consonants;
        total_words += words;

        println!("\nResults for '{}' :", name);
        println!("  Vowels              : {}", vowels);
        println!("  Consonants          : {}", consonants);
        println!("  Total Letters       : {}", letters);
        println!("  Vowel Ratio         : {:.1}%", vowel_ratio);
        println!("  Words               : {}", words);
        println!("  Sentences           : {}", sentences);
        println!("  Avg Word Length     : {:.2} chars", avg_word_len);
        if let Some(ch) = freq_char {
            println!("  Most Frequent Char  : '{}'", ch);
        }
        println!();
    }

    if paragraph_count > 1 {
        print_separator('=', 40);
        println!("OVERALL SUMMARY");
        print_separator('=', 40);
        println!("  Total Vowels        : {}", total_vowels);
        println!("  Total Consonants    : {}", total_consonants);
        println!("  Total Letters       : {}", total_vowels + total_consonants);
        println!("  Total Words         : {}", total_words);
        print_separator('=', 40);
    }
}
