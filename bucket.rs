use std::fs;
use std::io::{self, Write};

const FILE: &str = "bucketlist.txt";

fn read_line(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn save(items: &Vec<(String, bool)>) {
    let mut data = String::new();
    for (text, done) in items {
        let mark = if *done { "x" } else { " " };
        data.push_str(&format!("[{mark}] {text}\n"));
    }
    fs::write(FILE, data).unwrap();
}

fn load() -> Vec<(String, bool)> {
    let mut items = Vec::new();
    if let Ok(content) = fs::read_to_string(FILE) {
        for line in content.lines() {
            if let Some(text) = line.strip_prefix("[x] ") {
                items.push((text.to_string(), true));
            } else if let Some(text) = line.strip_prefix("[ ] ") {
                items.push((text.to_string(), false));
            }
        }
    }
    items
}

fn show(items: &Vec<(String, bool)>) {
    if items.is_empty() {
        println!("List is empty.");
    }
    for (i, (text, done)) in items.iter().enumerate() {
        let mark = if *done { "x" } else { " " };
        println!("{}. [{mark}] {text}", i + 1);
    }
}

fn main() {
    let mut items = load();

    loop {
        println!("\n1. Add  2. View  3. Toggle done  4. Remove  5. Save & exit");
        let choice = read_line("Choice: ");

        match choice.as_str() {
            "1" => {
                let text = read_line("New item: ");
                if !text.is_empty() {
                    items.push((text, false));
                }
            }
            "2" => show(&items),
            "3" => {
                show(&items);
                if let Ok(n) = read_line("Item number: ").parse::<usize>() {
                    if n >= 1 && n <= items.len() {
                        items[n - 1].1 = !items[n - 1].1;
                    }
                }
            }
            "4" => {
                show(&items);
                if let Ok(n) = read_line("Item number: ").parse::<usize>() {
                    if n >= 1 && n <= items.len() {
                        items.remove(n - 1);
                    }
                }
            }
            "5" => {
                save(&items);
                println!("Saved. Bye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}
