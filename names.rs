use std::{env, fs, path::Path};

fn main() {
    let dir = env::args().nth(1).unwrap_or_else(|| ".".into());
    let path = Path::new(&dir);

    if !path.exists() {
        eprintln!("path does not exist");
        return;
    }

    if !path.is_dir() {
        eprintln!("path is not a directory");
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let mut files = entries
        .filter_map(Result::ok)
        .filter_map(|e| e.file_name().into_string().ok())
        .collect::<Vec<_>>();

    files.sort();

    for file in files {
        println!("{file}");
    }
}
