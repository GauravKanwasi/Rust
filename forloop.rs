```rust
use std::collections::HashMap;
use std::fmt::Display;

fn print_section(title: &str) {
    println!("\n{}", "=".repeat(55));
    println!("{title}");
    println!("{}", "=".repeat(55));
}

fn print_items<T: Display>(items: &[T]) {
    for (index, item) in items.iter().enumerate() {
        println!("{:>3}. {}", index + 1, item);
    }
}

fn main() {
    print_section("1. Range Iteration");

    let numbers: Vec<i32> = (1..=5).collect();
    println!("{numbers:?}");

    print_section("2. Enumerate a Vector");

    let languages = ["Rust", "Python", "C++", "JavaScript", "Go"];
    print_items(&languages);

    print_section("3. Reverse Iteration");

    for number in (1..=5).rev() {
        print!("{number} ");
    }
    println!("\nLIFTOFF!");

    print_section("4. HashMap Sorting");

    let scores = HashMap::from([
        ("Alice", 95),
        ("Bob", 87),
        ("Charlie", 92),
        ("David", 95),
        ("Eve", 89),
    ]);

    let mut sorted_scores: Vec<_> = scores.iter().collect();
    sorted_scores.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    for (name, score) in sorted_scores {
        println!("{name:<10} {score}");
    }

    print_section("5. Filter, Map, and Collect");

    let squares: Vec<i32> = (1..=20)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();

    println!("{squares:?}");

    print_section("6. Step-by-Step Iteration");

    let steps: Vec<_> = (0..=30).step_by(5).collect();
    println!("{steps:?}");

    print_section("7. Zip Two Collections");

    let names = ["Alice", "Bob", "Charlie"];
    let ages = [30, 25, 35];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{name} is {age} years old");
    }

    print_section("8. Flatten Nested Collections");

    let departments = [
        ["Alice", "Bob"],
        ["Charlie", "Dave"],
        ["Eve", "Frank"],
    ];

    for employee in departments.iter().flatten() {
        println!("Employee: {employee}");
    }

    print_section("9. Fold and Sum");

    let total: u64 = (1..=100).sum();
    let product: u64 = (1..=10).product();
    let factorial: u64 = (1..=5).product();

    println!("Sum from 1 to 100: {total}");
    println!("Product from 1 to 10: {product}");
    println!("Factorial of 5: {factorial}");

    let folded_sum = (1..=100).fold(0u64, |acc, n| acc + n);
    println!("Folded sum: {folded_sum}");

    print_section("10. Chunked Iteration");

    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for chunk in data.chunks(3) {
        println!("{chunk:?}");
    }

    print_section("11. Running Totals");

    let running_totals: Vec<i32> = (1..=5)
        .scan(0, |state, n| {
            *state += n;
            Some(*state)
        })
        .collect();

    println!("{running_totals:?}");

    print_section("12. Sliding Windows");

    let series = [10, 20, 30, 40, 50];

    for window in series.windows(3) {
        let sum: i32 = window.iter().sum();
        println!("{window:?} -> Sum: {sum}");
    }

    print_section("13. Partition");

    let (evens, odds): (Vec<i32>, Vec<i32>) =
        (1..=10).partition(|n| n % 2 == 0);

    println!("Evens: {evens:?}");
    println!("Odds: {odds:?}");

    print_section("14. Take While and Skip While");

    let before: Vec<_> = (1..=10).take_while(|&n| n < 5).collect();
    let from_threshold: Vec<_> = (1..=10).skip_while(|&n| n < 5).collect();

    println!("Before threshold: {before:?}");
    println!("From threshold: {from_threshold:?}");

    print_section("15. Peekable Iterator");

    let mut iter = (1..=10).peekable();

    while let Some(&number) = iter.peek() {
        if number % 2 == 0 {
            iter.next();
        } else if let Some(number) = iter.next() {
            println!("Odd number: {number}");
        }
    }

    print_section("16. Chain Iterators");

    let first = [1, 2, 3];
    let second = [4, 5, 6];

    let combined: Vec<_> = first.iter().chain(second.iter()).copied().collect();
    println!("{combined:?}");

    print_section("17. Deduplicate Sorted Values");

    let mut values = vec![5, 2, 3, 2, 5, 1, 3, 4];
    values.sort_unstable();
    values.dedup();

    println!("{values:?}");

    print_section("18. Find and Any");

    let numbers = [3, 7, 12, 18, 21, 30];

    let first_even = numbers.iter().find(|&&n| n % 2 == 0);
    let has_large_number = numbers.iter().any(|&n| n > 25);

    println!("First even number: {first_even:?}");
    println!("Contains number greater than 25: {has_large_number}");

    print_section("19. Min, Max, and Statistics");

    let values = [12, 45, 7, 89, 34, 21];

    let minimum = values.iter().min().copied();
    let maximum = values.iter().max().copied();
    let sum: i32 = values.iter().sum();
    let average = sum as f64 / values.len() as f64;

    println!("Minimum: {minimum:?}");
    println!("Maximum: {maximum:?}");
    println!("Sum: {sum}");
    println!("Average: {average:.2}");

    print_section("20. Fibonacci Iterator");

    let fibonacci: Vec<_> = Fibonacci::new().take(10).collect();
    println!("{fibonacci:?}");

    print_section("21. Iterator with Result");

    let inputs = ["10", "20", "30", "invalid", "50"];

    let parsed: Vec<Result<i32, _>> = inputs
        .iter()
        .map(|value| value.parse::<i32>())
        .collect();

    for result in parsed {
        match result {
            Ok(number) => println!("Parsed: {number}"),
            Err(error) => println!("Error: {error}"),
        }
    }

    let valid_numbers: Result<Vec<i32>, _> = inputs
        .iter()
        .map(|value| value.parse::<i32>())
        .collect();

    println!("All values valid: {}", valid_numbers.is_ok());

    print_section("22. Custom Iterator Pipeline");

    let result: i32 = (1..=100)
        .filter(|n| n % 3 == 0)
        .map(|n| n * n)
        .take(10)
        .sum();

    println!("Sum of first 10 squared multiples of 3: {result}");

    print_section("23. Unique Values with HashSet");

    let values = [1, 2, 2, 3, 4, 4, 5];

    let unique: std::collections::HashSet<_> = values.iter().copied().collect();
    let mut unique_sorted: Vec<_> = unique.into_iter().collect();
    unique_sorted.sort_unstable();

    println!("{unique_sorted:?}");

    print_section("24. Safe Fibonacci Overflow Handling");

    let mut fibonacci = Fibonacci::new();

    for _ in 0..10 {
        match fibonacci.next() {
            Some(number) => print!("{number} "),
            None => break,
        }
    }

    println!();
}

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        let next = self.a.checked_add(self.b)?;

        self.a = self.b;
        self.b = next;

        Some(current)
    }
}
```
