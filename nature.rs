use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Read command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: nature <folder_path>");
        std::process::exit(1);
    }

    let folder_path = &args[1];
    let path = Path::new(folder_path);

    if !path.exists() {
        eprintln!("Error: Path does not exist.");
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("Error: Path is not a directory.");
        std::process::exit(1);
    }

    println!("Inspecting folder: {}\n", folder_path);

    let entries = fs::read_dir(path).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let entry_path = entry.path();
        let name = entry.file_name().into_string().unwrap_or_else(|_| "<invalid name>".to_string());

        let nature = match entry_path.metadata() {
            Ok(metadata) => {
                if metadata.is_dir() {
                    "Directory"
                } else if metadata.is_file() {
                    "File"
                } else {
                    "Other"
                }
            }
            Err(_) => "Unknown",
        };

        println!("{:<30} {}", name, nature);
    }
}
