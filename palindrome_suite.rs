use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::time::Instant;

// ---------- ANSI colors (no external crate needed) ----------

struct Palette {
    enabled: bool,
}

impl Palette {
    fn new(enabled: bool) -> Self {
        Palette { enabled }
    }
    fn paint(&self, code: &str, s: &str) -> String {
        if self.enabled {
            format!("\x1b[{}m{}\x1b[0m", code, s)
        } else {
            s.to_string()
        }
    }
    fn bold(&self, s: &str) -> String {
        self.paint("1", s)
    }
    fn cyan(&self, s: &str) -> String {
        self.paint("36", s)
    }
    fn green(&self, s: &str) -> String {
        self.paint("32", s)
    }
    fn yellow(&self, s: &str) -> String {
        self.paint("33", s)
    }
    fn red(&self, s: &str) -> String {
        self.paint("31", s)
    }
    fn magenta(&self, s: &str) -> String {
        self.paint("35", s)
    }
}

// ---------- Core palindrome logic ----------

fn clean(s: &str) -> Vec<char> {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn is_palindrome(s: &str) -> bool {
    let cleaned = clean(s);
    let reversed: Vec<char> = cleaned.iter().rev().cloned().collect();
    cleaned == reversed
}

/// Count all palindromic substrings using expand-around-center, O(n^2).
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
    let cleaned: String = clean(s).into_iter().collect();
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

/// NEW: Longest Palindromic Subsequence (not necessarily contiguous), via O(n^2) DP.
/// e.g. LPS of "character" is "carac" (length 5), distinct from the longest substring.
fn longest_palindromic_subsequence(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return String::new();
    }

    // dp[i][j] = length of LPS within chars[i..=j]
    let mut dp = vec![vec![0usize; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }

    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            if chars[i] == chars[j] {
                dp[i][j] = if len == 2 { 2 } else { dp[i + 1][j - 1] + 2 };
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Reconstruct the subsequence from the dp table.
    let mut result = Vec::new();
    let (mut i, mut j) = (0isize, (n - 1) as isize);
    while i <= j {
        if i == j {
            result.push((chars[i as usize], 0));
            break;
        }
        if chars[i as usize] == chars[j as usize] {
            result.push((chars[i as usize], 0));
            if i != j {
                result.push((chars[j as usize], 1));
            }
            i += 1;
            j -= 1;
        } else if dp[(i + 1) as usize][j as usize] >= dp[i as usize][(j - 1) as usize] {
            i += 1;
        } else {
            j -= 1;
        }
    }

    // result currently holds left-half chars (tag 0) then right-half chars (tag 1, reversed order).
    let mut left: Vec<char> = Vec::new();
    let mut right: Vec<char> = Vec::new();
    for (c, tag) in result {
        if tag == 0 {
            left.push(c);
        } else {
            right.push(c);
        }
    }
    right.reverse();
    left.extend(right);
    left.into_iter().collect()
}

/// NEW: "palindrome density" — fraction of all possible substrings that are palindromic.
fn palindrome_density(s: &str) -> f64 {
    let n = s.chars().count();
    if n == 0 {
        return 0.0;
    }
    let total_substrings = n * (n + 1) / 2;
    count_palindromic_substrings(s) as f64 / total_substrings as f64
}

// ---------- Reporting ----------

struct Report {
    input: String,
    is_palindrome: bool,
    is_closed_palindrome: bool,
    substring_count: usize,
    density: f64,
    longest_substring: String,
    longest_subsequence: String,
    unique_substrings: Vec<String>,
    frequency: Vec<(char, usize)>,
}

fn build_report(input: &str) -> Report {
    let mut freq: Vec<(char, usize)> = char_frequency(input).into_iter().collect();
    freq.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    Report {
        input: input.to_string(),
        is_palindrome: is_palindrome(input),
        is_closed_palindrome: is_closed_palindrome(input),
        substring_count: count_palindromic_substrings(input),
        density: palindrome_density(input),
        longest_substring: longest_palindromic_substring(input),
        longest_subsequence: longest_palindromic_subsequence(input),
        unique_substrings: collect_palindromic_substrings(input),
        frequency: freq,
    }
}

fn print_report(r: &Report, p: &Palette) {
    println!("{}", p.cyan(&"━".repeat(48)));
    println!("  {} : \"{}\"", p.bold("Input"), r.input);
    println!("{}", p.cyan(&"━".repeat(48)));

    let flag = |b: bool| if b { p.green("true") } else { p.red("false") };
    println!("  Is palindrome        : {}", flag(r.is_palindrome));
    println!(
        "  Is closed palindrome : {}",
        flag(r.is_closed_palindrome)
    );
    println!(
        "  Palindromic substrings (count) : {}",
        p.yellow(&r.substring_count.to_string())
    );
    println!("  Palindrome density              : {:.2}%", r.density * 100.0);
    println!(
        "  Longest palindromic substring   : \"{}\"",
        p.magenta(&r.longest_substring)
    );
    println!(
        "  Longest palindromic subsequence : \"{}\"",
        p.magenta(&r.longest_subsequence)
    );

    if r.unique_substrings.is_empty() {
        println!("  Unique palindromic substrings   : none");
    } else {
        println!("  Unique palindromic substrings   :");
        for sub in &r.unique_substrings {
            println!("    - \"{}\"", sub);
        }
    }

    println!("  Character frequency :");
    for (ch, cnt) in &r.frequency {
        println!("    '{}' → {}", ch, cnt);
    }
    println!();
}

fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            _ => out.push(c),
        }
    }
    out
}

fn report_to_json(r: &Report) -> String {
    let subs: Vec<String> = r
        .unique_substrings
        .iter()
        .map(|s| format!("\"{}\"", escape_json(s)))
        .collect();
    let freq: Vec<String> = r
        .frequency
        .iter()
        .map(|(c, n)| format!("{{\"char\":\"{}\",\"count\":{}}}", escape_json(&c.to_string()), n))
        .collect();

    format!(
        "{{\"input\":\"{}\",\"is_palindrome\":{},\"is_closed_palindrome\":{},\"substring_count\":{},\"density\":{:.4},\"longest_substring\":\"{}\",\"longest_subsequence\":\"{}\",\"unique_substrings\":[{}],\"frequency\":[{}]}}",
        escape_json(&r.input),
        r.is_palindrome,
        r.is_closed_palindrome,
        r.substring_count,
        r.density,
        escape_json(&r.longest_substring),
        escape_json(&r.longest_subsequence),
        subs.join(","),
        freq.join(",")
    )
}

// ---------- Summary across a batch ----------

fn print_summary(reports: &[Report], p: &Palette, elapsed_ms: f64) {
    let total = reports.len();
    let palindromes = reports.iter().filter(|r| r.is_palindrome).count();
    let closed = reports.iter().filter(|r| r.is_closed_palindrome).count();
    let total_subs: usize = reports.iter().map(|r| r.substring_count).sum();
    let avg_density: f64 = if total > 0 {
        reports.iter().map(|r| r.density).sum::<f64>() / total as f64
    } else {
        0.0
    };
    let longest_overall = reports
        .iter()
        .max_by_key(|r| r.longest_substring.chars().count())
        .map(|r| r.longest_substring.clone())
        .unwrap_or_default();

    println!("{}", p.cyan(&"═".repeat(48)));
    println!("  {}", p.bold("SUMMARY"));
    println!("{}", p.cyan(&"═".repeat(48)));
    println!("  Inputs analyzed              : {}", total);
    println!("  Full palindromes             : {}", palindromes);
    println!("  Closed (nested) palindromes  : {}", closed);
    println!("  Total palindromic substrings : {}", total_subs);
    println!("  Average palindrome density   : {:.2}%", avg_density * 100.0);
    println!("  Longest substring overall    : \"{}\"", longest_overall);
    println!("  Analysis time                : {:.3} ms", elapsed_ms);
    println!();
}

// ---------- CLI ----------

struct Options {
    json: bool,
    color: bool,
    file: Option<String>,
    summary: bool,
}

fn parse_args() -> Options {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut opts = Options {
        json: false,
        color: true,
        file: None,
        summary: true,
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => opts.json = true,
            "--no-color" => opts.color = false,
            "--no-summary" => opts.summary = false,
            "--file" => {
                if i + 1 < args.len() {
                    opts.file = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("--file requires a path argument");
                }
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => {
                eprintln!("Unknown argument: {}", other);
            }
        }
        i += 1;
    }

    opts
}

fn print_help() {
    println!(
        "Palindrome Suite

USAGE:
    palindrome_suite [OPTIONS]

OPTIONS:
    --file <PATH>   Read newline-separated strings from a file instead of stdin
    --json          Emit each report as a single-line JSON object instead of text
    --no-color      Disable ANSI color output
    --no-summary    Skip the aggregate summary printed after the batch
    -h, --help      Show this help message

With no --file, built-in sample strings are analyzed first, then lines are
read interactively from stdin (Ctrl+D to quit)."
    );
}

fn main() {
    let opts = parse_args();
    let palette = Palette::new(opts.color && env::var("NO_COLOR").is_err());

    if !opts.json {
        println!(
            "\n{}",
            palette.bold("╔══════════════════════════════════════════╗")
        );
        println!("{}", palette.bold("║         P A L I N D R O M E  S U I T E  ║"));
        println!("{}\n", palette.bold("╚══════════════════════════════════════════╝"));
    }

    let samples = vec![
        "racecar".to_string(),
        "A man a plan a canal Panama".to_string(),
        "hello".to_string(),
        "abacaba".to_string(),
        "Was it a car or a cat I saw".to_string(),
        "level".to_string(),
        "rust".to_string(),
        "amanaplanacanalpanama".to_string(),
        "character".to_string(),
    ];

    let start = Instant::now();
    let mut reports: Vec<Report> = Vec::new();

    let inputs: Vec<String> = if let Some(path) = &opts.file {
        match fs::read_to_string(path) {
            Ok(contents) => contents
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect(),
            Err(e) => {
                eprintln!("Failed to read {}: {}", path, e);
                std::process::exit(1);
            }
        }
    } else {
        samples.clone()
    };

    for input in &inputs {
        let r = build_report(input);
        if opts.json {
            println!("{}", report_to_json(&r));
        } else {
            print_report(&r, &palette);
        }
        reports.push(r);
    }

    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

    if opts.summary && !opts.json && !reports.is_empty() {
        print_summary(&reports, &palette, elapsed_ms);
    }

    // Interactive mode only when reading from built-in samples (not --file, not piped JSON batch).
    if opts.file.is_none() {
        if !opts.json {
            println!("{}", palette.cyan(&"━".repeat(48)));
            println!("  Interactive mode — enter strings to analyze (Ctrl+D to quit)");
            println!("{}\n", palette.cyan(&"━".repeat(48)));
        }

        let stdin = io::stdin();
        let mut extra_reports: Vec<Report> = Vec::new();
        for line in stdin.lock().lines() {
            match line {
                Ok(input) if !input.trim().is_empty() => {
                    let r = build_report(input.trim());
                    if opts.json {
                        println!("{}", report_to_json(&r));
                        io::stdout().flush().ok();
                    } else {
                        print_report(&r, &palette);
                    }
                    extra_reports.push(r);
                }
                Ok(_) => {}
                Err(_) => break,
            }
        }

        if opts.summary && !opts.json && !extra_reports.is_empty() {
            print_summary(&extra_reports, &palette, 0.0);
        }
    }
}

// ---------- Tests ----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_detection() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn closed_palindrome_detection() {
        // "level" -> inner "eve" is itself a palindrome and != "level"
        assert!(is_closed_palindrome("level"));
        assert!(is_closed_palindrome("aba")); // inner "b" (length 1) is trivially a palindrome
        assert!(!is_closed_palindrome("aa")); // too short after stripping ends
        assert!(!is_closed_palindrome("hello")); // not a palindrome at all
    }

    #[test]
    fn longest_substring_matches_known_case() {
        assert_eq!(longest_palindromic_substring("babad").len(), 3);
        assert_eq!(longest_palindromic_substring("cbbd"), "bb");
    }

    #[test]
    fn longest_subsequence_known_case() {
        assert_eq!(longest_palindromic_subsequence("character").len(), 5); // e.g. "carac"
        assert_eq!(longest_palindromic_subsequence("abc").len(), 1);
        assert_eq!(longest_palindromic_subsequence("bbbab").len(), 4); // "bbbb"
    }

    #[test]
    fn substring_count_matches_brute_force() {
        let s = "aabaa";
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut brute = 0;
        for i in 0..n {
            for j in i..n {
                let sub: String = chars[i..=j].iter().collect();
                if is_palindrome(&sub) {
                    brute += 1;
                }
            }
        }
        assert_eq!(count_palindromic_substrings(s), brute);
    }

    #[test]
    fn density_bounds() {
        let d = palindrome_density("racecar");
        assert!(d > 0.0 && d <= 1.0);
        assert_eq!(palindrome_density(""), 0.0);
    }

    #[test]
    fn json_escaping() {
        assert_eq!(escape_json("a\"b\\c"), "a\\\"b\\\\c");
    }
}
