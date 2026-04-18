fn main() {
    let mut numbers = vec![45, 12, 89, 3, 67, 23];
    numbers.sort_unstable_by(|a, b| b.cmp(a));

    println!("Reverse sorted numbers:");
    println!("{}", numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));

    let mut names = vec![
        String::from("Alice"),
        String::from("Charlie"),
        String::from("Bob"),
        String::from("David"),
    ];

    names.sort_unstable_by(|a, b| b.cmp(a));

    println!("\nReverse sorted names:");
    println!("{}", names.join("\n"));
}
