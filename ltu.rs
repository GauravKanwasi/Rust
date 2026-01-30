use std::io;

fn main() {
    let mut input = String::new();

    // Read input from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Convert to uppercase
    let output = input.to_uppercase();

    // Print result
    println!("{}", output);
}
