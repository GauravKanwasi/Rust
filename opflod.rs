use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SNIFF_LEN: usize = 1024;
const HEAD_LEN: u64 = 8192;
const CHUNK_LEN: usize = 64 * 1024;

fn main() -> ExitCode {
    let (dir, recursive) = match parse_args() {
        Some(v) => v,
        None => {
            eprintln!("Usage: opflod [-r|--recursive] <folder_path>");
            return ExitCode::from(2);
        }
    };

    let base = match fs::canonicalize(&dir) {
        Ok(p) if p.is_dir() => p,
        Ok(_) => {
            eprintln!("Error: not a directory: {}", dir.display());
            return ExitCode::from(1);
        }
        Err(e) => {
            eprintln!("Error: {}: {}", dir.display(), e);
            return ExitCode::from(1);
        }
    };

    let files = collect_files(&base, recursive);
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let result = files
        .iter()
        .try_for_each(|path| dump_file(&mut out, &base, path))
        .and_then(|_| out.flush());

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::from(1)
        }
    }
}

fn parse_args() -> Option<(PathBuf, bool)> {
    let mut recursive = false;
    let mut dir = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-r" | "--recursive" => recursive = true,
            _ if dir.is_none() => dir = Some(PathBuf::from(arg)),
            _ => return None,
        }
    }

    dir.map(|d| (d, recursive))
}

fn collect_files(base: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![base.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("skip {}: {}", dir.display(), e);
                continue;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };

            if file_type.is_dir() {
                if recursive {
                    stack.push(path);
                }
            } else if file_type.is_file() {
                files.push(path);
            } else if file_type.is_symlink() {
                if let Ok(real) = fs::canonicalize(&path) {
                    if real.starts_with(base) && real.is_file() {
                        files.push(path);
                    }
                }
            }
        }
    }

    files.sort();
    files
}

fn dump_file<W: Write>(out: &mut W, base: &Path, path: &Path) -> io::Result<()> {
    let display = path.strip_prefix(base).unwrap_or(path).display().to_string();

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("open error {}: {}", display, e);
            return Ok(());
        }
    };

    let mut head = Vec::with_capacity(HEAD_LEN as usize);
    if let Err(e) = file.by_ref().take(HEAD_LEN).read_to_end(&mut head) {
        eprintln!("read error {}: {}", display, e);
        return Ok(());
    }

    if head.iter().take(SNIFF_LEN).any(|&b| b == 0) {
        return Ok(());
    }

    writeln!(out, "--- File: {} ---", display)?;
    out.write_all(&head)?;
    let mut last = head.last().copied();

    let mut buf = vec![0u8; CHUNK_LEN];
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                out.write_all(&buf[..n])?;
                last = Some(buf[n - 1]);
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => {
                eprintln!("read error {}: {}", display, e);
                break;
            }
        }
    }

    if matches!(last, Some(b) if b != b'\n') {
        out.write_all(b"\n")?;
    }

    Ok(())
}
