use std::env;
use std::io::{self, Write};

const WIDTH: usize = 60;

struct Stats {
    comparisons: usize,
    shifts: usize,
    passes: usize,
}

struct Palette {
    green: &'static str,
    yellow: &'static str,
    grey: &'static str,
    reset: &'static str,
}

impl Palette {
    fn new() -> Self {
        if env::var_os("NO_COLOR").is_some() {
            Palette { green: "", yellow: "", grey: "", reset: "" }
        } else {
            Palette {
                green: "\x1b[32m",
                yellow: "\x1b[1;33m",
                grey: "\x1b[90m",
                reset: "\x1b[0m",
            }
        }
    }
}

fn render(arr: &[i32], sorted_end: usize, key_pos: usize, p: &Palette) -> String {
    let cell_width = arr.iter().map(|v| v.to_string().len()).max().unwrap_or(1) + 1;
    let mut out = String::new();
    for (idx, v) in arr.iter().enumerate() {
        let color = if idx == key_pos {
            p.yellow
        } else if idx <= sorted_end {
            p.green
        } else {
            p.grey
        };
        out.push_str(&format!("{}{:>w$}{}", color, v, p.reset, w = cell_width));
    }
    out
}

fn insertion_sort(arr: &mut [i32]) -> Stats {
    let p = Palette::new();
    let mut stats = Stats { comparisons: 0, shifts: 0, passes: 0 };

    println!("{}", "═".repeat(WIDTH));
    println!("{:^w$}", "INSERTION SORT VISUALIZER", w = WIDTH);
    println!("{}", "═".repeat(WIDTH));
    println!("Initial: {:?}", arr);
    println!("{}", "─".repeat(WIDTH));

    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        let mut pass_shifts = 0;
        let mut pass_comparisons = 0;

        while j > 0 {
            pass_comparisons += 1;
            if arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
                pass_shifts += 1;
            } else {
                break;
            }
        }
        arr[j] = key;

        stats.comparisons += pass_comparisons;
        stats.shifts += pass_shifts;
        stats.passes += 1;

        println!("Pass {:>2} │ Key: {:>4} │ Sorted: {:?}", i, key, &arr[..=i]);
        println!(
            "       │ {} comparison{}, {} shift{}, inserted at index {}",
            pass_comparisons,
            if pass_comparisons == 1 { "" } else { "s" },
            pass_shifts,
            if pass_shifts == 1 { "" } else { "s" },
            j
        );
        println!("       │ {}", render(arr, i, j, &p));
        println!("{}", "─".repeat(WIDTH));
    }

    println!("{:^w$}", "SORTED ✓", w = WIDTH);
    println!("{}", "═".repeat(WIDTH));
    stats
}

fn read_numbers() -> Option<Vec<i32>> {
    loop {
        print!("Enter numbers separated by spaces or commas (q to quit): ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input).ok()? == 0 {
            return None;
        }

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("q") {
            return None;
        }
        if trimmed.is_empty() {
            eprintln!("Error: no input provided. Try again.\n");
            continue;
        }

        let parsed: Result<Vec<i32>, _> = trimmed
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>())
            .collect();

        match parsed {
            Ok(nums) => return Some(nums),
            Err(e) => eprintln!("Error: {}. Please enter valid integers only.\n", e),
        }
    }
}

fn main() {
    let Some(mut numbers) = read_numbers() else {
        return;
    };

    if numbers.len() < 2 {
        println!("Array has fewer than 2 elements, nothing to sort.");
        println!("Result: {:?}", numbers);
        return;
    }

    let stats = insertion_sort(&mut numbers);

    println!("\n  Final sorted array: {:?}", numbers);
    println!(
        "  Passes: {} | Comparisons: {} | Shifts: {}",
        stats.passes, stats.comparisons, stats.shifts
    );
}
