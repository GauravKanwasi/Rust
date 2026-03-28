use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::env;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    InvalidPath(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O error: {}", e),
            AppError::InvalidPath(p) => write!(f, "Invalid path: {}", p),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

fn create_dir_if_missing(path: &Path) -> Result<bool, AppError> {
    if path.as_os_str().is_empty() {
        return Err(AppError::InvalidPath("Path cannot be empty".to_string()));
    }

    match fs::metadata(path) {
        Ok(meta) if meta.is_dir() => Ok(false),
        Ok(_) => Err(AppError::InvalidPath(format!(
            "'{}' exists but is not a directory",
            path.display()
        ))),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            fs::create_dir_all(path)?;
            Ok(true)
        }
        Err(e) => Err(AppError::from(e)),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let targets: Vec<PathBuf> = if args.is_empty() {
        vec![PathBuf::from("output")]
    } else {
        args.iter().map(PathBuf::from).collect()
    };

    let mut has_error = false;

    for path in &targets {
        match create_dir_if_missing(path) {
            Ok(true) => println!("[created]  {}", path.display()),
            Ok(false) => println!("[exists]   {}", path.display()),
            Err(e) => {
                eprintln!("[error]    {}: {}", path.display(), e);
                has_error = true;
            }
        }
    }

    if has_error {
        std::process::exit(1);
    }
}
