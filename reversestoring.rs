fn main() {
    // Example 1: Reverse sorting integers
    let mut numbers = vec![45, 12, 89, 3, 67, 23];
    numbers.sort_by(|a, b| b.cmp(a));

    println!("Reverse sorted numbers:");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();

    // Example 2: Reverse sorting strings
    let mut names = vec![
        String::from("Alice"),
        String::from("Charlie"),
        String::from("Bob"),
        String::from("David"),
    ];

    names.sort_by(|a, b| b.cmp(a));

    println!("\nReverse sorted names:");
    for name in &names {
        println!("{}", name);
    }
}
