use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: opflod <folder_path> [--recursive]");
        std::process::exit(1);
    }

    let recursive = args.get(2).map(|s| s == "--recursive").unwrap_or(false);

    let input_path = PathBuf::from(&args[1]);

    if !input_path.exists() || !input_path.is_dir() {
        eprintln!("Error: invalid directory");
        std::process::exit(1);
    }

    let base = fs::canonicalize(&input_path)?;

    if recursive {
        walk_dir(&base, &base)?;
    } else {
        for entry in fs::read_dir(&base)? {
            let entry = entry?;
            let path = entry.path();
            process_file(&base, &path)?;
        }
    }

    Ok(())
}

fn walk_dir(base: &Path, dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir(base, &path)?;
        } else {
            process_file(base, &path)?;
        }
    }
    Ok(())
}

fn process_file(base: &Path, path: &Path) -> io::Result<()> {
    if !path.is_file() {
        return Ok(());
    }

    if !is_within_base(base, path)? {
        return Ok(());
    }

    if is_binary(path)? {
        return Ok(());
    }

    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("<unknown>");
    println!("--- File: {} ---", name);

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("open error {}: {}", name, e);
            return Ok(());
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => println!("{}", l),
            Err(e) => {
                eprintln!("read error {}: {}", name, e);
                break;
            }
        }
    }

    Ok(())
}

fn is_within_base(base: &Path, path: &Path) -> io::Result<bool> {
    let canonical = fs::canonicalize(path)?;
    Ok(canonical.starts_with(base))
}

fn is_binary(path: &Path) -> io::Result<bool> {
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(_) => return Ok(true),
    };

    Ok(data.iter().take(1024).any(|&b| b == 0))
}
