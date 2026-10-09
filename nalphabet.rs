use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

const HELP: &str = "\
letter_freq - count letter frequencies in files or stdin

USAGE:
    letter_freq [OPTIONS] [FILE...]

OPTIONS:
    -t, --top <N>        Show only the N most/first letters
    -s, --sort <MODE>    Sort by 'freq' (default) or 'alpha'
    -w, --width <N>      Maximum bar width in characters (default 30)
    -z, --zeros          Include letters that never appear
    -h, --help           Show this help

With no FILE, input is read from stdin.";

#[derive(Clone, Copy, PartialEq)]
enum SortMode {
    Freq,
    Alpha,
}

struct Config {
    top: Option<usize>,
    sort: SortMode,
    width: usize,
    zeros: bool,
    files: Vec<String>,
}

fn parse_args() -> Result<Config, String> {
    let mut cfg = Config {
        top: None,
        sort: SortMode::Freq,
        width: 30,
        zeros: false,
        files: Vec::new(),
    };

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = |name: &str| {
            args.next()
                .ok_or_else(|| format!("missing value for {name}"))
        };
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{HELP}");
                process::exit(0);
            }
            "-z" | "--zeros" => cfg.zeros = true,
            "-t" | "--top" => {
                let v = value("--top")?;
                let n: usize = v.parse().map_err(|_| format!("invalid number: {v}"))?;
                cfg.top = Some(n);
            }
            "-w" | "--width" => {
                let v = value("--width")?;
                cfg.width = v
                    .parse()
                    .ok()
                    .filter(|&n| n > 0)
                    .ok_or_else(|| format!("invalid width: {v}"))?;
            }
            "-s" | "--sort" => {
                cfg.sort = match value("--sort")?.as_str() {
                    "freq" => SortMode::Freq,
                    "alpha" => SortMode::Alpha,
                    other => return Err(format!("unknown sort mode: {other}")),
                };
            }
            s if s.starts_with('-') && s.len() > 1 => {
                return Err(format!("unknown option: {s}"));
            }
            _ => cfg.files.push(arg),
        }
    }
    Ok(cfg)
}

fn count_letters(data: &[u8], counts: &mut [usize; 26]) {
    for b in data.iter().filter(|b| b.is_ascii_alphabetic()) {
        counts[(b.to_ascii_lowercase() - b'a') as usize] += 1;
    }
}

fn read_input(cfg: &Config) -> io::Result<[usize; 26]> {
    let mut counts = [0usize; 26];
    if cfg.files.is_empty() {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        count_letters(&buf, &mut counts);
    } else {
        for path in &cfg.files {
            let data = fs::read(path)
                .map_err(|e| io::Error::new(e.kind(), format!("{path}: {e}")))?;
            count_letters(&data, &mut counts);
        }
    }
    Ok(counts)
}

fn bar(count: usize, max: usize, width: usize) -> String {
    const PARTIAL: [&str; 8] = ["", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];
    if max == 0 || count == 0 {
        return String::new();
    }
    let eighths = (count * width * 8 + max / 2) / max;
    let eighths = eighths.max(1);
    format!("{}{}", "█".repeat(eighths / 8), PARTIAL[eighths % 8])
}

fn main() {
    let cfg = match parse_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}\n\n{HELP}");
            process::exit(2);
        }
    };

    let counts = match read_input(&cfg) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    let total: usize = counts.iter().sum();
    if total == 0 {
        println!("No alphabetic characters found.");
        return;
    }

    let vowels: usize = b"aeiou".iter().map(|&v| counts[(v - b'a') as usize]).sum();
    let distinct = counts.iter().filter(|&&c| c > 0).count();

    let mut rows: Vec<(char, usize)> = counts
        .iter()
        .enumerate()
        .filter(|&(_, &c)| cfg.zeros || c > 0)
        .map(|(i, &c)| ((b'A' + i as u8) as char, c))
        .collect();

    match cfg.sort {
        SortMode::Freq => rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0))),
        SortMode::Alpha => rows.sort_by_key(|r| r.0),
    }
    if let Some(n) = cfg.top {
        rows.truncate(n);
    }

    let max = rows.iter().map(|r| r.1).max().unwrap_or(0);
    let pct = |n: usize| n as f64 * 100.0 / total as f64;

    println!("Letter frequency ({total} letters, {distinct}/26 distinct)\n");
    println!("{:>2} | {:>8} | {:>6} | {:>6} | Distribution", "", "Count", "%", "Cum.%");
    println!("{:-<3}+{:-<10}+{:-<8}+{:-<8}+{:-<14}", "", "", "", "", "");

    let mut cumulative = 0usize;
    for (ch, count) in &rows {
        cumulative += count;
        println!(
            "{ch:>2} | {count:>8} | {:>5.1}% | {:>5.1}% | {}",
            pct(*count),
            pct(cumulative),
            bar(*count, max, cfg.width)
        );
    }

    let consonants = total - vowels;
    println!(
        "\nVowels: {vowels} ({:.1}%)   Consonants: {consonants} ({:.1}%)",
        pct(vowels),
        pct(consonants)
    );
}
