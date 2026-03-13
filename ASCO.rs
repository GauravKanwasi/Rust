use std::io::{self, Read};

fn main() {
    println!("Enter a word or sentence:");

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let input = input.trim_end();

    let max_len = input.chars().map(|ch| ch.len_utf8()).max().unwrap_or(1);
    let separator = "-".repeat(30 + max_len);

    println!("\n{:<15} {:<10} {}", "Character", "Decimal", "Hex");
    println!("{}", separator);

    for ch in input.chars() {
        if ch.is_ascii() {
            println!("'{:<14} {:<10} {:#04X}", format!("{}'", ch), ch as u8, ch as u8);
        } else {
            let code_point = ch as u32;
            println!("'{:<14} {:<10} U+{:04X}  (non-ASCII)", format!("{}'", ch), code_point, code_point);
        }
    }

    let total = input.chars().count();
    let ascii_count = input.chars().filter(|c| c.is_ascii()).count();
    let non_ascii_count = total - ascii_count;

    println!("{}", separator);
    println!("Total characters : {}", total);
    println!("ASCII            : {}", ascii_count);
    println!("Non-ASCII        : {}", non_ascii_count);
}
