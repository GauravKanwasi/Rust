use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: opflod <folder_path>");
        std::process::exit(1);
    }

    let folder_path = PathBuf::from(&args[1]);

    // Ensure the path exists and is a directory
    if !folder_path.exists() || !folder_path.is_dir() {
        eprintln!("Error: Provided path is not a valid folder.");
        std::process::exit(1);
    }

    // Canonicalize to prevent path traversal
    let base_dir = fs::canonicalize(&folder_path)?;

    // Read directory entries
    for entry in fs::read_dir(&base_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only process regular files directly inside the folder
        if path.is_file() && is_within_base(&base_dir, &path)? {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("<unknown>");

            println!("--- File: {} ---", filename);

            match fs::read_to_string(&path) {
                Ok(contents) => println!("{}", contents),
                Err(err) => eprintln!("Could not read {}: {}", filename, err),
            }
        }
    }

    Ok(())
}

// Ensures the file
