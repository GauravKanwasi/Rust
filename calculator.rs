use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Bang,
    LParen,
    RParen,
    Comma,
    Assign,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Num(n) => write!(f, "{}", n),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Caret => write!(f, "^"),
            Token::Bang => write!(f, "!"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Assign => write!(f, "="),
        }
    }
}

#[derive(Debug)]
enum CalcError {
    UnexpectedChar(char),
    InvalidNumber(String),
    UnexpectedToken(String),
    UnexpectedEnd,
    UnknownVariable(String),
    UnknownFunction(String),
    WrongArgCount { name: String, expected: usize, got: usize },
    DivisionByZero,
    ModuloByZero,
    Domain(&'static str),
    Overflow,
    ReservedName(String),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::UnexpectedChar(c) => write!(f, "Unexpected character '{}'", c),
            CalcError::InvalidNumber(s) => write!(f, "Invalid number '{}'", s),
            CalcError::UnexpectedToken(s) => write!(f, "Unexpected '{}'", s),
            CalcError::UnexpectedEnd => write!(f, "Expression ended unexpectedly"),
            CalcError::UnknownVariable(s) => write!(f, "Unknown variable '{}'", s),
            CalcError::UnknownFunction(s) => write!(f, "Unknown function '{}'", s),
            CalcError::WrongArgCount { name, expected, got } => {
                write!(f, "'{}' expects {} argument(s), got {}", name, expected, got)
            }
            CalcError::DivisionByZero => write!(f, "Division by zero is not allowed"),
            CalcError::ModuloByZero => write!(f, "Modulo by zero is not allowed"),
            CalcError::Domain(msg) => write!(f, "Math domain error: {}", msg),
            CalcError::Overflow => write!(f, "Result is too large or undefined"),
            CalcError::ReservedName(s) => write!(f, "'{}' is a reserved name", s),
        }
    }
}

fn constant(name: &str) -> Option<f64> {
    match name {
        "pi" => Some(std::f64::consts::PI),
        "e" => Some(std::f64::consts::E),
        "tau" => Some(std::f64::consts::TAU),
        _ => None,
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() || c == '.' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            let num = text
                .parse::<f64>()
                .map_err(|_| CalcError::InvalidNumber(text.clone()))?;
            tokens.push(Token::Num(num));
        } else if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            tokens.push(Token::Ident(text.to_lowercase()));
        } else {
            let token = match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '%' => Token::Percent,
                '^' => Token::Caret,
                '!' => Token::Bang,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                '=' => Token::Assign,
                _ => return Err(CalcError::UnexpectedChar(c)),
            };
            tokens.push(token);
            i += 1;
        }
    }

    Ok(tokens)
}

fn expect_args(name: &str, args: &[f64], expected: usize) -> Result<(), CalcError> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(CalcError::WrongArgCount {
            name: name.to_string(),
            expected,
            got: args.len(),
        })
    }
}

fn apply_function(name: &str, args: &[f64]) -> Result<f64, CalcError> {
    match name {
        "sqrt" => {
            expect_args(name, args, 1)?;
            if args[0] < 0.0 {
                Err(CalcError::Domain("sqrt of a negative number"))
            } else {
                Ok(args[0].sqrt())
            }
        }
        "abs" => {
            expect_args(name, args, 1)?;
            Ok(args[0].abs())
        }
        "sin" => {
            expect_args(name, args, 1)?;
            Ok(args[0].sin())
        }
        "cos" => {
            expect_args(name, args, 1)?;
            Ok(args[0].cos())
        }
        "tan" => {
            expect_args(name, args, 1)?;
            Ok(args[0].tan())
        }
        "asin" => {
            expect_args(name, args, 1)?;
            if args[0].abs() > 1.0 {
                Err(CalcError::Domain("asin needs a value between -1 and 1"))
            } else {
                Ok(args[0].asin())
            }
        }
        "acos" => {
            expect_args(name, args, 1)?;
            if args[0].abs() > 1.0 {
                Err(CalcError::Domain("acos needs a value between -1 and 1"))
            } else {
                Ok(args[0].acos())
            }
        }
        "atan" => {
            expect_args(name, args, 1)?;
            Ok(args[0].atan())
        }
        "ln" => {
            expect_args(name, args, 1)?;
            if args[0] <= 0.0 {
                Err(CalcError::Domain("ln needs a positive value"))
            } else {
                Ok(args[0].ln())
            }
        }
        "log" => {
            expect_args(name, args, 1)?;
            if args[0] <= 0.0 {
                Err(CalcError::Domain("log needs a positive value"))
            } else {
                Ok(args[0].log10())
            }
        }
        "log2" => {
            expect_args(name, args, 1)?;
            if args[0] <= 0.0 {
                Err(CalcError::Domain("log2 needs a positive value"))
            } else {
                Ok(args[0].log2())
            }
        }
        "exp" => {
            expect_args(name, args, 1)?;
            Ok(args[0].exp())
        }
        "floor" => {
            expect_args(name, args, 1)?;
            Ok(args[0].floor())
        }
        "ceil" => {
            expect_args(name, args, 1)?;
            Ok(args[0].ceil())
        }
        "round" => {
            expect_args(name, args, 1)?;
            Ok(args[0].round())
        }
        "deg" => {
            expect_args(name, args, 1)?;
            Ok(args[0].to_degrees())
        }
        "rad" => {
            expect_args(name, args, 1)?;
            Ok(args[0].to_radians())
        }
        "min" => {
            expect_args(name, args, 2)?;
            Ok(args[0].min(args[1]))
        }
        "max" => {
            expect_args(name, args, 2)?;
            Ok(args[0].max(args[1]))
        }
        "pow" => {
            expect_args(name, args, 2)?;
            power(args[0], args[1])
        }
        _ => Err(CalcError::UnknownFunction(name.to_string())),
    }
}

fn power(base: f64, exp: f64) -> Result<f64, CalcError> {
    if base == 0.0 && exp < 0.0 {
        return Err(CalcError::DivisionByZero);
    }
    if base < 0.0 && exp.fract() != 0.0 {
        return Err(CalcError::Domain("negative base with fractional exponent"));
    }
    let result = base.powf(exp);
    if result.is_finite() {
        Ok(result)
    } else {
        Err(CalcError::Overflow)
    }
}

fn factorial(n: f64) -> Result<f64, CalcError> {
    if n < 0.0 || n.fract() != 0.0 {
        return Err(CalcError::Domain("factorial needs a non-negative integer"));
    }
    if n > 170.0 {
        return Err(CalcError::Overflow);
    }
    let mut result = 1.0;
    for i in 2..=(n as u64) {
        result *= i as f64;
    }
    Ok(result)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    vars: &'a HashMap<String, f64>,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&'a Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn expect_rparen(&mut self) -> Result<(), CalcError> {
        match self.next() {
            Some(Token::RParen) => Ok(()),
            Some(t) => Err(CalcError::UnexpectedToken(t.to_string())),
            None => Err(CalcError::UnexpectedEnd),
        }
    }

    fn parse_expr(&mut self) -> Result<f64, CalcError> {
        let mut left = self.parse_term()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.pos += 1;
                    left += self.parse_term()?;
                }
                Token::Minus => {
                    self.pos += 1;
                    left -= self.parse_term()?;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<f64, CalcError> {
        let mut left = self.parse_unary()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Star => {
                    self.pos += 1;
                    left *= self.parse_unary()?;
                }
                Token::Slash => {
                    self.pos += 1;
                    let right = self.parse_unary()?;
                    if right == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }
                    left /= right;
                }
                Token::Percent => {
                    self.pos += 1;
                    let right = self.parse_unary()?;
                    if right == 0.0 {
                        return Err(CalcError::ModuloByZero);
                    }
                    left %= right;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<f64, CalcError> {
        match self.peek() {
            Some(Token::Minus) => {
                self.pos += 1;
                Ok(-self.parse_unary()?)
            }
            Some(Token::Plus) => {
                self.pos += 1;
                self.parse_unary()
            }
            _ => self.parse_power(),
        }
    }

    fn parse_power(&mut self) -> Result<f64, CalcError> {
        let base = self.parse_postfix()?;
        if let Some(Token::Caret) = self.peek() {
            self.pos += 1;
            let exp = self.parse_unary()?;
            return power(base, exp);
        }
        Ok(base)
    }

    fn parse_postfix(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_primary()?;
        while let Some(Token::Bang) = self.peek() {
            self.pos += 1;
            value = factorial(value)?;
        }
        Ok(value)
    }

    fn parse_primary(&mut self) -> Result<f64, CalcError> {
        match self.next() {
            Some(Token::Num(n)) => Ok(*n),
            Some(Token::Ident(name)) => {
                if let Some(Token::LParen) = self.peek() {
                    self.pos += 1;
                    let args = self.parse_args()?;
                    apply_function(name, &args)
                } else {
                    self.lookup(name)
                }
            }
            Some(Token::LParen) => {
                let value = self.parse_expr()?;
                self.expect_rparen()?;
                Ok(value)
            }
            Some(t) => Err(CalcError::UnexpectedToken(t.to_string())),
            None => Err(CalcError::UnexpectedEnd),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<f64>, CalcError> {
        let mut args = Vec::new();
        if let Some(Token::RParen) = self.peek() {
            self.pos += 1;
            return Ok(args);
        }
        loop {
            args.push(self.parse_expr()?);
            match self.next() {
                Some(Token::Comma) => continue,
                Some(Token::RParen) => break,
                Some(t) => return Err(CalcError::UnexpectedToken(t.to_string())),
                None => return Err(CalcError::UnexpectedEnd),
            }
        }
        Ok(args)
    }

    fn lookup(&self, name: &str) -> Result<f64, CalcError> {
        if let Some(c) = constant(name) {
            return Ok(c);
        }
        self.vars
            .get(name)
            .copied()
            .ok_or_else(|| CalcError::UnknownVariable(name.to_string()))
    }
}

fn evaluate(
    line: &str,
    vars: &HashMap<String, f64>,
) -> Result<(Option<String>, f64), CalcError> {
    let tokens = tokenize(line)?;

    let (target, body) = match (tokens.get(0), tokens.get(1)) {
        (Some(Token::Ident(name)), Some(Token::Assign)) => (Some(name.clone()), &tokens[2..]),
        _ => (None, &tokens[..]),
    };

    if let Some(name) = &target {
        if constant(name).is_some() || name == "ans" {
            return Err(CalcError::ReservedName(name.clone()));
        }
    }

    let mut parser = Parser {
        tokens: body,
        pos: 0,
        vars,
    };
    let value = parser.parse_expr()?;

    if let Some(token) = parser.peek() {
        return Err(CalcError::UnexpectedToken(token.to_string()));
    }
    if !value.is_finite() {
        return Err(CalcError::Overflow);
    }

    Ok((target, value))
}

fn format_number(v: f64) -> String {
    if v == v.trunc() && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else if v.abs() >= 1e15 || v.abs() < 1e-9 {
        format!("{:e}", v)
    } else {
        let s = format!("{:.10}", v);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn print_banner() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("          Advanced Calculator         ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Type an expression, 'help' for more, 'q' to quit\n");
}

fn print_help() {
    println!("\nOperators   + - * / % ^ !   and parentheses");
    println!("Constants   pi, e, tau, ans (last result)");
    println!("Functions   sqrt abs sin cos tan asin acos atan");
    println!("            ln log log2 exp floor ceil round deg rad");
    println!("            min(a, b)  max(a, b)  pow(a, b)");
    println!("Variables   x = 5 * 2   then use x in later expressions");
    println!("Commands    help, history, vars, clear, q\n");
    println!("Examples    2 + 3 * (4 - 1)^2");
    println!("            sqrt(16) + 5!");
    println!("            ans / 3\n");
}

fn print_history(history: &[String]) {
    if history.is_empty() {
        println!("No history yet\n");
        return;
    }
    println!();
    for (i, entry) in history.iter().enumerate() {
        println!("{:>3}. {}", i + 1, entry);
    }
    println!();
}

fn print_vars(vars: &HashMap<String, f64>) {
    if vars.is_empty() {
        println!("No variables defined\n");
        return;
    }
    let mut names: Vec<&String> = vars.keys().collect();
    names.sort();
    println!();
    for name in names {
        println!("{} = {}", name, format_number(vars[name]));
    }
    println!();
}

fn main() {
    print_banner();

    let mut vars: HashMap<String, f64> = HashMap::new();
    let mut history: Vec<String> = Vec::new();
    let stdin = io::stdin();

    loop {
        print!("calc> ");
        if io::stdout().flush().is_err() {
            break;
        }

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match line.to_lowercase().as_str() {
            "q" | "quit" | "exit" => break,
            "help" | "?" => {
                print_help();
                continue;
            }
            "history" => {
                print_history(&history);
                continue;
            }
            "vars" => {
                print_vars(&vars);
                continue;
            }
            "clear" => {
                vars.clear();
                history.clear();
                println!("Variables and history cleared\n");
                continue;
            }
            _ => {}
        }

        match evaluate(line, &vars) {
            Ok((target, value)) => {
                let shown = format_number(value);
                match target {
                    Some(name) => {
                        println!("{} = {}\n", name, shown);
                        vars.insert(name, value);
                    }
                    None => println!("= {}\n", shown),
                }
                vars.insert("ans".to_string(), value);
                history.push(format!("{} = {}", line, shown));
            }
            Err(e) => println!("Error: {}\n", e),
        }
    }

    println!("\nCalculator closed");
}
