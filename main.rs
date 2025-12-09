use std::io;

fn main() {
    // 1. Prompt the user for input
    println!("Hello! Please tell me your name:");

    // 2. Create a mutable string to store the input
    let mut user_name = String::new();

    // 3. Read the line from standard input (stdin)
    // The &mut user_name passes a mutable reference to the string.
    // .expect() is used for basic error handling.
    io::stdin()
        .read_line(&mut user_name)
        .expect("Failed to read line");

    // 4. Trim whitespace (including the newline character '\n' at the end)
    let trimmed_name = user_name.trim();

    // 5. Print the personalized greeting
    println!("It's great to meet you, {}!", trimmed_name);
}
