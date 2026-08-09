use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    InvalidPath(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::InvalidPath(path) => write!(f, "invalid path: {path}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

#[derive(Debug, Clone, Copy)]
enum DirStatus {
    Created,
    Exists,
}

fn validate_path(path: &Path) -> Result<(), AppError> {
    if path.as_os_str().is_empty() {
        return Err(AppError::InvalidPath(
            "path cannot be empty".to_string(),
        ));
    }

    Ok(())
}

fn create_dir_if_missing(path: &Path) -> Result<DirStatus, AppError> {
    validate_path(path)?;

    // Fast path: directory already exists.
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Ok(DirStatus::Exists);
            }

            return Err(AppError::InvalidPath(format!(
                "'{}' exists but is not a directory",
                path.display()
            )));
        }

        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            // Directory doesn't exist, so create it and all missing parents.
            fs::create_dir_all(path)?;

            // Verify the final path is actually a directory.
            let metadata = fs::metadata(path)?;

            if !metadata.is_dir() {
                return Err(AppError::InvalidPath(format!(
                    "'{}' was created but is not a directory",
                    path.display()
                )));
            }

            Ok(DirStatus::Created)
        }

        Err(err) => Err(AppError::Io(err)),
    }
}

fn print_usage(program: &str) {
    println!("Usage: {program} [DIRECTORY ...]");
    println!();
    println!("Create one or more directories if they don't already exist.");
    println!();
    println!("If no directories are provided, 'output' is created.");
}

fn main() -> ExitCode {
    let mut args = env::args_os();
    let program = args
        .next()
        .unwrap_or_else(|| "mkdir-safe".into());

    let targets: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if targets.iter().any(|path| path == Path::new("--help")) {
        print_usage(&program.to_string_lossy());
        return ExitCode::SUCCESS;
    }

    let targets = if targets.is_empty() {
        vec![PathBuf::from("output")]
    } else {
        targets
    };

    let mut created = 0usize;
    let mut existing = 0usize;
    let mut errors = 0usize;

    for path in &targets {
        match create_dir_if_missing(path) {
            Ok(DirStatus::Created) => {
                println!("[created] {}", path.display());
                created += 1;
            }

            Ok(DirStatus::Exists) => {
                println!("[exists]  {}", path.display());
                existing += 1;
            }

            Err(err) => {
                eprintln!("[error]   {}: {err}", path.display());
                errors += 1;
            }
        }
    }

    if errors > 0 {
        eprintln!(
            "\nCompleted with errors: {created} created, \
             {existing} already existed, {errors} failed."
        );
        ExitCode::FAILURE
    } else {
        println!(
            "\nDone: {created} created, {existing} already existed."
        );
        ExitCode::SUCCESS
    }
}
