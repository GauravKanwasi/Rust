use std::io;
use std::io::Write;

fn insertion_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    let width = 60;
    
    println!("{}", "═".repeat(width));
    println!("{:^width$}", "INSERTION SORT VISUALIZER");
    println!("{}", "═".repeat(width));
    println!("{:^width$}", format!("Initial: {:?}", arr));
    println!("{}", "─".repeat(width));

    for i in 1..n {
        let key = arr[i];
        let mut j = i as isize - 1;
        let comparisons_start = j;

        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;

        println!("Pass {:>2} │ Key: {:>4} │ Sorted: {:?}", i, key, &arr[..=i]);
        
        let shifts = (comparisons_start - j) as usize;
        if shifts > 0 {
            println!("       │ Shifted {} element{} right", shifts, if shifts == 1 { "" } else { "s" });
        } else {
            println!("       │ Key already in position");
        }

        print!("       │ ");
        for (idx, &val) in arr.iter().enumerate() {
            if idx < i {
                print!("\x1b[32m{:>3}\x1b[0m", val);
            } else if idx == i {
                print!("\x1b[33m{:>3}\x1b[0m", val);
            } else {
                print!("\x1b[90m{:>3}\x1b[0m", val);
            }
        }
        println!();
        println!("{}", "─".repeat(width));
    }

    println!("{:^width$}", "SORTED ✓");
    println!("{}", "═".repeat(width));
}

fn main() {
    print!("Enter numbers separated by spaces: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let trimmed = input.trim();
    if trimmed.is_empty() {
        eprintln!("Error: No input provided.");
        return;
    }

    let parse_result: Result<Vec<i32>, _> = trimmed
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect();

    let mut numbers = match parse_result {
        Ok(nums) => nums,
        Err(_) => {
            eprintln!("Error: Please enter valid integers only.");
            return;
        }
    };

    if numbers.len() < 2 {
        println!("Array has fewer than 2 elements — nothing to sort.");
        println!("Result: {:?}", numbers);
        return;
    }

    insertion_sort(&mut numbers);
    println!("\n  Final sorted array: {:?}", numbers);
}
