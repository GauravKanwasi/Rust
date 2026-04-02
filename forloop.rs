use std::collections::HashMap;

fn main() {
    println!("### Iterating over a range (1 to 5) ###");
    for number in 1..=5 {
        println!("Current number is: {}", number);
    }

    println!("\n### Iterating over a vector ###");
    let programming_languages = vec!["Rust", "Python", "C++", "JavaScript", "Go"];
    for (index, lang) in programming_languages.iter().enumerate() {
        println!("Index {}: Language is {}", index, lang);
    }

    println!("\n### Reverse iteration (counting down) ###");
    for i in (1..=4).rev() {
        println!("Countdown: {}", i);
    }
    println!("LIFTOFF!!!");

    println!("\n### Iterating over a HashMap ###");
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    let mut sorted_scores: Vec<(&&str, &u32)> = scores.iter().collect();
    sorted_scores.sort_by(|a, b| b.1.cmp(a.1));
    for (name, score) in &sorted_scores {
        println!("{}: {}", name, score);
    }

    println!("\n### Iterator chaining and transformation ###");
    let numbers: Vec<i32> = (1..=10).collect();
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    for val in &result {
        println!("Squared even: {}", val);
    }

    println!("\n### Iterating with a step ###");
    for i in (0..=20).step_by(5) {
        println!("Step value: {}", i);
    }

    println!("\n### Zipping two iterators ###");
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }

    println!("\n### Flat map over nested collections ###");
    let departments = vec![
        vec!["Alice", "Bob"],
        vec!["Charlie", "Dave"],
        vec!["Eve", "Frank"],
    ];
    for employee in departments.iter().flat_map(|d| d.iter()) {
        println!("Employee: {}", employee);
    }

    println!("\n### Folding / accumulation ###");
    let total: i32 = (1..=100).fold(0, |acc, x| acc + x);
    println!("Sum of 1 to 100: {}", total);

    println!("\n### Chunked iteration ###");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for chunk in data.chunks(3) {
        println!("Chunk: {:?}", chunk);
    }

    println!("\n### Scanning (running totals) ###");
    let running_totals: Vec<i32> = (1..=5).scan(0, |state, x| {
        *state += x;
        Some(*state)
    }).collect();
    for total in &running_totals {
        println!("Running total: {}", total);
    }

    println!("\n### Windows (sliding view) ###");
    let series = vec![10, 20, 30, 40, 50];
    for window in series.windows(3) {
        println!("Window: {:?}", window);
    }

    println!("\n### Partition into two collections ###");
    let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=10).partition(|&x| x % 2 == 0);
    println!("Evens: {:?}", evens);
    println!("Odds:  {:?}", odds);

    println!("\n### take_while and skip_while ###");
    let ascending: Vec<i32> = (1..=10).take_while(|&x| x < 5).collect();
    let after_threshold: Vec<i32> = (1..=10).skip_while(|&x| x < 5).collect();
    println!("take_while < 5: {:?}", ascending);
    println!("skip_while < 5: {:?}", after_threshold);

    println!("\n### Peekable iterator ###");
    let mut iter = vec![1, 2, 3, 4, 5].into_iter().peekable();
    while let Some(&next) = iter.peek() {
        if next % 2 == 0 {
            iter.next();
        } else {
            println!("Odd peeked: {}", iter.next().unwrap());
        }
    }

    println!("\n### Chain two iterators ###");
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];
    for val in first.iter().chain(second.iter()) {
        println!("Chained value: {}", val);
    }

    println!("\n### Custom struct iterator ###");
    let fib = Fibonacci { a: 0, b: 1 };
    for val in fib.take(8) {
        println!("Fibonacci: {}", val);
    }
}

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}
