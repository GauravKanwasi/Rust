use std::io::{self, Read};

fn main() {
    println!("Enter a word or sentence:");

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let input = input.trim_end();
    let separator = "-".repeat(45);

    println!("\n{:<16} {:<12} {}", "Character", "Decimal", "Hex");
    println!("{}", separator);

    for ch in input.chars() {
        let display = format!("'{}'", ch);
        if ch.is_ascii() {
            println!("{:<16} {:<12} {:#04X}", display, ch as u8, ch as u8);
        } else {
            let code_point = ch as u32;
            println!("{:<16} {:<12} U+{:04X}  (non-ASCII)", display, code_point, code_point);
        }
    }

    let total = input.chars().count();
    let ascii_count = input.chars().filter(|c| c.is_ascii()).count();
    let non_ascii_count = total - ascii_count;

    println!("{}", separator);
    println!("{:<16} {}", "Total:", total);
    println!("{:<16} {}", "ASCII:", ascii_count);
    println!("{:<16} {}", "Non-ASCII:", non_ascii_count);
}
