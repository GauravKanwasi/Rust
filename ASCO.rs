use std::io::{self, BufRead, Write};

struct Row {
    glyph: String,
    code: u32,
    utf8: String,
    kind: &'static str,
}

fn describe(ch: char) -> Row {
    let mut buf = [0u8; 4];
    let utf8 = ch
        .encode_utf8(&mut buf)
        .bytes()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ");

    let kind = if ch.is_control() {
        "control"
    } else if ch.is_ascii() {
        "ascii"
    } else {
        "non-ascii"
    };

    Row {
        glyph: format!("'{}'", ch.escape_debug()),
        code: ch as u32,
        utf8,
        kind,
    }
}

fn main() -> io::Result<()> {
    print!("Enter a word or sentence: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    let input = input.trim_end_matches(['\r', '\n']);

    if input.is_empty() {
        println!("Nothing to show.");
        return Ok(());
    }

    let rule = "-".repeat(66);
    println!();
    println!(
        "{:<10} {:>8}  {:<9} {:<10} {:<12} {}",
        "Char", "Decimal", "Hex", "Binary", "UTF-8", "Kind"
    );
    println!("{rule}");

    let (mut ascii, mut non_ascii, mut control) = (0usize, 0usize, 0usize);

    for ch in input.chars() {
        let row = describe(ch);
        match row.kind {
            "control" => control += 1,
            "ascii" => ascii += 1,
            _ => non_ascii += 1,
        }
        println!(
            "{:<10} {:>8}  {:<9} {:<10} {:<12} {}",
            row.glyph,
            row.code,
            format!("U+{:04X}", row.code),
            format!("{:08b}", row.code),
            row.utf8,
            row.kind
        );
    }

    println!("{rule}");
    println!("{:<16} {}", "Characters:", input.chars().count());
    println!("{:<16} {}", "Bytes (UTF-8):", input.len());
    println!("{:<16} {}", "ASCII:", ascii);
    println!("{:<16} {}", "Non-ASCII:", non_ascii);
    println!("{:<16} {}", "Control:", control);

    Ok(())
}
