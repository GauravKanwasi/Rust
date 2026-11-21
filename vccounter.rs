use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().lock().read_to_end(&mut buf)?;

    let (mut vowels, mut consonants) = (0usize, 0usize);
    for &b in &buf {
        if b.is_ascii_alphabetic() {
            if matches!(b.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u') {
                vowels += 1;
            } else {
                consonants += 1;
            }
        }
    }

    let text = String::from_utf8_lossy(&buf);
    let mut out = io::stdout().lock();
    writeln!(out, "{}\n{}\n{}", text.trim(), vowels, consonants)
}
