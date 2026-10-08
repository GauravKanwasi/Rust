use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut counts = [0usize; 26];
    for b in input.bytes().filter(u8::is_ascii_alphabetic) {
        counts[(b.to_ascii_lowercase() - b'a') as usize] += 1;
    }

    let total: usize = counts.iter().sum();
    if total == 0 {
        println!("No alphabetic characters found.");
        return Ok(());
    }

    let max = *counts.iter().max().unwrap();
    let mut rows: Vec<(char, usize)> = counts
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c > 0)
        .map(|(i, &c)| ((b'A' + i as u8) as char, c))
        .collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    println!("Letter frequency ({} letters total)\n", total);
    for (ch, count) in rows {
        let pct = count as f64 * 100.0 / total as f64;
        let bar = "█".repeat((count * 30).div_ceil(max));
        println!("{ch} | {count:>6} | {pct:>5.1}% | {bar}");
    }

    Ok(())
}
