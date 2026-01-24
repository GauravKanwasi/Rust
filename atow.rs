use std::io::{self, Read};

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut result = String::new();

    for token in input.split_whitespace() {
        match token.parse::<u8>() {
            Ok(code) => result.push(code as char),
            Err(_) => {
                eprintln!("Invalid ASCII code: {}", token);
                return;
            }
        }
    }

    println!("{}", result);
}
