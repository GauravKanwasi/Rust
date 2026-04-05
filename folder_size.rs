use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::time::UNIX_EPOCH;

struct FolderStats {
    total_size: u64,
    file_count: u64,
    dir_count: u64,
    largest_file: Option<(String, u64)>,
    extension_sizes: HashMap<String, (u64, u64)>, // ext -> (total_size, count)
    errors: Vec<String>,
}

impl FolderStats {
    fn new() -> Self {
        FolderStats {
            total_size: 0,
            file_count: 0,
            dir_count: 0,
            largest_file: None,
            extension_sizes: HashMap::new(),
            errors: Vec::new(),
        }
    }
}

fn get_folder_stats(path: &Path, stats: &mut FolderStats, depth: usize, max_depth: Option<usize>) {
    if let Some(max) = max_depth {
        if depth > max {
            return;
        }
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            stats.errors.push(format!("{}: {}", path.display(), e));
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                stats.errors.push(format!("Entry error: {}", e));
                continue;
            }
        };

        let entry_path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                stats.errors.push(format!("{}: {}", entry_path.display(), e));
                continue;
            }
        };

        if metadata.is_symlink() {
            continue;
        }

        if entry_path.is_dir() {
            stats.dir_count += 1;
            get_folder_stats(&entry_path, stats, depth + 1, max_depth);
        } else {
            let file_size = metadata.len();
            stats.total_size += file_size;
            stats.file_count += 1;

            let ext = entry_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("no extension")
                .to_lowercase();

            let entry = stats.extension_sizes.entry(ext).or_insert((0, 0));
            entry.0 += file_size;
            entry.1 += 1;

            match &stats.largest_file {
                None => stats.largest_file = Some((entry_path.display().to_string(), file_size)),
                Some((_, largest_size)) if file_size > *largest_size => {
                    stats.largest_file = Some((entry_path.display().to_string(), file_size));
                }
                _ => {}
            }
        }
    }
}

fn format_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

fn print_tree(path: &Path, prefix: &str, depth: usize, max_depth: usize) {
    if depth > max_depth {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => {
            let mut v: Vec<_> = e.filter_map(|e| e.ok()).collect();
            v.sort_by_key(|e| e.file_name());
            v
        }
        Err(_) => return,
    };

    let count = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let size_str = if metadata.is_file() {
            format!(" ({})", format_size(metadata.len()))
        } else {
            String::new()
        };

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| {
                let secs = d.as_secs();
                let days = secs / 86400;
                format!(" [{}d ago]", days)
            })
            .unwrap_or_default();

        println!(
            "{}{}{}{}{}", 
            prefix, connector,
            entry.file_name().to_string_lossy(),
            size_str,
            modified
        );

        if entry.path().is_dir() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_tree(&entry.path(), &new_prefix, depth + 1, max_depth);
        }
    }
}

fn print_separator() {
    println!("{}", "─".repeat(50));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut folder_path = String::new();
    let mut show_tree = false;
    let mut max_depth: Option<usize> = None;
    let mut tree_depth: usize = 2;
    let mut top_n: usize = 5;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--tree" | "-t"      => show_tree = true,
            "--depth" | "-d"     => {
                i += 1;
                if let Some(v) = args.get(i) {
                    max_depth = v.parse().ok();
                }
            }
            "--tree-depth" | "-td" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    tree_depth = v.parse().unwrap_or(2);
                }
            }
            "--top" | "-n"       => {
                i += 1;
                if let Some(v) = args.get(i) {
                    top_n = v.parse().unwrap_or(5);
                }
            }
            "--help" | "-h"      => {
                println!("Usage: {} <folder_path> [OPTIONS]", args[0]);
                println!();
                println!("Options:");
                println!("  --tree,       -t         Show directory tree");
                println!("  --depth,      -d <n>     Max scan depth (default: unlimited)");
                println!("  --tree-depth, -td <n>    Max tree display depth (default: 2)");
                println!("  --top,        -n <n>     Show top N extensions (default: 5)");
                println!("  --help,       -h         Show this help message");
                return;
            }
            _                    => folder_path = args[i].clone(),
        }
        i += 1;
    }

    if folder_path.is_empty() {
        eprintln!("Usage: {} <folder_path> [OPTIONS]", args[0]);
        eprintln!("Run with --help for more information.");
        std::process::exit(1);
    }

    let path = Path::new(&folder_path);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist", folder_path);
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory", folder_path);
        std::process::exit(1);
    }

    let mut stats = FolderStats::new();
    get_folder_stats(path, &mut stats, 0, max_depth);

    print_separator();
    println!("  📁 Folder Analysis: {}", folder_path);
    print_separator();
    println!("  Total Size   : {} ({} bytes)", format_size(stats.total_size), stats.total_size);
    println!("  Files        : {}", stats.file_count);
    println!("  Directories  : {}", stats.dir_count);

    if let Some((ref name, size)) = stats.largest_file {
        print_separator();
        println!("  Largest File : {}", name);
        println!("  Size         : {}", format_size(size));
    }

    if !stats.extension_sizes.is_empty() {
        print_separator();
        println!("  Top {} File Types by Size:", top_n);
        println!();

        let mut ext_list: Vec<(&String, &(u64, u64))> = stats.extension_sizes.iter().collect();
        ext_list.sort_by(|a, b| b.1.0.cmp(&a.1.0));

        for (ext, (size, count)) in ext_list.iter().take(top_n) {
            let pct = if stats.total_size > 0 {
                (**size as f64 / stats.total_size as f64) * 100.0
            } else {
                0.0
            };
            let bar_len = (pct / 2.0) as usize;
            let bar = "█".repeat(bar_len);
            println!(
                "  .{:<12} {:>8}  {:>6} files  {:>5.1}%  {}",
                ext,
                format_size(**size),
                count,
                pct,
                bar
            );
        }
    }

    if !stats.errors.is_empty() {
        print_separator();
        println!("  ⚠ Errors ({}):", stats.errors.len());
        for err in &stats.errors {
            println!("    - {}", err);
        }
    }

    if show_tree {
        print_separator();
        println!("  Directory Tree (depth ≤ {}):", tree_depth);
        println!();
        println!("{}", path.display());
        print_tree(path, "", 0, tree_depth);
    }

    print_separator();
}
