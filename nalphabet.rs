use std::env;
use std::fs::File;
use std::io::{self, BufWriter, IsTerminal, Read, Write};
use std::process::ExitCode;

const VERSION: &str = "0.2.0";

const HELP: &str = "\
letter_freq - count letter frequencies in files or stdin

USAGE:
    letter_freq [OPTIONS] [FILE...]

OPTIONS:
    -t, --top <N>        Show only the first N rows after sorting
    -s, --sort <MODE>    Sort by 'freq' (default) or 'alpha'
    -r, --reverse        Reverse the sort order
    -w, --width <N>      Maximum bar width in characters (default 30)
    -z, --zeros          Include letters that never appear
    -e, --english        Compare against typical English letter frequencies
    -f, --format <FMT>   Output as 'table' (default), 'csv' or 'json'
        --color <WHEN>   Colorize bars: 'auto' (default), 'always' or 'never'
    -h, --help           Show this help
    -V, --version        Show version

Options accept '--opt=value' and '-tN' forms; short flags can be combined
(e.g. -zr). Use '-' to read stdin explicitly and '--' to end option parsing.
With no FILE, input is read from stdin. Files that cannot be read are
reported on stderr; the remaining input is still counted (exit code 1).";

/// Relative frequency (%) of letters A-Z in typical English text.
const ENGLISH: [f64; 26] = [
    8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 6.966, 0.153, 0.772, 4.025, 2.406,
    6.749, 7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758, 0.978, 2.360, 0.150, 1.974, 0.074,
];

const PARTIAL: [&str; 8] = ["", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];

#[derive(Clone, Copy, PartialEq)]
enum SortMode {
    Freq,
    Alpha,
}

#[derive(Clone, Copy, PartialEq)]
enum Format {
    Table,
    Csv,
    Json,
}

#[derive(Clone, Copy, PartialEq)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

struct Config {
    top: Option<usize>,
    sort: SortMode,
    reverse: bool,
    width: usize,
    zeros: bool,
    english: bool,
    format: Format,
    color: ColorMode,
    files: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            top: None,
            sort: SortMode::Freq,
            reverse: false,
            width: 30,
            zeros: false,
            english: false,
            format: Format::Table,
            color: ColorMode::Auto,
            files: Vec::new(),
        }
    }
}

enum Action {
    Run(Config),
    Help,
    Version,
}

fn apply_value(cfg: &mut Config, name: &str, v: &str) -> Result<(), String> {
    match name {
        "top" => cfg.top = Some(v.parse().map_err(|_| format!("invalid number: {v}"))?),
        "width" => {
            cfg.width = v
                .parse()
                .ok()
                .filter(|&n| n > 0)
                .ok_or_else(|| format!("invalid width: {v}"))?
        }
        "sort" => {
            cfg.sort = match v {
                "freq" => SortMode::Freq,
                "alpha" => SortMode::Alpha,
                other => return Err(format!("unknown sort mode: {other}")),
            }
        }
        "format" => {
            cfg.format = match v {
                "table" => Format::Table,
                "csv" => Format::Csv,
                "json" => Format::Json,
                other => return Err(format!("unknown format: {other}")),
            }
        }
        "color" => {
            cfg.color = match v {
                "auto" => ColorMode::Auto,
                "always" => ColorMode::Always,
                "never" => ColorMode::Never,
                other => return Err(format!("unknown color mode: {other}")),
            }
        }
        _ => unreachable!("unhandled option {name}"),
    }
    Ok(())
}

fn parse_args<I: Iterator<Item = String>>(mut args: I) -> Result<Action, String> {
    let mut cfg = Config::default();
    let mut only_files = false;

    while let Some(arg) = args.next() {
        if only_files || arg == "-" || !arg.starts_with('-') {
            cfg.files.push(arg);
            continue;
        }
        if arg == "--" {
            only_files = true;
            continue;
        }

        if let Some(long) = arg.strip_prefix("--") {
            let (name, inline) = match long.split_once('=') {
                Some((n, v)) => (n, Some(v.to_string())),
                None => (long, None),
            };
            match name {
                "help" => return Ok(Action::Help),
                "version" => return Ok(Action::Version),
                "zeros" => cfg.zeros = true,
                "reverse" => cfg.reverse = true,
                "english" => cfg.english = true,
                "top" | "sort" | "width" | "format" | "color" => {
                    let v = match inline {
                        Some(v) => v,
                        None => args
                            .next()
                            .ok_or_else(|| format!("missing value for --{name}"))?,
                    };
                    apply_value(&mut cfg, name, &v)?;
                }
                _ => return Err(format!("unknown option: --{name}")),
            }
        } else {
            let shorts = &arg[1..];
            for (i, c) in shorts.char_indices() {
                match c {
                    'h' => return Ok(Action::Help),
                    'V' => return Ok(Action::Version),
                    'z' => cfg.zeros = true,
                    'r' => cfg.reverse = true,
                    'e' => cfg.english = true,
                    't' | 's' | 'w' | 'f' => {
                        let name = match c {
                            't' => "top",
                            's' => "sort",
                            'w' => "width",
                            _ => "format",
                        };
                        let rest = &shorts[i + c.len_utf8()..];
                        let v = if rest.is_empty() {
                            args.next()
                                .ok_or_else(|| format!("missing value for -{c}"))?
                        } else {
                            rest.to_string()
                        };
                        apply_value(&mut cfg, name, &v)?;
                        break;
                    }
                    _ => return Err(format!("unknown option: -{c}")),
                }
            }
        }
    }
    Ok(Action::Run(cfg))
}

/// Count ASCII letters from a reader in fixed-size chunks (constant memory).
fn count_reader<R: Read>(mut reader: R, counts: &mut [u64; 26]) -> io::Result<()> {
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        for &b in &buf[..n] {
            if b.is_ascii_alphabetic() {
                counts[usize::from((b | 0x20) - b'a')] += 1;
            }
        }
    }
    Ok(())
}

/// Returns the counts and whether any input failed to read.
fn read_input(cfg: &Config) -> ([u64; 26], bool) {
    let mut counts = [0u64; 26];
    let mut failed = false;

    let default = vec!["-".to_string()];
    let paths = if cfg.files.is_empty() {
        &default
    } else {
        &cfg.files
    };

    for path in paths {
        let result = if path == "-" {
            count_reader(io::stdin().lock(), &mut counts)
        } else {
            File::open(path).and_then(|f| count_reader(f, &mut counts))
        };
        if let Err(e) = result {
            let name = if path == "-" { "stdin" } else { path.as_str() };
            eprintln!("error: {name}: {e}");
            failed = true;
        }
    }
    (counts, failed)
}

fn bar(count: u64, max: u64, width: usize) -> String {
    if max == 0 || count == 0 {
        return String::new();
    }
    let eighths =
        ((count as u128 * width as u128 * 8 + max as u128 / 2) / max as u128).max(1) as usize;
    format!("{}{}", "█".repeat(eighths / 8), PARTIAL[eighths % 8])
}

fn pct(n: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        n as f64 * 100.0 / total as f64
    }
}

struct Row {
    ch: char,
    idx: usize,
    count: u64,
    cumulative: u64,
}

fn build_rows(counts: &[u64; 26], cfg: &Config) -> Vec<Row> {
    let mut rows: Vec<Row> = counts
        .iter()
        .enumerate()
        .filter(|&(_, &c)| cfg.zeros || c > 0)
        .map(|(i, &c)| Row {
            ch: (b'A' + i as u8) as char,
            idx: i,
            count: c,
            cumulative: 0,
        })
        .collect();

    match cfg.sort {
        SortMode::Freq => rows.sort_by(|a, b| b.count.cmp(&a.count).then(a.ch.cmp(&b.ch))),
        SortMode::Alpha => rows.sort_by_key(|r| r.ch),
    }
    if cfg.reverse {
        rows.reverse();
    }
    if let Some(n) = cfg.top {
        rows.truncate(n);
    }

    let mut running = 0;
    for r in &mut rows {
        running += r.count;
        r.cumulative = running;
    }
    rows
}

fn render_table(
    out: &mut impl Write,
    rows: &[Row],
    counts: &[u64; 26],
    total: u64,
    cfg: &Config,
    color: bool,
) -> io::Result<()> {
    if total == 0 {
        return writeln!(out, "No alphabetic characters found.");
    }

    let distinct = counts.iter().filter(|&&c| c > 0).count();
    writeln!(
        out,
        "Letter frequency ({total} letters, {distinct}/26 distinct)\n"
    )?;

    let mut header = format!("{:>2} | {:>10} | {:>6} | {:>6}", "", "Count", "%", "Cum.%");
    let mut rule = format!("{:-<3}+{:-<12}+{:-<8}+{:-<8}", "", "", "", "");
    if cfg.english {
        header += &format!(" | {:>6} | {:>7}", "Eng.%", "Diff");
        rule += &format!("+{:-<8}+{:-<9}", "", "");
    }
    header += " | Distribution";
    rule += &format!("+{:-<w$}", "", w = cfg.width + 2);
    writeln!(out, "{header}")?;
    writeln!(out, "{rule}")?;

    let max = rows.iter().map(|r| r.count).max().unwrap_or(0);
    for r in rows {
        let p = pct(r.count, total);
        let mut line = format!(
            "{:>2} | {:>10} | {:>5.1}% | {:>5.1}%",
            r.ch,
            r.count,
            p,
            pct(r.cumulative, total)
        );
        if cfg.english {
            let exp = ENGLISH[r.idx];
            line += &format!(" | {:>5.1}% | {:>+6.1}%", exp, p - exp);
        }
        let b = bar(r.count, max, cfg.width);
        if color && !b.is_empty() {
            writeln!(out, "{line} | \x1b[36m{b}\x1b[0m")?;
        } else {
            writeln!(out, "{line} | {b}")?;
        }
    }

    // Summary statistics always cover all input, not just the displayed rows.
    let vowels: u64 = b"aeiou".iter().map(|&v| counts[usize::from(v - b'a')]).sum();
    let consonants = total - vowels;
    let entropy: f64 = counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / total as f64;
            -p * p.log2()
        })
        .sum();

    writeln!(
        out,
        "\nVowels: {vowels} ({:.1}%)   Consonants: {consonants} ({:.1}%)",
        pct(vowels, total),
        pct(consonants, total)
    )?;
    writeln!(out, "Entropy: {entropy:.2} bits/letter (max 4.70)")
}

fn render_csv(out: &mut impl Write, rows: &[Row], total: u64, cfg: &Config) -> io::Result<()> {
    write!(out, "letter,count,percent,cumulative_percent")?;
    if cfg.english {
        write!(out, ",english_percent")?;
    }
    writeln!(out)?;
    for r in rows {
        write!(
            out,
            "{},{},{:.4},{:.4}",
            r.ch,
            r.count,
            pct(r.count, total),
            pct(r.cumulative, total)
        )?;
        if cfg.english {
            write!(out, ",{:.4}", ENGLISH[r.idx])?;
        }
        writeln!(out)?;
    }
    Ok(())
}

fn render_json(
    out: &mut impl Write,
    rows: &[Row],
    counts: &[u64; 26],
    total: u64,
    cfg: &Config,
) -> io::Result<()> {
    let distinct = counts.iter().filter(|&&c| c > 0).count();
    writeln!(out, "{{")?;
    writeln!(out, "  \"total\": {total},")?;
    writeln!(out, "  \"distinct\": {distinct},")?;
    writeln!(out, "  \"letters\": [")?;
    for (i, r) in rows.iter().enumerate() {
        write!(
            out,
            "    {{\"letter\": \"{}\", \"count\": {}, \"percent\": {:.4}, \"cumulative_percent\": {:.4}",
            r.ch,
            r.count,
            pct(r.count, total),
            pct(r.cumulative, total)
        )?;
        if cfg.english {
            write!(out, ", \"english_percent\": {:.4}", ENGLISH[r.idx])?;
        }
        let comma = if i + 1 < rows.len() { "," } else { "" };
        writeln!(out, "}}{comma}")?;
    }
    writeln!(out, "  ]")?;
    writeln!(out, "}}")
}

fn render(
    out: &mut impl Write,
    counts: &[u64; 26],
    cfg: &Config,
    color: bool,
) -> io::Result<()> {
    let total: u64 = counts.iter().sum();
    let rows = build_rows(counts, cfg);
    match cfg.format {
        Format::Table => render_table(out, &rows, counts, total, cfg, color),
        Format::Csv => render_csv(out, &rows, total, cfg),
        Format::Json => render_json(out, &rows, counts, total, cfg),
    }
}

fn main() -> ExitCode {
    let cfg = match parse_args(env::args().skip(1)) {
        Ok(Action::Run(c)) => c,
        Ok(Action::Help) => {
            println!("{HELP}");
            return ExitCode::SUCCESS;
        }
        Ok(Action::Version) => {
            println!("letter_freq {VERSION}");
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("error: {e}\n\n{HELP}");
            return ExitCode::from(2);
        }
    };

    let (counts, read_failed) = read_input(&cfg);

    let color = match cfg.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none(),
    };

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let result = render(&mut out, &counts, &cfg, color).and_then(|_| out.flush());

    match result {
        // A closed pipe (e.g. `letter_freq f | head`) is not an error worth reporting.
        Err(e) if e.kind() != io::ErrorKind::BrokenPipe => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
        _ if read_failed => ExitCode::from(1),
        _ => ExitCode::SUCCESS,
    }
}
