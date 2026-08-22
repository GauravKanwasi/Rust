use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use rand::Rng;
use colored::*;

struct Difficulty {
    name: &'static str,
    max: u32,
    max_attempts: u32,
}

const DIFFICULTIES: [Difficulty; 3] = [
    Difficulty { name: "Easy",   max: 50,  max_attempts: 10 },
    Difficulty { name: "Medium", max: 100, max_attempts: 7 },
    Difficulty { name: "Hard",   max: 200, max_attempts: 6 },
];

struct SessionStats {
    games_played: u32,
    total_attempts: u32,
    best: Option<u32>,
}

impl SessionStats {
    fn new() -> Self {
        SessionStats { games_played: 0, total_attempts: 0, best: None }
    }

    fn record(&mut self, attempts: u32) {
        self.games_played += 1;
        self.total_attempts += attempts;
        self.best = Some(match self.best {
            Some(b) => b.min(attempts),
            None => attempts,
        });
    }

    fn print_summary(&self) {
        if self.games_played == 0 {
            return;
        }
        let avg = self.total_attempts as f64 / self.games_played as f64;
        println!("{}", "╔══════════════════════════════════════╗".bright_red().bold());
        println!("{}", "║           SESSION SUMMARY             ║".bright_red().bold());
        println!("{}", "╚══════════════════════════════════════╝".bright_red().bold());
        println!("  games played : {}", self.games_played.to_string().bright_white().bold());
        println!("  avg attempts : {:.1}", avg);
        println!(
            "  best score   : {}",
            self.best.unwrap().to_string().bright_yellow().bold()
        );
        println!();
    }
}

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
    println!("{}", "║         FUN.RS  v0.3.0               ║".bright_red().bold());
    println!("{}", "║   the most serious fun in Rust        ║".red());
    println!("{}", "╚══════════════════════════════════════╝".bright_red().bold());
    println!();
}

fn print_range_bar(low: u32, high: u32, range_max: u32) {
    let width: usize = 40;
    let left = ((low - 1) as f64 / range_max as f64 * width as f64) as usize;
    let right = (high as f64 / range_max as f64 * width as f64).min(width as f64) as usize;

    print!("  {} ", "1".dimmed());
    for i in 0..width {
        if i >= left && i < right {
            print!("{}", "█".bright_red());
        } else {
            print!("{}", "░".dark_grey());
        }
    }
    println!(" {}", range_max.to_string().dimmed());

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

fn choose_difficulty() -> &'static Difficulty {
    println!("  {}", "Choose your difficulty:".bold());
    for (i, d) in DIFFICULTIES.iter().enumerate() {
        println!(
            "    {}. {}  (1-{}, {} attempts)",
            i + 1,
            d.name.bright_yellow(),
            d.max,
            d.max_attempts
        );
    }
    loop {
        print!("  {} ", "$".bright_red().bold());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= DIFFICULTIES.len() => {
                return &DIFFICULTIES[n - 1];
            }
            _ => {
                println!("  {} Pick 1, 2, or 3.", "!".yellow());
            }
        }
    }
}

fn play_round(difficulty: &Difficulty) -> Option<u32> {
    let secret: u32 = rand::thread_rng().gen_range(1..=difficulty.max);
    let mut attempts = 0u32;
    let mut low = 1u32;
    let mut high = difficulty.max;
    let max_attempts = difficulty.max_attempts;
    let mut prev_distance: Option<u32> = None;
    let mut hint_used = false;

    loop {
        print_range_bar(low, high, difficulty.max);
        print_attempts_bar(attempts, max_attempts);
        println!();
        println!(
            "  {} {}",
            "->".bright_red(),
            "Enter your guess ('hint' for a clue, 'quit' to escape reality):".bold()
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
            return None;
        }

        if input.eq_ignore_ascii_case("hint") {
            if hint_used {
                println!("  {} Only one hint per round. No freebies.\n", "!".yellow());
            } else {
                hint_used = true;
                let parity = if secret % 2 == 0 { "even" } else { "odd" };
                println!("  {} The number is {}.\n", "hint:".bright_cyan().bold(), parity);
            }
            thread::sleep(Duration::from_millis(900));
            clear_lines(2);
            continue;
        }

        let guess: u32 = match input.parse() {
            Ok(n) if (1..=difficulty.max).contains(&n) => n,
            Ok(_) => {
                println!(
                    "  {} Number must be between 1 and {}. Stay in range.\n",
                    "!".yellow(),
                    difficulty.max
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
        let distance = secret.abs_diff(guess);
        let temperature = match prev_distance {
            Some(prev) if distance < prev => "warmer".bright_green().bold(),
            Some(prev) if distance > prev => "colder".bright_blue().bold(),
            Some(_) => "same temp".dimmed(),
            None => "".normal(),
        };
        prev_distance = Some(distance);

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                low = low.max(guess + 1);
                println!(
                    "  {} {} -- think bigger. {}\n",
                    "^  Too small.".yellow().bold(),
                    format!("[{}]", guess).dimmed(),
                    temperature
                );
                thread::sleep(Duration::from_millis(500));
                clear_lines(2);
            }
            std::cmp::Ordering::Greater => {
                high = high.min(guess - 1);
                println!(
                    "  {} {} -- humble yourself. {}\n",
                    "v  Too big.".yellow().bold(),
                    format!("[{}]", guess).dimmed(),
                    temperature
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
                    1 => "Legendary. Are you cheating?".bright_magenta().bold(),
                    n if n <= max_attempts / 2 + 1 => "Impressive. Rust approves.".bright_green().bold(),
                    n if n <= max_attempts => "Not bad. Acceptable performance.".yellow().bold(),
                    _ => "Eventually... success.".dimmed(),
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
                return Some(attempts);
            }
        }
    }
}

fn main() {
    print_header();
    slow_print("Welcome to FUN.RS -- the most serious fun you'll ever have in Rust.", 18);
    thread::sleep(Duration::from_millis(200));
    slow_print("Pick a difficulty and I'll think of a number in that range.", 20);
    thread::sleep(Duration::from_millis(150));
    println!();

    let mut stats = SessionStats::new();

    loop {
        let difficulty = choose_difficulty();
        println!();
        slow_print(
            &format!(
                "{} mode: guessing between 1 and {}. Go.\n",
                difficulty.name, difficulty.max
            ),
            15,
        );
        thread::sleep(Duration::from_millis(200));

        if let Some(attempts) = play_round(difficulty) {
            stats.record(attempts);
        }

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

    println!();
    stats.print_summary();
    slow_print("Thanks for playing FUN.RS. May your code compile on the first try.", 18);
    println!();
}
