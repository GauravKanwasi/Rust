// sorting.rs
// A simple Rust program demonstrating sorting

use std::io;

fn main() {
    // Example 1: Sorting integers
    let mut numbers = vec![42, 7, 19, 73, 3, 25];
    println!("Original numbers: {:?}", numbers);

    numbers.sort();
    println!("Sorted numbers (ascending): {:?}", numbers);

    numbers.sort_by(|a, b| b.cmp(a));
    println!("Sorted numbers (descending): {:?}", numbers);

    // Example 2: Sorting strings
    let mut names = vec![
        String::from("Gaurav"),
        String::from("Amit"),
        String::from("Neha"),
        String::from("Ravi"),
        String::from("Sonia"),
    ];

    println!("\nOriginal names: {:?}", names);

    names.sort();
    println!("Sorted names (alphabetical): {:?}", names);

    // Example 3: Sorting user input
    println!("\nEnter words separated by spaces to sort:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut words: Vec<&str> = input.split_whitespace().collect();
    words.sort();

    println!("Sorted input words: {:?}", words);
}
