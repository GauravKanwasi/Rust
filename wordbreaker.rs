fn main() {
    // 1. Define the input string
    let text = "Hello! Welcome to the world of Rust programming.";

    println!("Original string: \"{}\"", text);
    println!("Breaking it down word by word:\n");

    // 2. Use .split_whitespace() to get an iterator over the words
    // This method is smart: it ignores multiple spaces and leading/trailing whitespace.
    let words = text.split_whitespace();

    // 3. Iterate through the words and print them
    for (index, word) in words.enumerate() {
        println!("Word {}: {}", index + 1, word);
    }

    // 4. Optional: If you need them in a List (Vector)
    let word_list: Vec<&str> = text.split_whitespace().collect();
    println!("\nTotal word count: {}", word_list.len());
}
