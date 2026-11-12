
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

struct Stats {
    size: u64,
    files: u64,
    folders: u64,
    largest: String,
    largest_size: u64,
    extensions: HashMap<String, (u64, u64)>,
}

impl Stats {
    fn new() -> Self {
        Self {
            size: 0,
            files: 0,
            folders: 0,
            largest: String::new(),
            largest_size: 0,
            extensions: HashMap::new(),
        }
    }
}

fn scan(path: &Path, stats: &mut Stats) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            stats.folders += 1;
            scan(&path, stats);
        } else if let Ok(metadata) = fs::metadata(&path) {
            let size = metadata.len();
            stats.files += 1;
            stats.size += size;

            if size > stats.largest_size {
                stats.largest_size = size;
                stats.largest = path.display().to_string();
            }

            let ext = path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("other")
                .to_string();

            let item = stats.extensions.entry(ext).or_insert((0, 0));
            item.0 += size;
            item.1 += 1;
        }
    }
}

fn print_tree(path: &Path, prefix: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|e| e.file_name());

        let total = entries.len();

        for (i, entry) in entries.iter().enumerate() {
            let last = i == total - 1;
            let name = entry.file_name().to_string_lossy().to_string();

            println!(
                "{}{}{}",
                prefix,
                if last { "└── " } else { "├── " },
                name
            );

            if entry.path().is_dir() {
                let next = format!(
                    "{}{}",
                    prefix,
                    if last { "    " } else { "│   " }
                );

                print_tree(&entry.path(), &next);
            }
        }
    }
}

fn format_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut i = 0;

    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }

    format!("{:.2} {}", size, units[i])
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <folder_path>");
        return;
    }

    let path = Path::new(&args[1]);

    if !path.is_dir() {
        println!("Invalid folder path");
        return;
    }

    let mut stats = Stats::new();

    scan(path, &mut stats);

    println!("\nFolder Analysis");
    println!("----------------------------");
    println!("Folder       : {}", path.display());
    println!("Total Size   : {}", format_size(stats.size));
    println!("Total Files  : {}", stats.files);
    println!("Directories  : {}", stats.folders);

    if stats.files > 0 {
        println!(
            "Average Size : {}",
            format_size(stats.size / stats.files)
        );
    }

    if stats.largest_size > 0 {
        println!("Largest File : {}", stats.largest);
        println!("Largest Size : {}", format_size(stats.largest_size));
    }

    println!("\nFile Types");
    println!("----------------------------");

    let mut extensions: Vec<_> = stats.extensions.iter().collect();
    extensions.sort_by(|a, b| b.1.0.cmp(&a.1.0));

    for (ext, (size, count)) in extensions {
        println!(
            ".{} : {} files, {}",
            ext,
            count,
            format_size(*size)
        );
    }

    println!("\nDirectory Tree");
    println!("----------------------------");
    println!("{}/", path.display());
    print_tree(path, "");
}
