use std::io::{self, BufRead, Write};

fn prompt<R: BufRead, W: Write>(
    input: &mut R,
    output: &mut W,
    message: &str,
) -> io::Result<Option<String>> {
    write!(output, "{message}")?;
    output.flush()?;

    let mut line = String::new();
    if input.read_line(&mut line)? == 0 {
        return Ok(None);
    }

    Ok(Some(line.split_whitespace().collect::<Vec<_>>().join(" ")))
}

fn greeting(name: &str) -> String {
    if name.is_empty() {
        "It's great to meet you, stranger!".to_string()
    } else {
        format!("It's great to meet you, {name}!")
    }
}

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();

    let name = prompt(&mut input, &mut output, "Hello! Please tell me your name: ")?
        .unwrap_or_default();

    writeln!(output, "{}", greeting(&name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn ask(input: &str) -> Option<String> {
        prompt(&mut Cursor::new(input), &mut Vec::new(), "").unwrap()
    }

    #[test]
    fn trims_and_collapses_whitespace() {
        assert_eq!(ask("  Ada   Lovelace \n"), Some("Ada Lovelace".into()));
    }

    #[test]
    fn blank_line_gives_empty_name() {
        assert_eq!(ask("\n"), Some(String::new()));
    }

    #[test]
    fn eof_gives_none() {
        assert_eq!(ask(""), None);
    }

    #[test]
    fn greetings() {
        assert_eq!(greeting("Ada"), "It's great to meet you, Ada!");
        assert_eq!(greeting(""), "It's great to meet you, stranger!");
    }
}
