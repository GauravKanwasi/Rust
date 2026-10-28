use std::collections::HashMap;
use std::io::{self, Write};

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
    vars: &'a HashMap<String, f64>,
}

impl Parser<'_> {
    fn peek(&mut self) -> Option<u8> {
        while self.s.get(self.i).is_some_and(u8::is_ascii_whitespace) {
            self.i += 1;
        }
        self.s.get(self.i).copied()
    }

    fn eat(&mut self, c: u8) -> bool {
        let found = self.peek() == Some(c);
        if found {
            self.i += 1;
        }
        found
    }

    // expr = term (('+' | '-') term)*
    fn expr(&mut self) -> Result<f64, String> {
        let mut v = self.term()?;
        loop {
            if self.eat(b'+') {
                v += self.term()?;
            } else if self.eat(b'-') {
                v -= self.term()?;
            } else {
                return Ok(v);
            }
        }
    }

    // term = unary (('*' | '/') unary)*
    fn term(&mut self) -> Result<f64, String> {
        let mut v = self.unary()?;
        loop {
            if self.eat(b'*') {
                v *= self.unary()?;
            } else if self.eat(b'/') {
                let d = self.unary()?;
                if d == 0.0 {
                    return Err("division by zero".into());
                }
                v /= d;
            } else {
                return Ok(v);
            }
        }
    }

    // unary = '-' unary | power
    fn unary(&mut self) -> Result<f64, String> {
        if self.eat(b'-') {
            return Ok(-self.unary()?);
        }
        let base = self.atom()?;
        if self.eat(b'^') {
            Ok(base.powf(self.unary()?))
        } else {
            Ok(base)
        }
    }

    // atom = number | name | name '(' expr ')' | '(' expr ')'
    fn atom(&mut self) -> Result<f64, String> {
        if self.eat(b'(') {
            let v = self.expr()?;
            return if self.eat(b')') { Ok(v) } else { Err("missing ')'".into()) };
        }

        let start = self.i;
        match self.peek() {
            None => Err("unexpected end of input".into()),

            Some(c) if c.is_ascii_digit() || c == b'.' => {
                let start = self.i;
                while self.s.get(self.i).is_some_and(|&c| c.is_ascii_digit() || c == b'.') {
                    self.i += 1;
                }
                let tok = std::str::from_utf8(&self.s[start..self.i]).unwrap();
                tok.parse().map_err(|_| format!("invalid number '{tok}'"))
            }

            Some(c) if c.is_ascii_alphabetic() => {
                let start = self.i.max(start);
                while self.s.get(self.i).is_some_and(|c| c.is_ascii_alphanumeric()) {
                    self.i += 1;
                }
                let name = std::str::from_utf8(&self.s[start..self.i]).unwrap();

                if self.eat(b'(') {
                    let x = self.expr()?;
                    if !self.eat(b')') {
                        return Err("missing ')'".into());
                    }
                    return match name {
                        "sqrt" => Ok(x.sqrt()),
                        "abs" => Ok(x.abs()),
                        "sin" => Ok(x.sin()),
                        "cos" => Ok(x.cos()),
                        "tan" => Ok(x.tan()),
                        "ln" => Ok(x.ln()),
                        _ => Err(format!("unknown function '{name}'")),
                    };
                }

                match name {
                    "pi" => Ok(std::f64::consts::PI),
                    "e" => Ok(std::f64::consts::E),
                    _ => self.vars.get(name).copied().ok_or(format!("unknown variable '{name}'")),
                }
            }

            Some(c) => Err(format!("unexpected '{}'", c as char)),
        }
    }
}

fn eval(src: &str, vars: &HashMap<String, f64>) -> Result<f64, String> {
    let mut p = Parser { s: src.as_bytes(), i: 0, vars };
    let v = p.expr()?;
    match p.peek() {
        Some(c) => Err(format!("unexpected '{}'", c as char)),
        None => Ok(v),
    }
}

fn main() {
    let mut vars = HashMap::new();
    println!("Calculator - try: 2*(3+4), let x = 5, sqrt(x), exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        let line = line.trim();

        match line {
            "" => continue,
            "exit" | "quit" => break,
            _ => {}
        }

        // "let x = expr" or plain "expr"
        let (name, src) = match line.strip_prefix("let ").and_then(|r| r.split_once('=')) {
            Some((n, e)) => (n.trim(), e.trim()),
            None => ("ans", line),
        };

        match eval(src, &vars) {
            Ok(v) => {
                println!("{name} = {v}");
                vars.insert(name.to_string(), v);
                vars.insert("ans".to_string(), v);
            }
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
