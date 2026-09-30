use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, process};

const DEFAULT_FILE: &str = "bucketlist.txt";
const BAR_WIDTH: usize = 20;

#[derive(Debug, Clone, PartialEq)]
struct Item {
    text: String,
    done: bool,
}

impl Item {
    fn new(text: String) -> Self {
        Self { text, done: false }
    }

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

#[derive(Clone, Copy)]
enum Filter {
    All,
    Pending,
    Done,
}

impl Filter {
    fn matches(self, item: &Item) -> bool {
        match self {
            Filter::All => true,
            Filter::Pending => !item.done,
            Filter::Done => item.done,
        }
    }
}

struct BucketList {
    items: Vec<Item>,
    path: PathBuf,
    dirty: bool,
}

impl BucketList {
    fn new(path: PathBuf) -> Self {
        Self { items: Vec::new(), path, dirty: false }
    }

    /// A missing file is fine (fresh list); any other read error is fatal so
    /// we never overwrite a file we couldn't read.
    fn load(path: PathBuf) -> io::Result<Self> {
        let mut list = Self::new(path);
        match fs::read_to_string(&list.path) {
            Ok(content) => list.items = content.lines().filter_map(Item::parse).collect(),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        Ok(list)
    }

    fn tmp_path(&self) -> PathBuf {
        let mut s = self.path.as_os_str().to_owned();
        s.push(".tmp");
        PathBuf::from(s)
    }

    fn save(&mut self) -> io::Result<()> {
        let mut data = self.items.iter().map(Item::serialize).collect::<Vec<_>>().join("\n");
        if !data.is_empty() {
            data.push('\n');
        }
        let tmp = self.tmp_path();
        let mut file = fs::File::create(&tmp)?;
        file.write_all(data.as_bytes())?;
        file.sync_all()?;
        drop(file);
        fs::rename(&tmp, &self.path)?;
        self.dirty = false;
        Ok(())
    }

    fn find_duplicate(&self, text: &str, skip: Option<usize>) -> bool {
        let needle = text.to_lowercase();
        self.items
            .iter()
            .enumerate()
            .any(|(i, item)| Some(i) != skip && item.text.to_lowercase() == needle)
    }

    fn add(&mut self, text: &str) -> Result<(), &'static str> {
        let text = normalize(text);
        if text.is_empty() {
            return Err("Empty item not added.");
        }
        if self.find_duplicate(&text, None) {
            return Err("That item is already on your list.");
        }
        self.items.push(Item::new(text));
        self.dirty = true;
        Ok(())
    }

    fn edit(&mut self, index: usize, text: &str) -> Result<(), &'static str> {
        let text = normalize(text);
        if text.is_empty() {
            return Err("Text cannot be empty.");
        }
        if self.find_duplicate(&text, Some(index)) {
            return Err("Another item already has that text.");
        }
        if self.items[index].text != text {
            self.items[index].text = text;
            self.dirty = true;
        }
        Ok(())
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

    fn done_count(&self) -> usize {
        self.items.iter().filter(|i| i.done).count()
    }

    fn clear_completed(&mut self) -> usize {
        let before = self.items.len();
        self.items.retain(|i| !i.done);
        let removed = before - self.items.len();
        if removed > 0 {
            self.dirty = true;
        }
        removed
    }

    fn search(&self, query: &str) -> Vec<usize> {
        let q = query.to_lowercase();
        self.items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.text.to_lowercase().contains(&q))
            .map(|(idx, _)| idx)
            .collect()
    }

    fn print_indices(&self, indices: impl Iterator<Item = usize>) -> usize {
        let width = self.items.len().to_string().len();
        let mut shown = 0;
        for i in indices {
            let item = &self.items[i];
            let mark = if item.done { "x" } else { " " };
            println!("{:>width$}. [{mark}] {}", i + 1, item.text);
            shown += 1;
        }
        shown
    }

    fn view(&self, filter: Filter) {
        if self.items.is_empty() {
            println!("Bucket list is empty.");
            return;
        }
        let (done, total) = (self.done_count(), self.items.len());
        println!("\nYour Bucket List: {done}/{total} completed ({}%)", done * 100 / total);
        println!("{}", progress_bar(done, total, BAR_WIDTH));
        let shown = self.print_indices((0..total).filter(|&i| filter.matches(&self.items[i])));
        if shown == 0 {
            println!("  (nothing to show)");
        }
    }
}

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn progress_bar(done: usize, total: usize, width: usize) -> String {
    let filled = if total == 0 { 0 } else { done * width / total };
    format!("[{}{}]", "#".repeat(filled), "-".repeat(width - filled))
}

/// Returns `None` on EOF or read error so callers can exit cleanly.
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

fn select_index(list: &BucketList, msg: &str) -> Option<usize> {
    if list.items.is_empty() {
        println!("Bucket list is empty.");
        return None;
    }
    list.view(Filter::All);
    let input = prompt(msg)?;
    if input.is_empty() {
        return None;
    }
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
            println!("Bucket list saved to {}.", list.path.display());
            true
        }
        Err(e) => {
            eprintln!("Error: failed to save bucket list: {e}");
            false
        }
    }
}

fn add_flow(list: &mut BucketList) {
    if let Some(text) = prompt("Enter a new bucket list item: ") {
        match list.add(&text) {
            Ok(()) => println!("Item added."),
            Err(msg) => println!("{msg}"),
        }
    }
}

fn view_flow(list: &BucketList) {
    let filter = match prompt("Show (a)ll, (p)ending, (d)one [a]: ")
        .as_deref()
        .map(str::to_lowercase)
        .as_deref()
    {
        Some("p" | "pending") => Filter::Pending,
        Some("d" | "done") => Filter::Done,
        _ => Filter::All,
    };
    list.view(filter);
}

fn toggle_flow(list: &mut BucketList) {
    if let Some(i) = select_index(list, "Enter item number to toggle (blank to cancel): ") {
        let state = if list.toggle(i) { "done" } else { "not done" };
        println!("Marked \"{}\" as {state}.", list.items[i].text);
    }
}

fn edit_flow(list: &mut BucketList) {
    let Some(i) = select_index(list, "Enter item number to edit (blank to cancel): ") else {
        return;
    };
    println!("Current: {}", list.items[i].text);
    let Some(text) = prompt("New text (blank to cancel): ") else {
        return;
    };
    if text.is_empty() {
        return;
    }
    match list.edit(i, &text) {
        Ok(()) => println!("Item updated."),
        Err(msg) => println!("{msg}"),
    }
}

fn remove_flow(list: &mut BucketList) {
    if let Some(i) = select_index(list, "Enter item number to remove (blank to cancel): ") {
        let removed = list.remove(i);
        println!("Removed \"{}\".", removed.text);
    }
}

fn search_flow(list: &BucketList) {
    let Some(query) = prompt("Search for: ") else {
        return;
    };
    if query.is_empty() {
        return;
    }
    let hits = list.search(&query);
    if hits.is_empty() {
        println!("No items match \"{query}\".");
    } else {
        println!("\n{} match(es):", hits.len());
        list.print_indices(hits.into_iter());
    }
}

fn clear_flow(list: &mut BucketList) {
    let n = list.done_count();
    if n == 0 {
        println!("No completed items to clear.");
    } else if confirm(&format!("Remove {n} completed item(s)?")) {
        list.clear_completed();
        println!("Cleared {n} item(s).");
    }
}

fn main() {
    let path = env::args_os().nth(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from(DEFAULT_FILE));
    let mut list = match BucketList::load(path.clone()) {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error: could not read {}: {e}", path.display());
            process::exit(1);
        }
    };
    println!("Loaded {} item(s) from {}.", list.items.len(), path.display());

    loop {
        println!("\n--- Bucket List Menu ---");
        println!("1. Add item");
        println!("2. View items");
        println!("3. Mark item done / not done");
        println!("4. Edit item");
        println!("5. Remove item");
        println!("6. Search");
        println!("7. Clear completed items");
        println!("8. Save");
        println!("9. Save and exit");
        println!("0. Exit without saving");

        let Some(choice) = prompt("Enter your choice: ") else {
            println!();
            if list.dirty {
                save_and_report(&mut list);
            }
            break;
        };

        match choice.as_str() {
            "1" => add_flow(&mut list),
            "2" => view_flow(&list),
            "3" => toggle_flow(&mut list),
            "4" => edit_flow(&mut list),
            "5" => remove_flow(&mut list),
            "6" => search_flow(&list),
            "7" => clear_flow(&mut list),
            "8" => {
                save_and_report(&mut list);
            }
            "9" => {
                if save_and_report(&mut list) {
                    println!("Exiting...");
                    break;
                }
            }
            "0" => {
                if !list.dirty || confirm("You have unsaved changes. Exit anyway?") {
                    println!("Exiting without saving...");
                    break;
                }
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list() -> BucketList {
        BucketList::new(PathBuf::from("unused.txt"))
    }

    #[test]
    fn parse_and_serialize_roundtrip() {
        for line in ["[x] Skydive", "[ ] Visit Japan"] {
            assert_eq!(Item::parse(line).unwrap().serialize(), line);
        }
        assert_eq!(Item::parse("Plain line").unwrap(), Item::new("Plain line".into()));
        assert!(Item::parse("   ").is_none());
    }

    #[test]
    fn add_rejects_empty_and_duplicates() {
        let mut l = list();
        assert!(l.add("  Learn Rust ").is_ok());
        assert!(l.add("learn   rust").is_err());
        assert!(l.add("   ").is_err());
        assert_eq!(l.items.len(), 1);
    }

    #[test]
    fn edit_checks_duplicates_but_allows_same_item() {
        let mut l = list();
        l.add("A").unwrap();
        l.add("B").unwrap();
        assert!(l.edit(0, "b").is_err());
        assert!(l.edit(0, "a").is_ok());
        assert!(l.edit(0, "C").is_ok());
        assert_eq!(l.items[0].text, "C");
    }

    #[test]
    fn clear_and_search() {
        let mut l = list();
        l.add("Run a marathon").unwrap();
        l.add("See the aurora").unwrap();
        l.toggle(0);
        assert_eq!(l.search("MARATHON"), vec![0]);
        assert_eq!(l.clear_completed(), 1);
        assert_eq!(l.items.len(), 1);
    }

    #[test]
    fn progress_bar_bounds() {
        assert_eq!(progress_bar(0, 0, 4), "[----]");
        assert_eq!(progress_bar(1, 2, 4), "[##--]");
        assert_eq!(progress_bar(2, 2, 4), "[####]");
    }

    #[test]
    fn save_and_load_roundtrip() {
        let path = env::temp_dir().join(format!("bucketlist_test_{}.txt", process::id()));
        let mut l = BucketList::new(path.clone());
        l.add("One").unwrap();
        l.add("Two").unwrap();
        l.toggle(1);
        l.save().unwrap();
        let loaded = BucketList::load(path.clone()).unwrap();
        assert_eq!(loaded.items, l.items);
        fs::remove_file(path).ok();
    }
}
