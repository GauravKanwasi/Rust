use std::collections::BTreeMap;
use std::io::{self, Write};

type Vars = BTreeMap<String, f64>;

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
    vars: &'a Vars,
}

impl Parser<'_> {
    fn eat(&mut self, c: u8) -> bool {
        while self.s.get(self.i).is_some_and(u8::is_ascii_whitespace) {
            self.i += 1;
        }
        let hit = self.s.get(self.i) == Some(&c);
        self.i += hit as usize;
        hit
    }

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

    fn unary(&mut self) -> Result<f64, String> {
        if self.eat(b'-') {
            return Ok(-self.unary()?);
        }
        let base = self.atom()?;
        Ok(if self.eat(b'^') { base.powf(self.unary()?) } else { base })
    }

    fn atom(&mut self) -> Result<f64, String> {
        if self.eat(b'(') {
            let v = self.expr()?;
            return if self.eat(b')') { Ok(v) } else { Err("missing ')'".into()) };
        }
        let start = self.i;
        while self.s.get(self.i).is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'.' || *c == b'_') {
            self.i += 1;
        }
        let tok = std::str::from_utf8(&self.s[start..self.i]).unwrap_or("");
        if tok.is_empty() {
            return Err("unexpected input".into());
        }
        tok.parse()
            .or_else(|_| self.vars.get(tok).copied().ok_or(format!("unknown '{tok}'")))
    }
}

fn eval(src: &str, vars: &Vars) -> Result<f64, String> {
    let mut p = Parser { s: src.as_bytes(), i: 0, vars };
    let v = p.expr()?;
    if p.eat(0) || p.i < p.s.len() {
        return Err("unexpected trailing input".into());
    }
    Ok(v)
}

fn fmt(v: f64) -> String {
    format!("{v:.10}").trim_end_matches('0').trim_end_matches('.').to_string()
}

const HELP: &str = "\
  <expr>           evaluate, e.g. 2*(3+4)^2 / x   (result saved as 'ans')
  let x = <expr>   store a variable
  vars             list variables
  upper|lower|reverse|count <text>
  greet [name]
  history | clear | help | exit";

fn main() {
    let mut vars = Vars::new();
    let mut hist: Vec<String> = Vec::new();
    println!("Rust CLI - type 'help'");

    loop {
        print!("› ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        hist.push(line.to_string());
        let (cmd, arg) = line.split_once(' ').map_or((line, ""), |(c, a)| (c, a.trim()));

        match cmd.to_lowercase().as_str() {
            "help" | "?" => println!("{HELP}"),
            "exit" | "quit" | "q" => break,
            "clear" => print!("\x1B[2J\x1B[1;1H"),
            "history" => hist.iter().enumerate().for_each(|(i, h)| println!("{:>3}  {h}", i + 1)),
            "vars" => vars.iter().for_each(|(k, v)| println!("{k} = {}", fmt(*v))),
            "upper" => println!("{}", arg.to_uppercase()),
            "lower" => println!("{}", arg.to_lowercase()),
            "reverse" => println!("{}", arg.chars().rev().collect::<String>()),
            "count" => println!("{} chars, {} words", arg.chars().count(), arg.split_whitespace().count()),
            "greet" => println!("Hello, {}!", if arg.is_empty() { "stranger" } else { arg }),
            "let" => match arg.split_once('=') {
                Some((name, e)) if !name.trim().is_empty() && name.trim().chars().all(|c| c.is_alphanumeric() || c == '_') => {
                    match eval(e, &vars) {
                        Ok(v) => {
                            vars.insert(name.trim().to_string(), v);
                            println!("{} = {}", name.trim(), fmt(v));
                        }
                        Err(e) => eprintln!("error: {e}"),
                    }
                }
                _ => eprintln!("usage: let <name> = <expr>"),
            },
            _ => match eval(line, &vars) {
                Ok(v) => {
                    vars.insert("ans".into(), v);
                    println!("= {}", fmt(v));
                }
                Err(e) => eprintln!("error: {e}"),
            },
        }
    }
}
