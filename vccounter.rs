use std::io::{self, Read, Write};

const VOWELS: u32 = 1 | 1 << 4 | 1 << 8 | 1 << 14 | 1 << 20;

fn count_letters(bytes: &[u8]) -> (usize, usize) {
    let (vowels, letters) = bytes.iter().fold((0usize, 0usize), |(v, l), &b| {
        let idx = (b | 0x20).wrapping_sub(b'a');
        let is_letter = idx < 26;
        let is_vowel = is_letter & ((VOWELS >> (idx & 31)) & 1 == 1);
        (v + is_vowel as usize, l + is_letter as usize)
    });
    (vowels, letters - vowels)
}

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().lock().read_to_end(&mut buf)?;

    let (vowels, consonants) = count_letters(&buf);

    let mut out = io::stdout().lock();
    out.write_all(buf.trim_ascii())?;
    writeln!(out, "\n{vowels}\n{consonants}")?;
    out.flush()
}
