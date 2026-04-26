use std::io::{self, BufRead, Write};

fn process_line(line: &str) -> String {
    line.chars()
        .map(|c| match c {
            'a'..='z' => (c as u8 - 32) as char,
            _ => c,
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut lines_processed = 0;
    let mut chars_converted = 0;

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        let original_lowercase: usize = line.chars().filter(|c| c.is_lowercase()).count();
        let processed = process_line(&line);

        writeln!(out, "{}", processed).expect("Failed to write output");

        lines_processed += 1;
        chars_converted += original_lowercase;
    }

    writeln!(
        out,
        "\n[Summary] Lines processed: {} | Characters converted: {}",
        lines_processed, chars_converted
    )
    .expect("Failed to write summary");
}
