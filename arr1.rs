use std::fmt;

struct Stats {
    sum: i32,
    average: f64,
    min: i32,
    max: i32,
    median: f64,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sum: {}, Average: {:.2}, Min: {}, Max: {}, Median: {:.1}",
            self.sum, self.average, self.min, self.max, self.median
        )
    }
}

fn compute_stats(arr: &[i32]) -> Option<Stats> {
    if arr.is_empty() {
        return None;
    }

    let sum: i32 = arr.iter().sum();
    let average = sum as f64 / arr.len() as f64;
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();

    let mut sorted = arr.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
    } else {
        sorted[mid] as f64
    };

    Some(Stats { sum, average, min, max, median })
}

fn main() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Array elements: {:?}", arr);

    match (arr.first(), arr.last()) {
        (Some(first), Some(last)) => println!("First: {}, Last: {}", first, last),
        _ => println!("Array is empty"),
    }

    println!("Length: {}", arr.len());

    if let Some(stats) = compute_stats(&arr) {
        println!("{}", stats);
    } else {
        println!("No stats available for an empty array");
    }

    println!("\nIndexed elements:");
    for (i, val) in arr.iter().enumerate() {
        println!("  arr[{}] = {}", i, val);
    }

    let target = 30;
    match arr.iter().position(|&x| x == target) {
        Some(idx) => println!("\nFound {} at index {}", target, idx),
        None => println!("\n{} not found", target),
    }

    let evens: Vec<i32> = arr.iter().copied().filter(|x| x % 2 == 0).collect();
    let odds: Vec<i32> = arr.iter().copied().filter(|x| x % 2 != 0).collect();
    println!("Even elements: {:?}", evens);
    println!("Odd elements:  {:?}", odds);

    let mut sorted_desc = arr.to_vec();
    sorted_desc.sort_unstable_by(|a, b| b.cmp(a));
    println!("\nSorted descending: {:?}", sorted_desc);

    let reversed: Vec<i32> = arr.iter().rev().copied().collect();
    println!("Reversed: {:?}", reversed);

    let doubled: Vec<i32> = arr.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let windows: Vec<i32> = arr.windows(2).map(|w| w[1] - w[0]).collect();
    println!("Consecutive differences: {:?}", windows);
}
