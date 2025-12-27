use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid integer");

    println!("The stored number is: {}", number);
}
