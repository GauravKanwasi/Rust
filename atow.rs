use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let result: Result<String, _> = input
        .split_whitespace()
        .map(|token| {
            token
                .parse::<u8>()
                .map(|code| code as char)
                .map_err(|_| token)
        })
        .collect();

    match result {
        Ok(text) => println!("{}", text),
        Err(bad_token) => eprintln!("Invalid ASCII code: {}", bad_token),
    }
}
