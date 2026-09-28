use std::fs;
use std::io::{self, Write};

const FILE_NAME: &str = "bucketlist.txt";
const TMP_FILE_NAME: &str = "bucketlist.txt.tmp";

#[derive(Debug, Clone, PartialEq)]
struct Item {
    text: String,
    done: bool,
}

impl Item {
    fn new(text: String) -> Self {
        Self { text, done: false }
    }

    /// Parses "[x] text" / "[ ] text". Plain lines (old file format) are
    /// treated as not-done items so existing lists keep working.
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        let item = if let Some(rest) = line.strip_prefix("[x] ") {
            Self { text: rest.to_string(), done: true }
        } else if let Some(rest) = line.strip_prefix("[ ] ") {
            Self { text: rest.to_string(), done: false }
        } else {
            Self::new(line.to_string())
        };
        Some(item)
    }

    fn serialize(&self) -> String {
        format!("[{}] {}", if self.done { 'x' } else { ' ' }, self.text)
    }
}

struct BucketList {
    items: Vec<Item>,
    dirty: bool,
}

impl BucketList {
    fn load() -> Self {
        let items = match fs::read_to_string(FILE_NAME) {
            Ok(content) => content.lines().filter_map(Item::parse).collect(),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => {
                eprintln!("Warning: could not read {FILE_NAME}: {e}");
                Vec::new()
            }
        };
        Self { items, dirty: false }
    }

    /// Writes to a temp file first, then renames, so a crash mid-write
    /// can't corrupt the existing list.
    fn save(&mut self) -> io::Result<()> {
        let data: Vec<String> = self.items.iter().map(Item::serialize).collect();
        fs::write(TMP_FILE_NAME, data.join("\n"))?;
        fs::rename(TMP_FILE_NAME, FILE_NAME)?;
        self.dirty = false;
        Ok(())
    }

    fn add(&mut self, text: String) -> Result<(), &'static str> {
        if text.is_empty() {
            return Err("Empty item not added.");
        }
        if self.items.iter().any(|i| i.text.eq_ignore_ascii_case(&text)) {
            return Err("That item is already on your list.");
        }
        self.items.push(Item::new(text));
        self.dirty = true;
        Ok(())
    }

    fn view(&self) {
        if self.items.is_empty() {
            println!("Bucket list is empty.");
            return;
        }
        let done = self.items.iter().filter(|i| i.done).count();
        println!("\nYour Bucket List ({done}/{} completed):", self.items.len());
        let width = self.items.len().to_string().len();
        for (i, item) in self.items.iter().enumerate() {
            let mark = if item.done { "x" } else { " " };
            println!("{:>width$}. [{mark}] {}", i + 1, item.text);
        }
    }

    fn toggle(&mut self, index: usize) -> bool {
        let item = &mut self.items[index];
        item.done = !item.done;
        self.dirty = true;
        item.done
    }

    fn remove(&mut self, index: usize) -> Item {
        self.dirty = true;
        self.items.remove(index)
    }
}

/// Prints a prompt and reads one trimmed line. Returns `None` on EOF
/// (Ctrl-D / Ctrl-Z) or a read error, so the caller can exit cleanly
/// instead of looping forever.
fn prompt(msg: &str) -> Option<String> {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(input.trim().to_string()),
    }
}

fn confirm(msg: &str) -> bool {
    matches!(
        prompt(&format!("{msg} (y/N): ")).as_deref().map(str::to_lowercase).as_deref(),
        Some("y" | "yes")
    )
}

/// Asks for a 1-based item number and returns a validated 0-based index.
fn select_index(list: &BucketList, msg: &str) -> Option<usize> {
    if list.items.is_empty() {
        println!("Bucket list is empty.");
        return None;
    }
    list.view();
    let input = prompt(msg)?;
    match input.parse::<usize>() {
        Ok(n) if (1..=list.items.len()).contains(&n) => Some(n - 1),
        Ok(_) => {
            println!("Invalid item number.");
            None
        }
        Err(_) => {
            println!("Invalid input.");
            None
        }
    }
}

fn save_and_report(list: &mut BucketList) -> bool {
    match list.save() {
        Ok(()) => {
            println!("Bucket list saved.");
            true
        }
        Err(e) => {
            eprintln!("Error: failed to save bucket list: {e}");
            false
        }
    }
}

fn main() {
    let mut list = BucketList::load();
    println!("Loaded {} item(s).", list.items.len());

    loop {
        println!("\n--- Bucket List Menu ---");
        println!("1. Add item");
        println!("2. View items");
        println!("3. Mark item done / not done");
        println!("4. Remove item");
        println!("5. Save");
        println!("6. Save and exit");
        println!("7. Exit without saving");

        let Some(choice) = prompt("Enter your choice: ") else {
            // EOF: don't lose work.
            println!();
            if list.dirty {
                save_and_report(&mut list);
            }
            break;
        };

        match choice.as_str() {
            "1" => {
                if let Some(text) = prompt("Enter a new bucket list item: ") {
                    match list.add(text) {
                        Ok(()) => println!("Item added."),
                        Err(msg) => println!("{msg}"),
                    }
                }
            }
            "2" => list.view(),
            "3" => {
                if let Some(i) = select_index(&list, "Enter item number to toggle: ") {
                    let state = if list.toggle(i) { "done" } else { "not done" };
                    println!("Marked as {state}.");
                }
            }
            "4" => {
                if let Some(i) = select_index(&list, "Enter item number to remove: ") {
                    let removed = list.remove(i);
                    println!("Removed \"{}\".", removed.text);
                }
            }
            "5" => {
                save_and_report(&mut list);
            }
            "6" => {
                if save_and_report(&mut list) {
                    println!("Exiting...");
                    break;
                }
            }
            "7" => {
                if !list.dirty || confirm("You have unsaved changes. Exit anyway?") {
                    println!("Exiting without saving...");
                    break;
                }
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
