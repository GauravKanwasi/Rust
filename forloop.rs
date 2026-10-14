```rust
use std::collections::HashMap;

fn main() {
    println!("=== 1. Range Iteration ===");
    for number in 1..=5 {
        print!("{} ", number);
    }
    println!("\n");

    println!("=== 2. Enumerate a Vector ===");
    let languages = vec!["Rust", "Python", "C++", "JavaScript", "Go"];

    for (index, language) in languages.iter().enumerate() {
        println!("{}. {}", index + 1, language);
    }

    println!("\n=== 3. Reverse Iteration ===");
    for number in (1..=5).rev() {
        print!("{} ", number);
    }
    println!("\nLIFTOFF!\n");

    println!("=== 4. HashMap Sorting ===");
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    scores.insert("David", 95);
    scores.insert("Eve", 89);

    let mut sorted_scores: Vec<_> = scores.iter().collect();
    sorted_scores.sort_by(|a, b| {
        b.1.cmp(a.1).then_with(|| a.0.cmp(b.0))
    });

    for (name, score) in sorted_scores {
        println!("{:<10} {}", name, score);
    }

    println!("\n=== 5. Filter, Map, and Collect ===");
    let squares: Vec<i32> = (1..=20)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();

    println!("Squares of even numbers: {:?}", squares);

    println!("\n=== 6. Step-by-Step Iteration ===");
    for number in (0..=30).step_by(5) {
        print!("{} ", number);
    }
    println!("\n");

    println!("=== 7. Zip Two Collections ===");
    let names = ["Alice", "Bob", "Charlie"];
    let ages = [30, 25, 35];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }

    println!("\n=== 8. Flatten Nested Collections ===");
    let departments = vec![
        vec!["Alice", "Bob"],
        vec!["Charlie", "Dave"],
        vec!["Eve", "Frank"],
    ];

    for employee in departments.iter().flatten() {
        println!("Employee: {}", employee);
    }

    println!("\n=== 9. Fold and Sum ===");
    let total: i32 = (1..=100).sum();
    let product: u64 = (1..=10).product();

    println!("Sum from 1 to 100: {}", total);
    println!("Product from 1 to 10: {}", product);

    let factorial = (1..=5).fold(1u64, |acc, x| acc * x);
    println!("Factorial of 5: {}", factorial);

    println!("\n=== 10. Chunked Iteration ===");
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for chunk in data.chunks(3) {
        println!("{:?}", chunk);
    }

    println!("\n=== 11. Running Totals ===");
    let running_totals: Vec<i32> = (1..=5)
        .scan(0, |state, number| {
            *state += number;
            Some(*state)
        })
        .collect();

    println!("{:?}", running_totals);

    println!("\n=== 12. Sliding Windows ===");
    let series = [10, 20, 30, 40, 50];

    for window in series.windows(3) {
        println!("{:?} -> Sum: {}", window, window.iter().sum::<i32>());
    }

    println!("\n=== 13. Partition ===");
    let (evens, odds): (Vec<i32>, Vec<i32>) =
        (1..=10).partition(|number| number % 2 == 0);

    println!("Evens: {:?}", evens);
    println!("Odds:  {:?}", odds);

    println!("\n=== 14. Take While and Skip While ===");
    let ascending: Vec<i32> = (1..=10)
        .take_while(|&x| x < 5)
        .collect();

    let remaining: Vec<i32> = (1..=10)
        .skip_while(|&x| x < 5)
        .collect();

    println!("Before threshold: {:?}", ascending);
    println!("From threshold:   {:?}", remaining);

    println!("\n=== 15. Peekable Iterator ===");
    let mut iter = (1..=10).peekable();

    while let Some(&next) = iter.peek() {
        if next % 2 == 0 {
            iter.next();
        } else {
            println!("Odd number: {}", iter.next().unwrap());
        }
    }

    println!("\n=== 16. Chain Iterators ===");
    let first = [1, 2, 3];
    let second = [4, 5, 6];

    for number in first.iter().chain(second.iter()) {
        print!("{} ", number);
    }
    println!("\n");

    println!("=== 17. Deduplicate Sorted Values ===");
    let mut values = vec![5, 2, 3, 2, 5, 1, 3, 4];
    values.sort_unstable();
    values.dedup();

    println!("{:?}", values);

    println!("\n=== 18. Find and Any ===");
    let numbers = [3, 7, 12, 18, 21, 30];

    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    let has_large_number = numbers.iter().any(|&x| x > 25);

    println!("First even number: {:?}", first_even);
    println!("Contains number greater than 25: {}", has_large_number);

    println!("\n=== 19. Min, Max, and Statistics ===");
    let values = [12, 45, 7, 89, 34, 21];

    let minimum = values.iter().min().unwrap();
    let maximum = values.iter().max().unwrap();
    let average = values.iter().sum::<i32>() as f64 / values.len() as f64;

    println!("Minimum: {}", minimum);
    println!("Maximum: {}", maximum);
    println!("Average: {:.2}", average);

    println!("\n=== 20. Fibonacci Iterator ===");
    for number in Fibonacci::new().take(10) {
        print!("{} ", number);
    }
    println!();

    println!("\n=== 21. Iterator with Result ===");
    let inputs = ["10", "20", "30", "invalid", "50"];

    let parsed: Vec<Result<i32, _>> = inputs
        .iter()
        .map(|value| value.parse::<i32>())
        .collect();

    for result in parsed {
        match result {
            Ok(number) => println!("Parsed: {}", number),
            Err(error) => println!("Error: {}", error),
        }
    }

    println!("\n=== 22. Custom Iterator Pipeline ===");
    let result: i32 = (1..=100)
        .filter(|x| x % 3 == 0)
        .map(|x| x * x)
        .take(10)
        .sum();

    println!("Sum of the first 10 squared multiples of 3: {}", result);
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
