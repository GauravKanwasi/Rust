use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn build_path(parts: &[&str]) -> PathBuf {
    parts.iter().collect()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        build_path(&["data", "example.txt"])
    };

    match read_file(&file_path) {
        Ok(contents) => {
            println!("Path: {}\n", file_path.display());
            println!("{}", contents);
        }
        Err(e) => {
            eprintln!("Failed to read {}: {}", file_path.display(), e);
        }
    }

    Ok(())
}
