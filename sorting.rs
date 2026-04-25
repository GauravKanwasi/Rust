use std::io;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let mut numbers = vec![42, 7, 19, 73, 3, 25];
    println!("Original: {:?}", numbers);

    numbers.sort();
    println!("Ascending: {:?}", numbers);

    numbers.reverse();
    println!("Descending: {:?}", numbers);

    let mut users = vec![
        User { name: "Gaurav".to_string(), age: 30 },
        User { name: "Amit".to_string(), age: 25 },
        User { name: "Neha".to_string(), age: 28 },
    ];

    users.sort_by_key(|u| u.age);
    println!("\nUsers sorted by age: {:?}", users);

    println!("\nEnter words separated by spaces:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut words: Vec<&str> = input.split_whitespace().collect();
    words.sort_unstable();

    println!("Sorted input: {:?}", words);
}
