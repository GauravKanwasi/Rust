use std::collections::BTreeMap;
use std::io::{self, Write};

type Vars = BTreeMap<String, f64>;

const MAX_DEPTH: usize = 100;
const COMMANDS: &[&str] = &[
    "help", "?", "exit", "quit", "q", "clear", "history", "vars", "upper", "lower", "reverse",
    "count", "greet", "let",
];

const HELP: &str = "\
  <expr>           evaluate, e.g. 2*(3+4)^2 / x   (result saved as 'ans')
  let x = <expr>   store a variable
  vars             list variables
  upper|lower|reverse|count <text>
  greet [name]
  history | clear | help | exit";

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
    depth: usize,
    vars: &'a Vars,
}

fn finite(v: f64) -> Result<f64, String> {
    if v.is_finite() { Ok(v) } else { Err("result is not a finite number".into()) }
}

fn valid_name(s: &str) -> bool {
    let mut cs = s.chars();
    cs.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && cs.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

impl Parser<'_> {
    fn peek(&mut self) -> Option<u8> {
        while self.s.get(self.i).is_some_and(u8::is_ascii_whitespace) {
            self.i += 1;
        }
        self.s.get(self.i).copied()
    }

    fn eat(&mut self, c: u8) -> bool {
        let hit = self.peek() == Some(c);
        self.i += hit as usize;
        hit
    }

    fn take(&mut self, f: impl Fn(u8) -> bool) {
        while self.s.get(self.i).is_some_and(|&c| f(c)) {
            self.i += 1;
        }
    }

    fn expr(&mut self) -> Result<f64, String> {
        let mut v = self.term()?;
        loop {
            if self.eat(b'+') {
                v = finite(v + self.term()?)?;
            } else if self.eat(b'-') {
                v = finite(v - self.term()?)?;
            } else {
                return Ok(v);
            }
        }
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut v = self.unary()?;
        loop {
            if self.eat(b'*') {
                v = finite(v * self.unary()?)?;
            } else if self.eat(b'/') {
                let d = self.unary()?;
                if d == 0.0 {
                    return Err("division by zero".into());
                }
                v = finite(v / d)?;
            } else {
                return Ok(v);
            }
        }
    }

    fn unary(&mut self) -> Result<f64, String> {
        if self.depth >= MAX_DEPTH {
            return Err("expression too deeply nested".into());
        }
        self.depth += 1;
        let r = if self.eat(b'-') {
            self.unary().map(|v| -v)
        } else {
            self.atom().and_then(|base| {
                if self.eat(b'^') {
                    self.unary().and_then(|e| finite(base.powf(e)))
                } else {
                    Ok(base)
                }
            })
        };
        self.depth -= 1;
        r
    }

    fn atom(&mut self) -> Result<f64, String> {
        if self.eat(b'(') {
            let v = self.expr()?;
            return if self.eat(b')') { Ok(v) } else { Err("missing ')'".into()) };
        }
        match self.peek() {
            None => Err("unexpected end of input".into()),
            Some(c) if c.is_ascii_digit() || c == b'.' => {
                let start = self.i;
                self.take(|c| c.is_ascii_digit() || c == b'.');
                if matches!(self.s.get(self.i), Some(b'e' | b'E')) {
                    let mut j = self.i + 1;
                    if matches!(self.s.get(j), Some(b'+' | b'-')) {
                        j += 1;
                    }
                    if self.s.get(j).is_some_and(u8::is_ascii_digit) {
                        self.i = j;
                        self.take(|c| c.is_ascii_digit());
                    }
                }
                let tok = String::from_utf8_lossy(&self.s[start..self.i]).into_owned();
                tok.parse::<f64>()
                    .map_err(|_| format!("invalid number '{tok}'"))
                    .and_then(finite)
            }
            Some(c) if c.is_ascii_alphabetic() || c == b'_' => {
                let start = self.i;
                self.take(|c| c.is_ascii_alphanumeric() || c == b'_');
                let name = String::from_utf8_lossy(&self.s[start..self.i]).into_owned();
                self.vars.get(&name).copied().ok_or(format!("unknown variable '{name}'"))
            }
            Some(c) => Err(format!("unexpected '{}'", c as char)),
        }
    }
}

fn eval(src: &str, vars: &Vars) -> Result<f64, String> {
    let mut p = Parser { s: src.as_bytes(), i: 0, depth: 0, vars };
    let v = p.expr()?;
    if p.peek().is_some() {
        return Err(format!("unexpected '{}'", p.s[p.i] as char));
    }
    Ok(v)
}

fn fmt(v: f64) -> String {
    let s = format!("{v:.10}");
    match s.trim_end_matches('0').trim_end_matches('.') {
        "-0" => "0".into(),
        t => t.into(),
    }
}

fn main() {
    let mut vars = Vars::new();
    let mut hist: Vec<String> = Vec::new();
    println!("Rust CLI - type 'help'");

    loop {
        print!("› ");
        io::stdout().flush().ok();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        hist.push(line.to_string());
        let (cmd, arg) = line
            .split_once(char::is_whitespace)
            .map_or((line, ""), |(c, a)| (c, a.trim()));

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
                Some((name, e))
                    if valid_name(name.trim())
                        && !COMMANDS.contains(&name.trim().to_lowercase().as_str()) =>
                {
                    match eval(e, &vars) {
                        Ok(v) => {
                            vars.insert(name.trim().to_string(), v);
                            println!("{} = {}", name.trim(), fmt(v));
                        }
                        Err(e) => eprintln!("error: {e}"),
                    }
                }
                _ => eprintln!("usage: let <name> = <expr>  (name can't be a command)"),
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
