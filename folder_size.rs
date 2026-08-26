use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ---------- Color handling ----------

struct Palette {
    enabled: bool,
}

impl Palette {
    const RESET: &'static str = "\x1b[0m";
    const BOLD: &'static str = "\x1b[1m";
    const DIM: &'static str = "\x1b[2m";
    const CYAN: &'static str = "\x1b[36m";
    const GREEN: &'static str = "\x1b[32m";
    const YELLOW: &'static str = "\x1b[33m";
    const RED: &'static str = "\x1b[31m";
    const BLUE: &'static str = "\x1b[34m";

    fn wrap(&self, code: &str, text: &str) -> String {
        if self.enabled {
            format!("{}{}{}", code, text, Self::RESET)
        } else {
            text.to_string()
        }
    }
    fn bold(&self, text: &str) -> String { self.wrap(Self::BOLD, text) }
    fn dim(&self, text: &str) -> String { self.wrap(Self::DIM, text) }
    fn cyan(&self, text: &str) -> String { self.wrap(Self::CYAN, text) }
    fn green(&self, text: &str) -> String { self.wrap(Self::GREEN, text) }
    fn yellow(&self, text: &str) -> String { self.wrap(Self::YELLOW, text) }
    fn red(&self, text: &str) -> String { self.wrap(Self::RED, text) }
    fn blue(&self, text: &str) -> String { self.wrap(Self::BLUE, text) }
}

// ---------- Stats ----------

#[derive(Clone)]
struct FileRecord {
    path: String,
    size: u64,
    modified_secs: Option<u64>,
}

struct FolderStats {
    total_size: u64,
    file_count: u64,
    dir_count: u64,
    empty_dir_count: u64,
    largest_file: Option<FileRecord>,
    oldest_file: Option<FileRecord>,
    newest_file: Option<FileRecord>,
    extension_sizes: HashMap<String, (u64, u64)>, // ext -> (total_size, count)
    errors: Vec<String>,
    skipped_hidden: u64,
    skipped_excluded: u64,
    skipped_symlinks: u64,
}

impl FolderStats {
    fn new() -> Self {
        FolderStats {
            total_size: 0,
            file_count: 0,
            dir_count: 0,
            empty_dir_count: 0,
            largest_file: None,
            oldest_file: None,
            newest_file: None,
            extension_sizes: HashMap::new(),
            errors: Vec::new(),
            skipped_hidden: 0,
            skipped_excluded: 0,
            skipped_symlinks: 0,
        }
    }

    fn average_file_size(&self) -> f64 {
        if self.file_count == 0 {
            0.0
        } else {
            self.total_size as f64 / self.file_count as f64
        }
    }
}

struct ScanOptions {
    max_depth: Option<usize>,
    include_hidden: bool,
    excludes: Vec<String>,
}

fn is_hidden(name: &str) -> bool {
    name.starts_with('.') && name != "." && name != ".."
}

/// Very small glob matcher supporting `*` as a multi-character wildcard.
/// Matching is done against the path's string representation (case-sensitive).
fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let (mut pi, mut ti) = (0usize, 0usize);
    let (mut star_idx, mut match_idx) = (None, 0usize);

    while ti < t.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star_idx = Some(pi);
            match_idx = ti;
            pi += 1;
        } else if let Some(si) = star_idx {
            pi = si + 1;
            match_idx += 1;
            ti = match_idx;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }
    pi == p.len()
}

fn is_excluded(path: &Path, excludes: &[String]) -> bool {
    if excludes.is_empty() {
        return false;
    }
    let path_str = path.to_string_lossy();
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    excludes
        .iter()
        .any(|pat| glob_match(pat, &path_str) || glob_match(pat, &name))
}

/// Iterative (stack-based) traversal — avoids blowing the call stack on very
/// deep directory trees, and lets us track per-directory emptiness.
fn get_folder_stats(root: &Path, stats: &mut FolderStats, opts: &ScanOptions) {
    let mut stack: Vec<(PathBuf, usize)> = vec![(root.to_path_buf(), 0)];

    while let Some((dir, depth)) = stack.pop() {
        if let Some(max) = opts.max_depth {
            if depth > max {
                continue;
            }
        }

        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                stats.errors.push(format!("{}: {}", dir.display(), e));
                continue;
            }
        };

        let mut saw_any = false;

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    stats.errors.push(format!("Entry error: {}", e));
                    continue;
                }
            };

            let entry_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if !opts.include_hidden && is_hidden(&file_name) {
                stats.skipped_hidden += 1;
                continue;
            }

            if is_excluded(&entry_path, &opts.excludes) {
                stats.skipped_excluded += 1;
                continue;
            }

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(e) => {
                    stats.errors.push(format!("{}: {}", entry_path.display(), e));
                    continue;
                }
            };

            if metadata.is_symlink() {
                stats.skipped_symlinks += 1;
                continue;
            }

            saw_any = true;

            if entry_path.is_dir() {
                stats.dir_count += 1;
                stack.push((entry_path, depth + 1));
            } else {
                let file_size = metadata.len();
                stats.total_size += file_size;
                stats.file_count += 1;

                let ext = entry_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("no extension")
                    .to_lowercase();

                let bucket = stats.extension_sizes.entry(ext).or_insert((0, 0));
                bucket.0 += file_size;
                bucket.1 += 1;

                let modified_secs = metadata
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map(|d| d.as_secs());

                let record = FileRecord {
                    path: entry_path.display().to_string(),
                    size: file_size,
                    modified_secs,
                };

                match &stats.largest_file {
                    None => stats.largest_file = Some(record.clone()),
                    Some(cur) if file_size > cur.size => stats.largest_file = Some(record.clone()),
                    _ => {}
                }

                if let Some(m) = modified_secs {
                    match &stats.oldest_file {
                        None => stats.oldest_file = Some(record.clone()),
                        Some(cur) if cur.modified_secs.map_or(true, |c| m < c) => {
                            stats.oldest_file = Some(record.clone())
                        }
                        _ => {}
                    }
                    match &stats.newest_file {
                        None => stats.newest_file = Some(record.clone()),
                        Some(cur) if cur.modified_secs.map_or(true, |c| m > c) => {
                            stats.newest_file = Some(record.clone())
                        }
                        _ => {}
                    }
                }
            }
        }

        if !saw_any {
            stats.empty_dir_count += 1;
        }
    }
}

// ---------- Formatting helpers ----------

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

fn format_count(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::new();
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(*b as char);
    }
    out
}

fn format_age(modified_secs: Option<u64>) -> String {
    let m = match modified_secs {
        Some(m) => m,
        None => return "unknown".to_string(),
    };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(m);
    let days = now.saturating_sub(m) / 86400;
    if days == 0 {
        "today".to_string()
    } else if days == 1 {
        "1 day ago".to_string()
    } else {
        format!("{} days ago", days)
    }
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

// ---------- Tree printing ----------

fn print_tree(path: &Path, prefix: &str, depth: usize, max_depth: usize, include_hidden: bool, pal: &Palette) {
    if depth > max_depth {
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => {
            let mut v: Vec<_> = e
                .filter_map(|e| e.ok())
                .filter(|e| {
                    include_hidden || !is_hidden(&e.file_name().to_string_lossy())
                })
                .collect();
            v.sort_by(|a, b| {
                let a_dir = a.path().is_dir();
                let b_dir = b.path().is_dir();
                b_dir.cmp(&a_dir).then_with(|| a.file_name().cmp(&b.file_name()))
            });
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

        let is_dir = metadata.is_dir();
        let raw_name = entry.file_name().to_string_lossy().to_string();
        let name = if is_dir {
            pal.blue(&format!("{}/", raw_name))
        } else {
            raw_name.clone()
        };

        let size_str = if metadata.is_file() {
            pal.dim(&format!(" ({})", format_size(metadata.len())))
        } else {
            String::new()
        };

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| pal.dim(&format!(" [{}]", format_age(Some(d.as_secs())))))
            .unwrap_or_default();

        println!("{}{}{}{}{}", prefix, connector, name, size_str, modified);

        if is_dir {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_tree(&entry.path(), &new_prefix, depth + 1, max_depth, include_hidden, pal);
        }
    }
}

fn print_separator(pal: &Palette) {
    println!("{}", pal.dim(&"─".repeat(56)));
}

// ---------- Output ----------

fn print_pretty(
    stats: &FolderStats,
    folder_path: &str,
    top_n: usize,
    sort_by_count: bool,
    show_tree: bool,
    tree_path: &Path,
    tree_depth: usize,
    include_hidden: bool,
    elapsed_ms: u128,
    pal: &Palette,
) {
    print_separator(pal);
    println!("  {} {}", pal.bold("📁 Folder Analysis:"), pal.cyan(folder_path));
    print_separator(pal);
    println!("  Total Size    : {}", pal.bold(&format_size(stats.total_size)));
    println!("  Files         : {}", format_count(stats.file_count));
    println!("  Directories   : {}", format_count(stats.dir_count));
    println!("  Empty Dirs    : {}", format_count(stats.empty_dir_count));
    if stats.file_count > 0 {
        println!("  Avg File Size : {}", format_size(stats.average_file_size() as u64));
    }

    if stats.skipped_hidden > 0 || stats.skipped_excluded > 0 || stats.skipped_symlinks > 0 {
        println!(
            "  {}",
            pal.dim(&format!(
                "Skipped: {} hidden, {} excluded, {} symlinks",
                stats.skipped_hidden, stats.skipped_excluded, stats.skipped_symlinks
            ))
        );
    }

    if let Some(f) = &stats.largest_file {
        print_separator(pal);
        println!("  {}", pal.bold("Largest File"));
        println!("    {}", f.path);
        println!("    {}", pal.yellow(&format_size(f.size)));
    }

    if let (Some(oldest), Some(newest)) = (&stats.oldest_file, &stats.newest_file) {
        print_separator(pal);
        println!("  {}", pal.bold("File Age Range"));
        println!("    Oldest : {}  {}", oldest.path, pal.dim(&format!("({})", format_age(oldest.modified_secs))));
        println!("    Newest : {}  {}", newest.path, pal.dim(&format!("({})", format_age(newest.modified_secs))));
    }

    if !stats.extension_sizes.is_empty() {
        print_separator(pal);
        let sort_label = if sort_by_count { "Count" } else { "Size" };
        println!("  {}", pal.bold(&format!("Top {} File Types by {}:", top_n, sort_label)));
        println!();

        let mut ext_list: Vec<(&String, &(u64, u64))> = stats.extension_sizes.iter().collect();
        if sort_by_count {
            ext_list.sort_by(|a, b| b.1.1.cmp(&a.1.1));
        } else {
            ext_list.sort_by(|a, b| b.1.0.cmp(&a.1.0));
        }

        for (ext, (size, count)) in ext_list.iter().take(top_n) {
            let pct = if stats.total_size > 0 {
                (*size as f64 / stats.total_size as f64) * 100.0
            } else {
                0.0
            };
            let bar_len = (pct / 2.0) as usize;
            let bar = pal.green(&"█".repeat(bar_len));
            println!(
                "  .{:<12} {:>10}  {:>7} files  {:>5.1}%  {}",
                ext,
                format_size(*size),
                format_count(*count),
                pct,
                bar
            );
        }

        let shown: u64 = ext_list.iter().take(top_n).map(|(_, (_, c))| c).sum();
        let remaining_types = ext_list.len().saturating_sub(top_n);
        if remaining_types > 0 {
            let remaining_count = stats.file_count - shown;
            println!(
                "  {}",
                pal.dim(&format!(
                    "... and {} more type(s) ({} files)",
                    remaining_types, remaining_count
                ))
            );
        }
    }

    if !stats.errors.is_empty() {
        print_separator(pal);
        println!("  {}", pal.red(&format!("⚠ Errors ({}):", stats.errors.len())));
        for err in &stats.errors {
            println!("    - {}", err);
        }
    }

    if show_tree {
        print_separator(pal);
        println!("  {}", pal.bold(&format!("Directory Tree (depth ≤ {}):", tree_depth)));
        println!();
        println!("{}", pal.blue(&format!("{}/", tree_path.display())));
        print_tree(tree_path, "", 0, tree_depth, include_hidden, pal);
    }

    print_separator(pal);
    println!("  {}", pal.dim(&format!("Scanned in {} ms", elapsed_ms)));
    print_separator(pal);
}

fn print_json(stats: &FolderStats, folder_path: &str, elapsed_ms: u128) {
    let mut ext_entries = Vec::new();
    for (ext, (size, count)) in &stats.extension_sizes {
        ext_entries.push(format!(
            "{{\"extension\":\"{}\",\"size_bytes\":{},\"count\":{}}}",
            json_escape(ext),
            size,
            count
        ));
    }

    let largest = stats
        .largest_file
        .as_ref()
        .map(|f| format!("{{\"path\":\"{}\",\"size_bytes\":{}}}", json_escape(&f.path), f.size))
        .unwrap_or_else(|| "null".to_string());

    let oldest = stats
        .oldest_file
        .as_ref()
        .map(|f| {
            format!(
                "{{\"path\":\"{}\",\"modified_epoch\":{}}}",
                json_escape(&f.path),
                f.modified_secs.unwrap_or(0)
            )
        })
        .unwrap_or_else(|| "null".to_string());

    let newest = stats
        .newest_file
        .as_ref()
        .map(|f| {
            format!(
                "{{\"path\":\"{}\",\"modified_epoch\":{}}}",
                json_escape(&f.path),
                f.modified_secs.unwrap_or(0)
            )
        })
        .unwrap_or_else(|| "null".to_string());

    let errors: Vec<String> = stats.errors.iter().map(|e| format!("\"{}\"", json_escape(e))).collect();

    println!(
        "{{\"folder\":\"{}\",\"total_size_bytes\":{},\"file_count\":{},\"dir_count\":{},\"empty_dir_count\":{},\"average_file_size_bytes\":{:.2},\"largest_file\":{},\"oldest_file\":{},\"newest_file\":{},\"extensions\":[{}],\"errors\":[{}],\"skipped_hidden\":{},\"skipped_excluded\":{},\"skipped_symlinks\":{},\"scan_ms\":{}}}",
        json_escape(folder_path),
        stats.total_size,
        stats.file_count,
        stats.dir_count,
        stats.empty_dir_count,
        stats.average_file_size(),
        largest,
        oldest,
        newest,
        ext_entries.join(","),
        errors.join(","),
        stats.skipped_hidden,
        stats.skipped_excluded,
        stats.skipped_symlinks,
        elapsed_ms
    );
}

fn print_help(prog: &str) {
    println!("Usage: {} <folder_path> [OPTIONS]", prog);
    println!();
    println!("Options:");
    println!("  --tree,        -t              Show directory tree");
    println!("  --depth,       -d <n>          Max scan depth (default: unlimited)");
    println!("  --tree-depth,  -td <n>         Max tree display depth (default: 2)");
    println!("  --top,         -n <n>          Show top N extensions (default: 5)");
    println!("  --sort-count                   Rank extensions by file count instead of size");
    println!("  --exclude <pattern>            Exclude paths matching a glob (*, ?); repeatable");
    println!("  --hidden                       Include hidden files/directories (dotfiles)");
    println!("  --json                         Output machine-readable JSON instead of text");
    println!("  --no-color                     Disable ANSI colors");
    println!("  --help,        -h              Show this help message");
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let mut folder_path = String::new();
    let mut show_tree = false;
    let mut max_depth: Option<usize> = None;
    let mut tree_depth: usize = 2;
    let mut top_n: usize = 5;
    let mut sort_by_count = false;
    let mut excludes: Vec<String> = Vec::new();
    let mut include_hidden = false;
    let mut json_output = false;
    let mut no_color = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--tree" | "-t" => show_tree = true,
            "--depth" | "-d" => {
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
            "--top" | "-n" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    top_n = v.parse().unwrap_or(5);
                }
            }
            "--sort-count" => sort_by_count = true,
            "--exclude" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    excludes.push(v.clone());
                }
            }
            "--hidden" => include_hidden = true,
            "--json" => json_output = true,
            "--no-color" => no_color = true,
            "--help" | "-h" => {
                print_help(&args[0]);
                return;
            }
            other => {
                if other.starts_with('-') {
                    eprintln!("Warning: unrecognized option '{}'", other);
                } else {
                    folder_path = other.to_string();
                }
            }
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

    let pal = Palette {
        enabled: !no_color && !json_output && std::io::stdout().is_terminal(),
    };

    let opts = ScanOptions {
        max_depth,
        include_hidden,
        excludes,
    };

    let mut stats = FolderStats::new();
    get_folder_stats(path, &mut stats, &opts);

    let elapsed_ms = start.elapsed().as_millis();

    if json_output {
        print_json(&stats, &folder_path, elapsed_ms);
    } else {
        print_pretty(
            &stats,
            &folder_path,
            top_n,
            sort_by_count,
            show_tree,
            path,
            tree_depth,
            include_hidden,
            elapsed_ms,
            &pal,
        );
    }
}
