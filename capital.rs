use std::io::{self, Read};

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    // Convert entire input to uppercase
    let output = input.to_uppercase();

    // Print the result
    print!("{}", output);
}
