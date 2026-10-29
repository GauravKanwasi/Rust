use std::io::{self, BufRead, BufWriter, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = BufWriter::new(stdout.lock());
    let mut buffer = String::new();

    while reader.read_line(&mut buffer)? > 0 {
        writer.write_all(buffer.to_uppercase().as_bytes())?;
        buffer.clear();
    }

    writer.flush()
}
