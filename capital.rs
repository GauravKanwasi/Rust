use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut stdout = io::stdout().lock();
    stdout.write_all(input.to_uppercase().as_bytes()).unwrap();
}
