use std::collections::HashMap;
use std::io::{self, BufRead, Write};

struct TextStats {
    words: usize,
    chars_no_space: usize,
    chars_with_space: usize,
    sentences: usize,
    lines: usize,
    avg_word_len: f64,
    longest_word: String,
    unique_words: usize,
    most_common_word: Option<(String, usize)>,
    reading_time_secs: f64,
}

fn analyze(text: &str) -> TextStats {
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();

    let chars_no_space = text.chars().filter(|c| !c.is_whitespace()).count();
    let chars_with_space = text.chars().count();

    let sentence_count = text
        .chars()
        .filter(|&c| c == '.' || c == '!' || c == '?')
        .count()
        .max(if text.trim().is_empty() { 0 } else { 1 });

    let line_count = text.lines().count();

    let avg_word_len = if word_count > 0 {
        words.iter().map(|w| w.chars().count()).sum::<usize>() as f64 / word_count as f64
    } else {
        0.0
    };

    let longest_word = words
        .iter()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .max_by_key(|w| w.chars().count())
        .unwrap_or("")
        .to_string();

    let mut freq: HashMap<String, usize> = HashMap::new();
    for w in &words {
        let cleaned = w
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        if !cleaned.is_empty() {
            *freq.entry(cleaned).or_insert(0) += 1;
        }
    }
    let unique_words = freq.len();
    let most_common_word = freq.into_iter().max_by_key(|(_, count)| *count);

    // Average reading speed ~200 words per minute
    let reading_time_secs = (word_count as f64 / 200.0) * 60.0;

    TextStats {
        words: word_count,
        chars_no_space,
        chars_with_space,
        sentences: sentence_count,
        lines: line_count,
        avg_word_len,
        longest_word,
        unique_words,
        most_common_word,
        reading_time_secs,
    }
}

fn print_separator(width: usize) {
    println!("{}", "─".repeat(width));
}

fn format_duration(secs: f64) -> String {
    if secs < 60.0 {
        format!("{:.0} sec", secs.max(1.0))
    } else {
        format!("{:.1} min", secs / 60.0)
    }
}

fn print_report(stats: &TextStats) {
    let width = 40;
    print_separator(width);
    println!(" TEXT ANALYSIS REPORT");
    print_separator(width);
    println!(" {:<24} {:>13}", "Words", stats.words);
    println!(" {:<24} {:>13}", "Characters (no spaces)", stats.chars_no_space);
    println!(" {:<24} {:>13}", "Characters (with spaces)", stats.chars_with_space);
    println!(" {:<24} {:>13}", "Sentences", stats.sentences);
    println!(" {:<24} {:>13}", "Lines", stats.lines);
    println!(" {:<24} {:>13}", "Unique words", stats.unique_words);
    println!(" {:<24} {:>13.2}", "Avg word length", stats.avg_word_len);
    println!(" {:<24} {:>13}", "Longest word", stats.longest_word);

    if let Some((word, count)) = &stats.most_common_word {
        println!(" {:<24} {:>13}", "Most frequent word", format!("{} ({}x)", word, count));
    }

    println!(
        " {:<24} {:>13}",
        "Est. reading time",
        format_duration(stats.reading_time_secs)
    );
    print_separator(width);
}

fn read_multiline_input() -> String {
    println!("Enter text (press Enter on an empty line to finish):");
    let stdin = io::stdin();
    let mut lines = Vec::new();

    for line in stdin.lock().lines() {
        match line {
            Ok(l) if l.is_empty() => break,
            Ok(l) => lines.push(l),
            Err(_) => break,
        }
    }

    lines.join("\n")
}

fn main() {
    io::stdout().flush().expect("Failed to flush stdout");

    let input = read_multiline_input();
    let text = input.trim();

    if text.is_empty() {
        eprintln!("No input provided.");
        std::process::exit(1);
    }

    let stats = analyze(text);
    print_report(&stats);
}
