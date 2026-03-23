fn main() {
    let text = "Hello! Welcome to the world of Rust programming.";

    println!("Original: \"{}\"\n", text);

    let words: Vec<&str> = text.split_whitespace().collect();

    for (i, word) in words.iter().enumerate() {
        println!("  [{:>2}] {}", i + 1, word);
    }

    println!("\nTotal words: {}", words.len());
}
