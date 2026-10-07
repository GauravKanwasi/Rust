use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const DEFAULT_DIR: &str = "output";
const VERSION: &str = match option_env!("CARGO_PKG_VERSION") {
    Some(v) => v,
    None => "0.1.0",
};

#[derive(Debug)]
enum AppError {
    EmptyPath,
    NotADirectory(PathBuf),
    Io { path: PathBuf, source: io::Error },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => write!(f, "path cannot be empty"),
            Self::NotADirectory(path) => {
                write!(f, "'{}' exists but is not a directory", path.display())
            }
            Self::Io { source, .. } => write!(f, "{source}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl AppError {
    fn io(path: &Path, source: io::Error) -> Self {
        Self::Io {
            path: path.to_path_buf(),
            source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirStatus {
    Created,
    Exists,
    WouldCreate,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Options {
    dry_run: bool,
    quiet: bool,
    targets: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Help,
    Version,
    Run(Options),
}

#[derive(Debug, Default)]
struct Summary {
    created: usize,
    existing: usize,
    failed: usize,
}

fn ensure_dir(path: &Path, dry_run: bool) -> Result<DirStatus, AppError> {
    if path.as_os_str().is_empty() {
        return Err(AppError::EmptyPath);
    }

    match fs::metadata(path) {
        Ok(meta) if meta.is_dir() => return Ok(DirStatus::Exists),
        Ok(_) => return Err(AppError::NotADirectory(path.to_path_buf())),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {}
        Err(err) => return Err(AppError::io(path, err)),
    }

    if dry_run {
        return Ok(DirStatus::WouldCreate);
    }

    fs::create_dir_all(path).map_err(|err| AppError::io(path, err))?;
    Ok(DirStatus::Created)
}

fn parse_args<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = OsString>,
{
    let mut options = Options::default();
    let mut only_paths = false;

    for arg in args {
        if !only_paths {
            match arg.to_str() {
                Some("--") => {
                    only_paths = true;
                    continue;
                }
                Some("-h" | "--help") => return Ok(Command::Help),
                Some("-V" | "--version") => return Ok(Command::Version),
                Some("-n" | "--dry-run") => {
                    options.dry_run = true;
                    continue;
                }
                Some("-q" | "--quiet") => {
                    options.quiet = true;
                    continue;
                }
                Some(flag) if flag.starts_with('-') && flag.len() > 1 => {
                    return Err(format!("unknown option '{flag}'"));
                }
                _ => {}
            }
        }

        let path = PathBuf::from(arg);
        if !options.targets.contains(&path) {
            options.targets.push(path);
        }
    }

    if options.targets.is_empty() {
        options.targets.push(PathBuf::from(DEFAULT_DIR));
    }

    Ok(Command::Run(options))
}

fn print_usage(program: &str) {
    println!("Usage: {program} [OPTIONS] [--] [DIRECTORY ...]");
    println!();
    println!("Create one or more directories (and any missing parents) if they");
    println!("don't already exist. If none are given, '{DEFAULT_DIR}' is used.");
    println!();
    println!("Options:");
    println!("  -n, --dry-run   Show what would be created without creating anything");
    println!("  -q, --quiet     Only print errors");
    println!("  -h, --help      Show this help");
    println!("  -V, --version   Show version");
}

fn run(options: &Options) -> Summary {
    let mut summary = Summary::default();

    for path in &options.targets {
        match ensure_dir(path, options.dry_run) {
            Ok(status) => {
                let label = match status {
                    DirStatus::Created => {
                        summary.created += 1;
                        "created"
                    }
                    DirStatus::WouldCreate => {
                        summary.created += 1;
                        "would create"
                    }
                    DirStatus::Exists => {
                        summary.existing += 1;
                        "exists"
                    }
                };
                if !options.quiet {
                    println!("[{label}] {}", path.display());
                }
            }
            Err(err) => {
                eprintln!("[error] {}: {err}", path.display());
                summary.failed += 1;
            }
        }
    }

    summary
}

fn main() -> ExitCode {
    let mut args = env::args_os();
    let program = args
        .next()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "mkdir-safe".to_string());

    let options = match parse_args(args) {
        Ok(Command::Help) => {
            print_usage(&program);
            return ExitCode::SUCCESS;
        }
        Ok(Command::Version) => {
            println!("{program} {VERSION}");
            return ExitCode::SUCCESS;
        }
        Ok(Command::Run(options)) => options,
        Err(msg) => {
            eprintln!("{program}: {msg}\nTry '{program} --help' for more information.");
            return ExitCode::from(2);
        }
    };

    let summary = run(&options);
    let verb = if options.dry_run { "would be created" } else { "created" };

    if summary.failed > 0 {
        eprintln!(
            "\nCompleted with errors: {} {verb}, {} already existed, {} failed.",
            summary.created, summary.existing, summary.failed
        );
        ExitCode::FAILURE
    } else {
        if !options.quiet {
            println!(
                "\nDone: {} {verb}, {} already existed.",
                summary.created, summary.existing
            );
        }
        ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;

    fn scratch(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("mkdir-safe-{}-{name}", process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    fn os(args: &[&str]) -> Vec<OsString> {
        args.iter().map(OsString::from).collect()
    }

    #[test]
    fn creates_nested_then_reports_exists() {
        let dir = scratch("nested").join("a/b/c");
        assert_eq!(ensure_dir(&dir, false).unwrap(), DirStatus::Created);
        assert_eq!(ensure_dir(&dir, false).unwrap(), DirStatus::Exists);
        let _ = fs::remove_dir_all(dir.ancestors().nth(3).unwrap());
    }

    #[test]
    fn dry_run_creates_nothing() {
        let dir = scratch("dry");
        assert_eq!(ensure_dir(&dir, true).unwrap(), DirStatus::WouldCreate);
        assert!(!dir.exists());
    }

    #[test]
    fn rejects_empty_path_and_files() {
        assert!(matches!(ensure_dir(Path::new(""), false), Err(AppError::EmptyPath)));

        let base = scratch("file");
        fs::create_dir_all(&base).unwrap();
        let file = base.join("f.txt");
        fs::write(&file, b"x").unwrap();
        assert!(matches!(ensure_dir(&file, false), Err(AppError::NotADirectory(_))));
        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn parses_flags_defaults_and_separator() {
        assert_eq!(parse_args(os(&["--help"])), Ok(Command::Help));
        assert!(parse_args(os(&["--bogus"])).is_err());

        let Ok(Command::Run(opts)) = parse_args(os(&[])) else { panic!() };
        assert_eq!(opts.targets, vec![PathBuf::from(DEFAULT_DIR)]);

        let Ok(Command::Run(opts)) = parse_args(os(&["-n", "--", "--help", "a", "a"])) else {
            panic!()
        };
        assert!(opts.dry_run);
        assert_eq!(opts.targets, vec![PathBuf::from("--help"), PathBuf::from("a")]);
    }
}
