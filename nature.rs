use std::env;
use std::fs;
use std::path::Path;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";
const MAGENTA: &str = "\x1b[35m";

#[derive(Debug)]
struct EntryInfo {
    name: String,
    kind: EntryKind,
    size: u64,
    depth: usize,
    is_hidden: bool,
    is_symlink: bool,
    extension: Option<String>,
}

#[derive(Debug, PartialEq)]
enum EntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

struct Stats {
    files: usize,
    dirs: usize,
    symlinks: usize,
    total_size: u64,
    hidden: usize,
}

impl Stats {
    fn new() -> Self {
        Stats { files: 0, dirs: 0, symlinks: 0, total_size: 0, hidden: 0 }
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut size = bytes as f64;
    let mut unit_index = 0;
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn kind_label(kind: &EntryKind) -> (&'static str, &'static str) {
    match kind {
        EntryKind::Directory => ("DIR ", BLUE),
        EntryKind::File      => ("FILE", GREEN),
        EntryKind::Symlink   => ("LINK", CYAN),
        EntryKind::Other     => ("???", YELLOW),
    }
}

fn file_icon(entry: &EntryInfo) -> &'static str {
    match &entry.kind {
        EntryKind::Directory => "▶",
        EntryKind::Symlink   => "⇒",
        EntryKind::Other     => "·",
        EntryKind::File => {
            match entry.extension.as_deref() {
                Some("rs")                          => "🦀",
                Some("py")                          => "🐍",
                Some("js") | Some("ts")             => "⚡",
                Some("html") | Some("htm")          => "🌐",
                Some("css")                         => "🎨",
                Some("json") | Some("toml") | Some("yaml") | Some("yml") => "⚙",
                Some("md") | Some("txt")            => "📄",
                Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => "🖼",
                Some("mp4") | Some("mkv") | Some("avi") | Some("mov") => "🎬",
                Some("mp3") | Some("wav") | Some("flac")               => "🎵",
                Some("zip") | Some("tar") | Some("gz") | Some("xz") | Some("bz2") => "📦",
                Some("pdf")                         => "📕",
                Some("sh") | Some("bash") | Some("zsh") => "🔧",
                Some("lock")                        => "🔒",
                _                                   => "·",
            }
        }
    }
}

fn collect_entries(path: &Path, depth: usize, max_depth: usize, show_hidden: bool) -> Vec<EntryInfo> {
    let mut entries = Vec::new();
    let dir = match fs::read_dir(path) {
        Ok(d) => d,
        Err(_) => return entries,
    };

    let mut raw: Vec<_> = dir.filter_map(|e| e.ok()).collect();
    raw.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    for entry in raw {
        let entry_path = entry.path();
        let name = entry.file_name().into_string().unwrap_or_else(|_| "<invalid>".to_string());
        let is_hidden = name.starts_with('.');

        if is_hidden && !show_hidden {
            continue;
        }

        let symlink_meta = fs::symlink_metadata(&entry_path);
        let is_symlink = symlink_meta.as_ref().map(|m| m.file_type().is_symlink()).unwrap_or(false);

        let meta = entry_path.metadata();
        let (kind, size) = match &meta {
            Ok(m) if is_symlink             => (EntryKind::Symlink, m.len()),
            Ok(m) if m.is_dir()             => (EntryKind::Directory, 0),
            Ok(m) if m.is_file()            => (EntryKind::File, m.len()),
            Ok(_)                           => (EntryKind::Other, 0),
            Err(_)                          => (EntryKind::Other, 0),
        };

        let extension = entry_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        let info = EntryInfo {
            name: name.clone(),
            kind,
            size,
            depth,
            is_hidden,
            is_symlink,
            extension,
        };

        let is_dir = info.kind == EntryKind::Directory;
        entries.push(info);

        if is_dir && depth < max_depth {
            let mut children = collect_entries(&entry_path, depth + 1, max_depth, show_hidden);
            entries.append(&mut children);
        }
    }

    entries
}

fn print_entry(entry: &EntryInfo) {
    let indent = if entry.depth == 0 {
        String::new()
    } else {
        format!("{}{}", "  │ ".repeat(entry.depth - 1), "  ├─")
    };

    let icon = file_icon(entry);
    let (label, color) = kind_label(&entry.kind);

    let size_str = if entry.kind == EntryKind::File {
        format!("{DIM}{:>10}{RESET}", format_size(entry.size))
    } else {
        format!("{:>10}", "")
    };

    let hidden_marker = if entry.is_hidden { format!("{DIM} [hidden]{RESET}") } else { String::new() };
    let sym_marker   = if entry.is_symlink { format!("{CYAN} → symlink{RESET}") } else { String::new() };

    let name_color = match entry.kind {
        EntryKind::Directory => format!("{BOLD}{BLUE}"),
        EntryKind::Symlink   => format!("{CYAN}"),
        EntryKind::File      => format!("{WHITE}"),
        EntryKind::Other     => format!("{YELLOW}"),
    };

    println!(
        "{}{}  {color}[{label}]{RESET}  {}  {name_color}{}{RESET}{}{}",
        indent,
        icon,
        size_str,
        entry.name,
        hidden_marker,
        sym_marker,
    );
}

fn print_stats(stats: &Stats) {
    println!();
    println!("{BOLD}{WHITE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{RESET}");
    println!(
        "  {BOLD}Summary{RESET}  {BLUE}{} dirs{RESET}  ·  {GREEN}{} files{RESET}  ·  {CYAN}{} symlinks{RESET}",
        stats.dirs, stats.files, stats.symlinks
    );
    if stats.hidden > 0 {
        println!("  {DIM}{} hidden entries (use -a to show){RESET}", stats.hidden);
    }
    println!("  {MAGENTA}Total size: {}{RESET}", format_size(stats.total_size));
    println!("{BOLD}{WHITE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{RESET}");
}

fn usage_and_exit() -> ! {
    eprintln!("{BOLD}Usage:{RESET}  nature <folder_path> [options]");
    eprintln!();
    eprintln!("{BOLD}Options:{RESET}");
    eprintln!("  {CYAN}-a{RESET}           Show hidden files and directories");
    eprintln!("  {CYAN}-d <depth>{RESET}   Max recursion depth (default: 0 = top-level only, -1 = unlimited)");
    eprintln!();
    eprintln!("{BOLD}Examples:{RESET}");
    eprintln!("  nature /home/user");
    eprintln!("  nature /home/user -a");
    eprintln!("  nature /home/user -d 2");
    eprintln!("  nature /home/user -a -d 3");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage_and_exit();
    }

    let folder_path = &args[1];
    let mut show_hidden = false;
    let mut max_depth: usize = 0;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-a" => {
                show_hidden = true;
            }
            "-d" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("{RED}Error: -d requires a depth argument{RESET}");
                    usage_and_exit();
                }
                match args[i].parse::<i64>() {
                    Ok(n) if n < 0 => max_depth = usize::MAX,
                    Ok(n)          => max_depth = n as usize,
                    Err(_) => {
                        eprintln!("{RED}Error: invalid depth '{}'{RESET}", args[i]);
                        usage_and_exit();
                    }
                }
            }
            other => {
                eprintln!("{RED}Error: unknown option '{other}'{RESET}");
                usage_and_exit();
            }
        }
        i += 1;
    }

    let path = Path::new(folder_path);

    if !path.exists() {
        eprintln!("{RED}Error: Path does not exist: {folder_path}{RESET}");
        std::process::exit(1);
    }
    if !path.is_dir() {
        eprintln!("{RED}Error: Path is not a directory: {folder_path}{RESET}");
        std::process::exit(1);
    }

    println!();
    println!("{BOLD}{WHITE}  📁  {CYAN}{folder_path}{RESET}");
    println!("{DIM}  Depth: {}   Hidden: {}{RESET}", if max_depth == usize::MAX { "unlimited".to_string() } else { max_depth.to_string() }, show_hidden);
    println!("{BOLD}{WHITE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{RESET}");
    println!();

    let entries = collect_entries(path, 0, max_depth, show_hidden);

    if entries.is_empty() {
        println!("{DIM}  (empty directory){RESET}");
    }

    let mut stats = Stats::new();

    for entry in &entries {
        print_entry(entry);
        match entry.kind {
            EntryKind::Directory => stats.dirs += 1,
            EntryKind::File      => { stats.files += 1; stats.total_size += entry.size; }
            EntryKind::Symlink   => stats.symlinks += 1,
            EntryKind::Other     => {}
        }
        if entry.is_hidden {
            stats.hidden += 1;
        }
    }

    print_stats(&stats);
    println!();
}
