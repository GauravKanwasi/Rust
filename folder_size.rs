use std::env;
use std::fs;
use std::path::Path;

fn get_folder_size(path: &Path) -> std::io::Result<u64> {
    let mut total_size = 0;
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            
            if entry_path.is_dir() {
                total_size += get_folder_size(&entry_path)?;
            } else {
                total_size += entry.metadata()?.len();
            }
        }
    }
    
    Ok(total_size)
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

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    }
    
    let folder_path = &args[1];
    let path = Path::new(folder_path);
    
    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist", folder_path);
        std::process::exit(1);
    }
    
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory", folder_path);
        std::process::exit(1);
    }
    
    match get_folder_size(path) {
        Ok(size) => {
            println!("Folder: {}", folder_path);
            println!("Total size: {} ({})", format_size(size), size);
        }
        Err(e) => {
            eprintln!("Error calculating folder size: {}", e);
            std::process::exit(1);
        }
    }
}
