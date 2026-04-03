use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use rand::Rng;
use colored::*;

fn slow_print(message: &str, delay_ms: u64) {
    for c in message.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

fn print_header() {
    println!();
    println!("{}", "╔══════════════════════════════════════╗".bright_red().bold());
    println!("{}", "║         FUN.RS  v0.2.0               ║".bright_red().bold());
    println!("{}", "║   the most serious fun in Rust        ║".red());
    println!("{}", "╚══════════════════════════════════════╝".bright_red().bold());
    println!();
}

fn print_range_bar(low: u32, high: u32) {
    let width: usize = 40;
    let left = ((low - 1) as f64 / 100.0 * width as f64) as usize;
    let right = (high as f64 / 100.0 * width as f64).min(width as f64) as usize;

    print!("  {} ", "1".dimmed());
    for i in 0..width {
        if i >= left && i < right {
            print!("{}", "█".bright_red());
        } else {
            print!("{}", "░".dark_grey());
        }
    }
    println!(" {}", "100".dimmed());

    let low_str = format!("{}↑", low);
    let high_str = format!("↑{}", high);
    let gap = right.saturating_sub(left).saturating_sub(low_str.len());
    println!(
        "     {}{}{}",
        " ".repeat(left),
        low_str.yellow(),
        if gap >= high_str.len() {
            format!("{:>width$}", high_str, width = gap).yellow().to_string()
        } else {
            String::new()
        }
    );
    println!();
}

fn print_attempts_bar(attempts: u32, max: u32) {
    let filled = attempts.min(max) as usize;
    let empty = (max - attempts.min(max)) as usize;
    print!("  attempts [");
    print!("{}", "■".repeat(filled).bright_red());
    print!("{}", "·".repeat(empty).dark_grey());
    println!("] {}/{}", attempts, max);
}

fn clear_lines(n: u32) {
    for _ in 0..n {
        print!("\x1B[1A\x1B[2K");
    }
    io::stdout().flush().unwrap();
}

fn play_round() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0u32;
    let mut low = 1u32;
    let mut high = 100u32;
    let max_attempts = 10u32;

    loop {
        print_range_bar(low, high);
        print_attempts_bar(attempts, max_attempts);
        println!();
        println!(
            "  {} {}",
            "->".bright_red(),
            "Enter your guess (or 'quit' to escape reality):".bold()
        );
        print!("  {} ", "$".bright_red().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        clear_lines(7);

        if input.eq_ignore_ascii_case("quit") {
            println!(
                "\n  {} The number was {}. I forgive you.\n",
                "You gave up.".red().bold(),
                secret.to_string().bright_yellow().bold()
            );
            return;
        }

        let guess: u32 = match input.parse() {
            Ok(n) if (1..=100).contains(&n) => n,
            Ok(_) => {
                println!(
                    "  {} Number must be between 1 and 100. Stay in range.\n",
                    "!".yellow(),
                );
                thread::sleep(Duration::from_millis(700));
                clear_lines(2);
                continue;
            }
            Err(_) => {
                println!(
                    "  {} That's not a number. Rust is disappointed.\n",
                    "x".red().bold()
                );
                thread::sleep(Duration::from_millis(700));
                clear_lines(2);
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                low = low.max(guess + 1);
                println!(
                    "  {} {} -- think bigger.\n",
                    "^  Too small.".yellow().bold(),
                    format!("[{}]", guess).dimmed()
                );
                thread::sleep(Duration::from_millis(500));
                clear_lines(2);
            }
            std::cmp::Ordering::Greater => {
                high = high.min(guess - 1);
                println!(
                    "  {} {} -- humble yourself.\n",
                    "v  Too big.".yellow().bold(),
                    format!("[{}]", guess).dimmed()
                );
                thread::sleep(Duration::from_millis(500));
                clear_lines(2);
            }
            std::cmp::Ordering::Equal => {
                println!();
                println!("  {}", "CORRECT!".green().bold());
                println!();
                println!(
                    "  You guessed {} in {} {}.",
                    secret.to_string().bright_yellow().bold(),
                    attempts.to_string().bright_white().bold(),
                    if attempts == 1 { "attempt" } else { "attempts" }
                );
                println!();

                let verdict = match attempts {
                    1      => "Legendary. Are you cheating?".bright_magenta().bold(),
                    2..=5  => "Impressive. Rust approves.".bright_green().bold(),
                    6..=10 => "Not bad. Acceptable performance.".yellow().bold(),
                    _      => "Eventually... success.".dimmed(),
                };
                println!("  {}", verdict);
                println!();

                let frames = [
                    "[=         ]",
                    "[==        ]",
                    "[===       ]",
                    "[====      ]",
                    "[=====     ]",
                    "[======    ]",
                    "[=======   ]",
                    "[========  ]",
                    "[========= ]",
                    "[==========]",
                ];
                for frame in &frames {
                    print!("\r  {}", frame.green());
                    io::stdout().flush().unwrap();
                    thread::sleep(Duration::from_millis(55));
                }
                println!(" done");
                println!();
                return;
            }
        }
    }
}

fn main() {
    print_header();
    slow_print("Welcome to FUN.RS -- the most serious fun you'll ever have in Rust.", 18);
    thread::sleep(Duration::from_millis(200));
    slow_print("I am thinking of a number between 1 and 100.", 20);
    thread::sleep(Duration::from_millis(150));
    slow_print("Can you guess it? Let's find out.\n", 20);
    thread::sleep(Duration::from_millis(300));

    loop {
        play_round();

        println!("  {} Play again? (y/n)", "->".bright_red());
        print!("  {} ", "$".bright_red().bold());
        io::stdout().flush().unwrap();

        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read input");
        clear_lines(3);

        match again.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                println!("  {}\n", "New game started. Good luck.".dimmed());
                thread::sleep(Duration::from_millis(400));
                clear_lines(2);
            }
            _ => break,
        }
    }

    slow_print("\nThanks for playing FUN.RS. May your code compile on the first try.", 18);
    println!();
}
