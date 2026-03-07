use std::collections::HashMap;
use std::io::{self, BufRead};

fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let reversed: Vec<char> = cleaned.iter().rev().cloned().collect();
    cleaned == reversed
}

fn count_palindromic_substrings(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut count = 0;

    for i in 0..n {
        let mut l = i as isize;
        let mut r = i as isize;
        while l >= 0 && r < n as isize && chars[l as usize] == chars[r as usize] {
            count += 1;
            l -= 1;
            r += 1;
        }

        let mut l = i as isize;
        let mut r = i as isize + 1;
        while l >= 0 && r < n as isize && chars[l as usize] == chars[r as usize] {
            count += 1;
            l -= 1;
            r += 1;
        }
    }

    count
}

fn collect_palindromic_substrings(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut found: Vec<String> = Vec::new();

    for i in 0..n {
        let mut l = i as isize;
        let mut r = i as isize;
        while l >= 0 && r < n as isize && chars[l as usize] == chars[r as usize] {
            let sub: String = chars[l as usize..=r as usize].iter().collect();
            if sub.len() > 1 {
                found.push(sub);
            }
            l -= 1;
            r += 1;
        }

        let mut l = i as isize;
        let mut r = i as isize + 1;
        while l >= 0 && r < n as isize && chars[l as usize] == chars[r as usize] {
            let sub: String = chars[l as usize..=r as usize].iter().collect();
            found.push(sub);
            l -= 1;
            r += 1;
        }
    }

    found.sort();
    found.dedup();
    found
}

fn is_closed_palindrome(s: &str) -> bool {
    if !is_palindrome(s) {
        return false;
    }
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if cleaned.len() <= 2 {
        return false;
    }
    let inner = &cleaned[1..cleaned.len() - 1];
    is_palindrome(inner) && inner != cleaned
}

fn char_frequency(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars().filter(|c| c.is_alphanumeric()) {
        *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }
    map
}

fn longest_palindromic_substring(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return String::new();
    }

    let mut best_start = 0;
    let mut best_len = 1;

    let expand = |mut l: isize, mut r: isize| -> (usize, usize) {
        while l >= 0 && r < n as isize && chars[l as usize] == chars[r as usize] {
            l -= 1;
            r += 1;
        }
        let start = (l + 1) as usize;
        let length = (r - l - 1) as usize;
        (start, length)
    };

    for i in 0..n {
        let (s1, l1) = expand(i as isize, i as isize);
        if l1 > best_len {
            best_start = s1;
            best_len = l1;
        }

        let (s2, l2) = expand(i as isize, i as isize + 1);
        if l2 > best_len {
            best_start = s2;
            best_len = l2;
        }
    }

    chars[best_start..best_start + best_len].iter().collect()
}

fn analyze(input: &str) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Input       : \"{}\"", input);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("  Is palindrome        : {}", is_palindrome(input));
    println!("  Is closed palindrome : {}", is_closed_palindrome(input));

    let count = count_palindromic_substrings(input);
    println!("  Palindromic substrings (count) : {}", count);

    let longest = longest_palindromic_substring(input);
    println!("  Longest palindromic substring  : \"{}\"", longest);

    let subs = collect_palindromic_substrings(input);
    if subs.is_empty() {
        println!("  Unique palindromic substrings  : none");
    } else {
        println!("  Unique palindromic substrings  :");
        for sub in &subs {
            println!("    - \"{}\"", sub);
        }
    }

    let freq = char_frequency(input);
    let mut freq_vec: Vec<(char, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("  Character frequency :");
    for (ch, cnt) in &freq_vec {
        println!("    '{}' → {}", ch, cnt);
    }

    println!();
}

fn main() {
    let samples = vec![
        "racecar",
        "A man a plan a canal Panama",
        "hello",
        "abacaba",
        "Was it a car or a cat I saw",
        "level",
        "rust",
        "amanaplanacanalpanama",
    ];

    println!("\n╔══════════════════════════════════════════╗");
    println!("║         P A L I N D R O M E  S U I T E  ║");
    println!("╚══════════════════════════════════════════╝\n");

    for sample in &samples {
        analyze(sample);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Interactive mode — enter strings to analyze (Ctrl+D to quit)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) if !input.trim().is_empty() => analyze(input.trim()),
            Ok(_) => {}
            Err(_) => break,
        }
    }
}
