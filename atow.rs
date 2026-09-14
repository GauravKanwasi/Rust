use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read input: {}", e);
        process::exit(1);
    }

    let mut text = String::new();
    let mut errors: Vec<(usize, &str)> = Vec::new();

    for (index, token) in input.split_whitespace().enumerate() {
        match token.parse::<u16>() {
            Ok(code) if code <= 127 => text.push(code as u8 as char),
            _ => errors.push((index + 1, token)),
        }
    }

    if errors.is_empty() {
        println!("{}", text);
    } else {
        for (pos, token) in &errors {
            eprintln!("Invalid ASCII code at position {}: {}", pos, token);
        }
        process::exit(1);
    }
}
