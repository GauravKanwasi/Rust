use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    DivisionByZero,
    EmptyInput,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(_) => write!(f, "Please enter a valid integer"),
            AppError::ParseFloat(_) => write!(f, "Please enter a valid number"),
            AppError::DivisionByZero => write!(f, "Cannot divide by zero"),
            AppError::EmptyInput => write!(f, "Input cannot be empty"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(e: std::num::ParseFloatError) -> Self {
        AppError::ParseFloat(e)
    }
}

struct History {
    entries: Vec<String>,
    max_size: usize,
}

impl History {
    fn new(max_size: usize) -> Self {
        History {
            entries: Vec::new(),
            max_size,
        }
    }

    fn push(&mut self, entry: String) {
        if self.entries.len() >= self.max_size {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }

    fn display(&self) {
        if self.entries.is_empty() {
            println!("  No command history yet.");
            return;
        }
        for (i, entry) in self.entries.iter().enumerate() {
            println!("  {:>3}. {}", i + 1, entry);
        }
    }
}

struct App {
    history: History,
    vars: HashMap<String, f64>,
    running: bool,
}

impl App {
    fn new() -> Self {
        App {
            history: History::new(20),
            vars: HashMap::new(),
            running: true,
        }
    }

    fn run(&mut self) {
        print_banner();
        while self.running {
            match self.prompt_and_dispatch() {
                Ok(_) => {}
                Err(e) => eprintln!("  ✖  {}", e),
            }
        }
    }

    fn prompt_and_dispatch(&mut self) -> Result<(), AppError> {
        let input = read_line("\n› ")?;
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();
        let args = parts.get(1).copied().unwrap_or("").trim();

        self.history.push(trimmed.to_string());

        match command.as_str() {
            "help" | "?" => self.cmd_help(),
            "greet" => self.cmd_greet(args),
            "add" | "+" => self.cmd_math(args, '+'),
            "sub" | "-" => self.cmd_math(args, '-'),
            "mul" | "*" => self.cmd_math(args, '*'),
            "div" | "/" => self.cmd_math(args, '/'),
            "calc" => self.cmd_calc(args),
            "let" => self.cmd_let(args),
            "vars" => self.cmd_vars(),
            "upper" => self.cmd_transform(args, true),
            "lower" => self.cmd_transform(args, false),
            "reverse" => self.cmd_reverse(args),
            "count" => self.cmd_count(args),
            "history" => self.cmd_history(),
            "clear" => self.cmd_clear(),
            "exit" | "quit" | "q" => {
                println!("\n  Goodbye! ✦\n");
                self.running = false;
            }
            _ => println!("  Unknown command '{}'. Type 'help' for options.", command),
        }

        Ok(())
    }

    fn cmd_help(&self) {
        println!("\n  ┌─────────────────────────────────────────────┐");
        println!("  │               AVAILABLE COMMANDS            │");
        println!("  ├──────────────┬──────────────────────────────┤");
        println!("  │ help / ?     │ Show this menu               │");
        println!("  │ greet <name> │ Get a personalised greeting  │");
        println!("  ├──────────────┼──────────────────────────────┤");
        println!("  │ add  <a> <b> │ Add two numbers              │");
        println!("  │ sub  <a> <b> │ Subtract two numbers         │");
        println!("  │ mul  <a> <b> │ Multiply two numbers         │");
        println!("  │ div  <a> <b> │ Divide two numbers           │");
        println!("  │ calc <expr>  │ Evaluate a+b, a-b, a*b, a/b  │");
        println!("  ├──────────────┼──────────────────────────────┤");
        println!("  │ let x = 3.14 │ Store a variable             │");
        println!("  │ vars         │ Show all variables           │");
        println!("  ├──────────────┼──────────────────────────────┤");
        println!("  │ upper <text> │ Convert text to UPPERCASE    │");
        println!("  │ lower <text> │ Convert text to lowercase    │");
        println!("  │ reverse <t>  │ Reverse a string             │");
        println!("  │ count <text> │ Count words and characters   │");
        println!("  ├──────────────┼──────────────────────────────┤");
        println!("  │ history      │ Show command history         │");
        println!("  │ clear        │ Clear the screen             │");
        println!("  │ exit / quit  │ Exit the program             │");
        println!("  └──────────────┴──────────────────────────────┘");
    }

    fn cmd_greet(&self, args: &str) {
        let name = if args.is_empty() {
            match read_line("  Enter your name: ") {
                Ok(n) => n.trim().to_string(),
                Err(e) => {
                    eprintln!("  ✖  {}", e);
                    return;
                }
            }
        } else {
            args.to_string()
        };

        if name.is_empty() {
            println!("  Hello, stranger! 👋");
            return;
        }

        let hour = get_hour_of_day();
        let greeting = match hour {
            5..=11 => "Good morning",
            12..=16 => "Good afternoon",
            17..=20 => "Good evening",
            _ => "Hello",
        };

        println!("  {} {}, welcome to the Rust CLI! 🚀", greeting, name);
    }

    fn cmd_math(&self, args: &str, op: char) {
        let (a, b) = match parse_two_floats(args) {
            Some(pair) => pair,
            None => {
                let a = match prompt_float("  First number : ") {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("  ✖  {}", e);
                        return;
                    }
                };
                let b = match prompt_float("  Second number: ") {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("  ✖  {}", e);
                        return;
                    }
                };
                (a, b)
            }
        };

        let result = match op {
            '+' => Ok(a + b),
            '-' => Ok(a - b),
            '*' => Ok(a * b),
            '/' => {
                if b == 0.0 {
                    Err(AppError::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            }
            _ => unreachable!(),
        };

        match result {
            Ok(r) => println!("  = {}", format_number(r)),
            Err(e) => eprintln!("  ✖  {}", e),
        }
    }

    fn cmd_calc(&self, args: &str) {
        let expr = if args.is_empty() {
            match read_line("  Expression (e.g. 3.5 * 7): ") {
                Ok(s) => s.trim().to_string(),
                Err(e) => {
                    eprintln!("  ✖  {}", e);
                    return;
                }
            }
        } else {
            args.to_string()
        };

        let ops = ['+', '-', '*', '/'];
        let found = ops.iter().find_map(|&op| {
            let parts: Vec<&str> = expr.splitn(2, op).collect();
            if parts.len() == 2 {
                let a = parts[0].trim().parse::<f64>().ok()?;
                let b = parts[1].trim().parse::<f64>().ok()?;
                Some((a, b, op))
            } else {
                None
            }
        });

        match found {
            Some((a, b, op)) => {
                let result = match op {
                    '+' => Ok(a + b),
                    '-' => Ok(a - b),
                    '*' => Ok(a * b),
                    '/' if b == 0.0 => Err(AppError::DivisionByZero),
                    '/' => Ok(a / b),
                    _ => unreachable!(),
                };
                match result {
                    Ok(r) => println!("  {} {} {} = {}", a, op, b, format_number(r)),
                    Err(e) => eprintln!("  ✖  {}", e),
                }
            }
            None => eprintln!("  ✖  Could not parse expression. Use format: 3.5 + 2"),
        }
    }

    fn cmd_let(&mut self, args: &str) {
        let parts: Vec<&str> = args.splitn(3, ' ').collect();
        if parts.len() < 3 || parts[1] != "=" {
            eprintln!("  ✖  Usage: let <name> = <value>");
            return;
        }
        let name = parts[0].to_string();
        match parts[2].trim().parse::<f64>() {
            Ok(val) => {
                self.vars.insert(name.clone(), val);
                println!("  {} = {}", name, format_number(val));
            }
            Err(_) => eprintln!("  ✖  Invalid number: {}", parts[2]),
        }
    }

    fn cmd_vars(&self) {
        if self.vars.is_empty() {
            println!("  No variables stored. Use 'let x = 42' to create one.");
            return;
        }
        let mut sorted: Vec<(&String, &f64)> = self.vars.iter().collect();
        sorted.sort_by_key(|(k, _)| k.as_str());
        println!("\n  Stored variables:");
        for (k, v) in sorted {
            println!("    {} = {}", k, format_number(*v));
        }
    }

    fn cmd_transform(&self, args: &str, upper: bool) {
        let text = if args.is_empty() {
            match read_line("  Enter text: ") {
                Ok(s) => s.trim().to_string(),
                Err(e) => {
                    eprintln!("  ✖  {}", e);
                    return;
                }
            }
        } else {
            args.to_string()
        };
        if upper {
            println!("  {}", text.to_uppercase());
        } else {
            println!("  {}", text.to_lowercase());
        }
    }

    fn cmd_reverse(&self, args: &str) {
        let text = if args.is_empty() {
            match read_line("  Enter text: ") {
                Ok(s) => s.trim().to_string(),
                Err(e) => {
                    eprintln!("  ✖  {}", e);
                    return;
                }
            }
        } else {
            args.to_string()
        };
        let reversed: String = text.chars().rev().collect();
        println!("  {}", reversed);
    }

    fn cmd_count(&self, args: &str) {
        let text = if args.is_empty() {
            match read_line("  Enter text: ") {
                Ok(s) => s.trim().to_string(),
                Err(e) => {
                    eprintln!("  ✖  {}", e);
                    return;
                }
            }
        } else {
            args.to_string()
        };
        let chars = text.chars().count();
        let words = text.split_whitespace().count();
        let lines = text.lines().count().max(1);
        println!("  Characters : {}", chars);
        println!("  Words      : {}", words);
        println!("  Lines      : {}", lines);
    }

    fn cmd_history(&self) {
        println!("\n  Command history:");
        self.history.display();
    }

    fn cmd_clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
        let _ = io::stdout().flush();
        print_banner();
    }
}

fn print_banner() {
    println!("\n  ╔══════════════════════════════════════╗");
    println!("  ║         RUST  INTERACTIVE  CLI       ║");
    println!("  ║   type 'help' to see all commands    ║");
    println!("  ╚══════════════════════════════════════╝");
}

fn read_line(prompt: &str) -> Result<String, AppError> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn prompt_float(prompt: &str) -> Result<f64, AppError> {
    loop {
        let input = read_line(prompt)?;
        match input.trim().parse::<f64>() {
            Ok(v) => return Ok(v),
            Err(e) => eprintln!("  ✖  {}", AppError::ParseFloat(e)),
        }
    }
}

fn parse_two_floats(s: &str) -> Option<(f64, f64)> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() >= 2 {
        let a = parts[0].parse::<f64>().ok()?;
        let b = parts[1].parse::<f64>().ok()?;
        Some((a, b))
    } else {
        None
    }
}

fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{:.6}", n)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

fn get_hour_of_day() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    ((secs % 86400) / 3600) as u32
}

fn main() {
    App::new().run();
}
