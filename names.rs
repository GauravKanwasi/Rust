use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
    process,
};

struct Options {
    path: PathBuf,
    show_hidden: bool,
    files_only: bool,
    dirs_only: bool,
    recursive: bool,
}

fn print_usage(program: &str) {
    println!(
        "\
Rust Directory Explorer

USAGE:
    {program} [OPTIONS] [PATH]

OPTIONS:
    -a, --all          Show hidden files and directories
    -f, --files        Show files only
    -d, --dirs         Show directories only
    -r, --recursive    Recursively list directories
    -h, --help         Show this help message

EXAMPLES:
    {program}
    {program} .
    {program} -a
    {program} -f ./src
    {program} -r ./project
    {program} -a -r ./project
"
    );
}

fn human_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", size as u64, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}

fn is_hidden(name: &str) -> bool {
    name.starts_with('.')
}

fn get_file_size(path: &Path) -> u64 {
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

fn collect_entries(path: &Path, options: &Options) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !options.show_hidden && is_hidden(&name) {
            continue;
        }

        let metadata = entry.metadata()?;
        let is_dir = metadata.is_dir();
        let is_file = metadata.is_file();

        if options.files_only && !is_file {
            continue;
        }

        if options.dirs_only && !is_dir {
            continue;
        }

        entries.push(entry);
    }

    entries.sort_by_key(|entry| entry.file_name());

    Ok(entries)
}

fn print_entry(path: &Path, entry: &fs::DirEntry, prefix: &str) {
    let name = entry.file_name().to_string_lossy().to_string();

    match entry.metadata() {
        Ok(metadata) if metadata.is_dir() => {
            println!("{prefix}📁 {name}/");
        }

        Ok(metadata) if metadata.is_file() => {
            let size = human_size(get_file_size(path));
            println!("{prefix}📄 {name}  ({size})");
        }

        Ok(_) => {
            println!("{prefix}❓ {name}");
        }

        Err(_) => {
            println!("{prefix}⚠️  {name}");
        }
    }
}

fn list_directory(path: &Path, options: &Options, prefix: &str) -> io::Result<()> {
    let entries = collect_entries(path, options)?;

    for entry in entries {
        let entry_path = entry.path();

        print_entry(&entry_path, &entry, prefix);

        if options.recursive && entry_path.is_dir() {
            let new_prefix = format!("{prefix}    ");

            if let Err(error) = list_directory(&entry_path, options, &new_prefix) {
                eprintln!(
                    "{}{}: {}",
                    new_prefix,
                    entry_path.display(),
                    error
                );
            }
        }
    }

    Ok(())
}

fn parse_args() -> Result<Options, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut path = PathBuf::from(".");
    let mut show_hidden = false;
    let mut files_only = false;
    let mut dirs_only = false;
    let mut recursive = false;

    let mut path_provided = false;

    for arg in args {
        match arg.as_str() {
            "-a" | "--all" => show_hidden = true,

            "-f" | "--files" => files_only = true,

            "-d" | "--dirs" => dirs_only = true,

            "-r" | "--recursive" => recursive = true,

            "-h" | "--help" => {
                print_usage("rlist");
                process::exit(0);
            }

            _ if arg.starts_with('-') => {
                return Err(format!("unknown option: {arg}"));
            }

            _ => {
                if path_provided {
                    return Err("only one path can be provided".into());
                }

                path = PathBuf::from(arg);
                path_provided = true;
            }
        }
    }

    if files_only && dirs_only {
        return Err("cannot use --files and --dirs together".into());
    }

    Ok(Options {
        path,
        show_hidden,
        files_only,
        dirs_only,
        recursive,
    })
}

fn main() {
    let options = match parse_args() {
        Ok(options) => options,

        Err(error) => {
            eprintln!("Error: {error}");
            eprintln!("Use -h or --help for usage information.");
            process::exit(1);
        }
    };

    if !options.path.exists() {
        eprintln!("Error: path does not exist: {}", options.path.display());
        process::exit(1);
    }

    if !options.path.is_dir() {
        eprintln!("Error: not a directory: {}", options.path.display());
        process::exit(1);
    }

    println!("📂 {}\n", options.path.display());

    if let Err(error) = list_directory(&options.path, &options, "") {
        eprintln!("Error reading directory: {error}");
        process::exit(1);
    }
}
