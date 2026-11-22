use std::io::{self, BufWriter, Read, Write};

const KIND: [u8; 256] = build_table();

const fn build_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let b = i as u8;
        table[i] = if b.is_ascii_alphabetic() {
            match b.to_ascii_lowercase() {
                b'a' | b'e' | b'i' | b'o' | b'u' => 1,
                _ => 2,
            }
        } else {
            0
        };
        i += 1;
    }
    table
}

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().lock().read_to_end(&mut buf)?;

    let mut counts = [0usize; 3];
    for &b in &buf {
        counts[KIND[b as usize] as usize] += 1;
    }

    let mut out = BufWriter::new(io::stdout().lock());
    out.write_all(buf.trim_ascii())?;
    writeln!(out, "\n{}\n{}", counts[1], counts[2])?;
    out.flush()
}
