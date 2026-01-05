use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get directory path from command-line arguments
    let args: Vec<String> = env::args().collect();
    let dir_path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    let path = Path::new(dir_path);

    // Ensure the path exists and is a directory
    if !path.exists() {
        eprintln!("Error: Path does not exist.");
        return;
    }

    if !path.is_dir() {
        eprintln!("Error: Path is not a directory.");
        return;
    }

    // Read and list directory entries
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        if let Some(name) = e.file_name().to_str() {
                            println!("{}", name);
                        }
                    }
                    Err(err) => eprintln!("Error reading entry: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Error reading directory: {}", err),
    }
}
