use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Instant;

/// Holds every computed metric for a single paragraph.
struct Stats {
    name: String,
    vowels: usize,
    consonants: usize,
    digits: usize,
    punctuation: usize,
    uppercase: usize,
    words: usize,
    unique_words: usize,
    sentences: usize,
    longest_word: String,
    shortest_word: String,
    avg_word_len: f64,
    freq_char: Option<(char, usize)>,
    top_words: Vec<(String, usize)>,
    reading_time_secs: f64,
    is_palindrome_ignoring_spaces: bool,
}

const VOWELS: &str = "aeiouAEIOU";
const WORDS_PER_MINUTE: f64 = 200.0;

fn count_char_classes(text: &str) -> (usize, usize, usize, usize, usize) {
    // (vowels, consonants, digits, punctuation, uppercase)
    text.chars().fold((0, 0, 0, 0, 0), |(v, c, d, p, u), ch| {
        let is_upper = ch.is_uppercase();
        if ch.is_alphabetic() {
            if VOWELS.contains(ch) {
                (v + 1, c, d, p, u + is_upper as usize)
            } else {
                (v, c + 1, d, p, u + is_upper as usize)
            }
        } else if ch.is_ascii_digit() {
            (v, c, d + 1, p, u)
        } else if ch.is_ascii_punctuation() {
            (v, c, d, p + 1, u)
        } else {
            (v, c, d, p, u)
        }
    })
}

fn count_sentences(text: &str) -> usize {
    text.chars()
        .filter(|&c| c == '.' || c == '!' || c == '?')
        .count()
        .max(1)
}

fn most_frequent_char(text: &str) -> Option<(char, usize)> {
    let freq = text
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
            map
        });
    freq.into_iter().max_by_key(|&(_, count)| count)
}

fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    let mut freq: HashMap<String, usize> = HashMap::new();
    for w in text.split_whitespace() {
        let cleaned: String = w
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        if !cleaned.is_empty() {
            *freq.entry(cleaned).or_insert(0) += 1;
        }
    }
    let mut pairs: Vec<(String, usize)> = freq.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    pairs.truncate(n);
    pairs
}

fn longest_and_shortest_word(text: &str) -> (String, String) {
    let mut words: Vec<&str> = text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| c.is_ascii_punctuation()))
        .filter(|w| !w.is_empty())
        .collect();
    if words.is_empty() {
        return (String::new(), String::new());
    }
    words.sort_by_key(|w| w.len());
    (
        words.last().unwrap().to_string(),
        words.first().unwrap().to_string(),
    )
}

fn average_word_length(text: &str) -> f64 {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return 0.0;
    }
    let total_chars: usize = words
        .iter()
        .map(|w| w.chars().filter(|c| c.is_alphabetic()).count())
        .sum();
    total_chars as f64 / words.len() as f64
}

fn is_palindrome_ignoring_spaces(text: &str) -> bool {
    let cleaned: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if cleaned.len() < 2 {
        return false;
    }
    let reversed: Vec<char> = cleaned.iter().rev().cloned().collect();
    cleaned == reversed
}

fn analyze(name: String, content: &str) -> Stats {
    let (vowels, consonants, digits, punctuation, uppercase) = count_char_classes(content);
    let words = content.split_whitespace().count();
    let unique_words: usize = content
        .split_whitespace()
        .map(|w| {
            w.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase()
        })
        .filter(|w| !w.is_empty())
        .collect::<std::collections::HashSet<_>>()
        .len();
    let sentences = count_sentences(content);
    let (longest_word, shortest_word) = longest_and_shortest_word(content);
    let avg_word_len = average_word_length(content);
    let freq_char = most_frequent_char(content);
    let top = top_words(content, 3);
    let reading_time_secs = (words as f64 / WORDS_PER_MINUTE) * 60.0;
    let palindrome = is_palindrome_ignoring_spaces(content);

    Stats {
        name,
        vowels,
        consonants,
        digits,
        punctuation,
        uppercase,
        words,
        unique_words,
        sentences,
        longest_word,
        shortest_word,
        avg_word_len,
        freq_char,
        top_words: top,
        reading_time_secs,
        is_palindrome_ignoring_spaces: palindrome,
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_positive_usize(prompt: &str) -> usize {
    loop {
        match read_input(prompt).parse::<usize>() {
            Ok(n) if n > 0 => return n,
            _ => println!("  Please enter a valid positive whole number."),
        }
    }
}

fn print_separator(ch: char, count: usize) {
    println!("{}", ch.to_string().repeat(count));
}

fn print_header(title: &str) {
    print_separator('=', 46);
    println!("{:^46}", title);
    print_separator('=', 46);
}

fn print_stats(s: &Stats) {
    let letters = s.vowels + s.consonants;
    let vowel_ratio = if letters > 0 {
        (s.vowels as f64 / letters as f64) * 100.0
    } else {
        0.0
    };

    println!("\nResults for '{}':", s.name);
    println!("  Vowels              : {}", s.vowels);
    println!("  Consonants          : {}", s.consonants);
    println!("  Digits              : {}", s.digits);
    println!("  Punctuation marks   : {}", s.punctuation);
    println!("  Uppercase letters   : {}", s.uppercase);
    println!("  Total Letters       : {}", letters);
    println!("  Vowel Ratio         : {:.1}%", vowel_ratio);
    println!("  Words (total)       : {}", s.words);
    println!("  Words (unique)      : {}", s.unique_words);
    println!("  Sentences           : {}", s.sentences);
    println!("  Avg Word Length     : {:.2} chars", s.avg_word_len);
    if !s.longest_word.is_empty() {
        println!("  Longest Word        : \"{}\" ({} chars)", s.longest_word, s.longest_word.len());
        println!("  Shortest Word       : \"{}\" ({} chars)", s.shortest_word, s.shortest_word.len());
    }
    if let Some((ch, count)) = s.freq_char {
        println!("  Most Frequent Char  : '{}' ({} times)", ch, count);
    }
    if !s.top_words.is_empty() {
        let formatted: Vec<String> = s
            .top_words
            .iter()
            .map(|(w, c)| format!("{} ({})", w, c))
            .collect();
        println!("  Top Words           : {}", formatted.join(", "));
    }
    println!("  Est. Reading Time   : {:.1} sec", s.reading_time_secs);
    println!("  Palindrome (clean)  : {}", if s.is_palindrome_ignoring_spaces { "yes" } else { "no" });
}

fn main() {
    print_header("PARAGRAPH ANALYSIS TOOL");

    let paragraph_count = read_positive_usize("\nEnter number of paragraphs: ");

    let mut all_stats: Vec<Stats> = Vec::with_capacity(paragraph_count);
    let start_time = Instant::now();

    println!();

    for i in 1..=paragraph_count {
        print_separator('-', 46);
        println!("PARAGRAPH {} of {}", i, paragraph_count);
        print_separator('-', 46);

        let name = {
            let n = read_input("Enter paragraph name (blank = auto): ");
            if n.is_empty() {
                format!("Paragraph {}", i)
            } else {
                n
            }
        };
        let content = read_input("Enter paragraph text : ");

        if content.is_empty() {
            println!("  No content provided. Skipping...\n");
            continue;
        }

        let stats = analyze(name, &content);
        print_stats(&stats);
        all_stats.push(stats);
        println!();
    }

    if all_stats.is_empty() {
        println!("No valid paragraphs were analyzed.");
        return;
    }

    if all_stats.len() > 1 {
        print_header("OVERALL SUMMARY");

        let total_vowels: usize = all_stats.iter().map(|s| s.vowels).sum();
        let total_consonants: usize = all_stats.iter().map(|s| s.consonants).sum();
        let total_words: usize = all_stats.iter().map(|s| s.words).sum();
        let total_sentences: usize = all_stats.iter().map(|s| s.sentences).sum();
        let total_reading_time: f64 = all_stats.iter().map(|s| s.reading_time_secs).sum();

        let longest_overall = all_stats
            .iter()
            .max_by_key(|s| s.words)
            .map(|s| s.name.clone())
            .unwrap_or_default();

        println!("  Paragraphs Analyzed : {}", all_stats.len());
        println!("  Total Vowels        : {}", total_vowels);
        println!("  Total Consonants    : {}", total_consonants);
        println!("  Total Letters       : {}", total_vowels + total_consonants);
        println!("  Total Words         : {}", total_words);
        println!("  Total Sentences     : {}", total_sentences);
        println!("  Total Reading Time  : {:.1} sec", total_reading_time);
        println!("  Longest Paragraph   : {}", longest_overall);
        print_separator('=', 46);
    }

    println!("\nAnalysis completed in {:.2?}.", start_time.elapsed());
}
