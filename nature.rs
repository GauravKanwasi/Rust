use std::{env, fs, path::Path, process};

const R: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";

#[derive(Default)]
struct Stats {
    files: usize,
    dirs: usize,
    links: usize,
    hidden: usize,
    size: u64,
}

fn fmt_size(bytes: u64) -> String {
    let mut s = bytes as f64;
    let mut u = 0;
    while s >= 1024.0 && u < 4 {
        s /= 1024.0;
        u += 1;
    }
    if u == 0 {
        format!("{bytes} B")
    } else {
        format!("{s:.1} {}", ["B", "KB", "MB", "GB", "TB"][u])
    }
}

fn walk(dir: &Path, prefix: &str, depth: usize, max: usize, all: bool, st: &mut Stats) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut items: Vec<_> = rd
        .flatten()
        .filter_map(|e| e.file_type().ok().map(|t| (e, t)))
        .collect();
    items.sort_by_key(|(e, t)| (!t.is_dir(), e.file_name()));

    let mut shown = Vec::new();
    for item in items {
        if item.0.file_name().to_string_lossy().starts_with('.') {
            st.hidden += 1;
            if !all {
                continue;
            }
        }
        shown.push(item);
    }

    let n = shown.len();
    for (i, (e, t)) in shown.into_iter().enumerate() {
        let (conn, ext) = if i + 1 == n { ("└── ", "    ") } else { ("├── ", "│   ") };
        let name = e.file_name().to_string_lossy().into_owned();

        if t.is_dir() {
            st.dirs += 1;
            println!("{:>9}  {prefix}{conn}{BOLD}{BLUE}{name}/{R}", "");
            if depth < max {
                walk(&e.path(), &format!("{prefix}{ext}"), depth + 1, max, all, st);
            }
        } else if t.is_symlink() {
            st.links += 1;
            let target = fs::read_link(e.path()).map(|p| p.display().to_string()).unwrap_or_default();
            println!("{:>9}  {prefix}{conn}{CYAN}{name}{R} {DIM}→ {target}{R}", "");
        } else {
            let size = e.metadata().map(|m| m.len()).unwrap_or(0);
            st.files += 1;
            st.size += size;
            println!("{DIM}{:>9}{R}  {prefix}{conn}{name}", fmt_size(size));
        }
    }
}

fn usage() -> ! {
    eprintln!("Usage: nature <folder> [-a] [-d <depth>]");
    eprintln!("  -a          show hidden entries");
    eprintln!("  -d <depth>  recursion depth (default 0, -1 = unlimited)");
    process::exit(1);
}

fn main() {
    let mut args = env::args().skip(1);
    let root = args.next().unwrap_or_else(|| usage());
    let (mut all, mut max) = (false, 0usize);

    while let Some(a) = args.next() {
        match a.as_str() {
            "-a" => all = true,
            "-d" => {
                max = match args.next().and_then(|v| v.parse::<i64>().ok()) {
                    Some(n) if n < 0 => usize::MAX,
                    Some(n) => n as usize,
                    None => usage(),
                }
            }
            _ => usage(),
        }
    }

    let path = Path::new(&root);
    if !path.is_dir() {
        eprintln!("{RED}Error: not a directory: {root}{R}");
        process::exit(1);
    }

    println!("\n{BOLD}{CYAN}{root}{R}");
    let mut st = Stats::default();
    walk(path, "", 0, max, all, &mut st);

    print!(
        "\n{BOLD}{}{R} dirs, {BOLD}{}{R} files, {BOLD}{}{R} symlinks, {BOLD}{}{R} total",
        st.dirs, st.files, st.links, fmt_size(st.size)
    );
    if st.hidden > 0 && !all {
        print!(" {DIM}({} hidden, use -a){R}", st.hidden);
    }
    println!("\n");
}
