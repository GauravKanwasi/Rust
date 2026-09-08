use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;

fn process_line(line: &str) -> String {
    line.chars().flat_map(|c| c.to_uppercase()).collect()
}

fn run<R: BufRead, W: Write>(reader: R, mut out: W) -> io::Result<()> {
    let mut lines_processed: usize = 0;
    let mut chars_converted: usize = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        let original_lowercase = line.chars().filter(|c| c.is_lowercase()).count();
        let processed = process_line(&line);

        writeln!(out, "{}", processed)?;

        lines_processed += 1;
        chars_converted += original_lowercase;
    }

    writeln!(
        out,
        "\n[Summary] Lines processed: {} | Characters converted: {}",
        lines_processed, chars_converted
    )?;

    out.flush()
}

fn main() {
    let stdout = io::stdout();
    let out = io::BufWriter::new(stdout.lock());

    let args: Vec<String> = env::args().collect();

    let result = if let Some(path) = args.get(1) {
        match File::open(path) {
            Ok(file) => run(BufReader::new(file), out),
            Err(e) => {
                eprintln!("Failed to open '{}': {}", path, e);
                process::exit(1);
            }
        }
    } else {
        let stdin = io::stdin();
        run(stdin.lock(), out)
    };

    if let Err(e) = result {
        eprintln!("Fatal I/O error: {}", e);
        process::exit(1);
    }
}
