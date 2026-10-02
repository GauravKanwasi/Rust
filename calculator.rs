use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
enum Command {
    Quit,
    Help,
    History,
    Clear,
}

enum Input<T> {
    Value(T),
    Command(Command),
    EndOfInput,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    FloorDivide,
    Power,
    Factorial,
    LeftParen,
    RightParen,
    Comma,
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
    answer: Option<f64>,
}

impl Parser {
    fn new(tokens: Vec<Token>, answer: Option<f64>) -> Self {
        Self {
            tokens,
            position: 0,
            answer,
        }
    }

    fn parse(&mut self) -> Result<f64, String> {
        if self.tokens.is_empty() {
            return Err("enter an expression".to_string());
        }

        let value = self.parse_addition()?;

        if let Some(token) = self.peek() {
            return Err(format!("unexpected token: {}", describe_token(token)));
        }

        finite(value)
    }

    // Addition and subtraction have the lowest binary precedence.
    fn parse_addition(&mut self) -> Result<f64, String> {
        let mut value = self.parse_multiplication()?;

        loop {
            let operator = match self.peek() {
                Some(Token::Plus) => Some('+'),
                Some(Token::Minus) => Some('-'),
                _ => None,
            };

            let Some(operator) = operator else {
                break;
            };

            self.take();
            let right = self.parse_multiplication()?;
            value = calculate_binary(value, right, operator)?;
        }

        Ok(value)
    }

    // Multiplication, division, remainder, and floor division.
    fn parse_multiplication(&mut self) -> Result<f64, String> {
        let mut value = self.parse_unary()?;

        loop {
            let operator = match self.peek() {
                Some(Token::Multiply) => Some('*'),
                Some(Token::Divide) => Some('/'),
                Some(Token::Modulo) => Some('%'),
                Some(Token::FloorDivide) => Some('⌊'),
                _ => None,
            };

            let Some(operator) = operator else {
                break;
            };

            self.take();
            let right = self.parse_unary()?;
            value = calculate_binary(value, right, operator)?;
        }

        Ok(value)
    }

    fn parse_unary(&mut self) -> Result<f64, String> {
        match self.peek() {
            Some(Token::Plus) => {
                self.take();
                self.parse_unary()
            }
            Some(Token::Minus) => {
                self.take();
                finite(-self.parse_unary()?)
            }
            _ => self.parse_power(),
        }
    }

    // Power is right-associative: 2 ^ 3 ^ 2 means 2 ^ (3 ^ 2).
    fn parse_power(&mut self) -> Result<f64, String> {
        let base = self.parse_postfix()?;

        if matches!(self.peek(), Some(Token::Power)) {
            self.take();
            let exponent = self.parse_unary()?;
            return finite(base.powf(exponent));
        }

        Ok(base)
    }

    fn parse_postfix(&mut self) -> Result<f64, String> {
        let mut value = self.parse_primary()?;

        while matches!(self.peek(), Some(Token::Factorial)) {
            self.take();
            value = factorial(value)?;
        }

        Ok(value)
    }

    fn parse_primary(&mut self) -> Result<f64, String> {
        match self.take() {
            Some(Token::Number(value)) => Ok(value),
            Some(Token::Identifier(name)) => {
                if matches!(self.peek(), Some(Token::LeftParen)) {
                    self.take();
                    self.parse_function(&name)
                } else {
                    self.constant(&name)
                }
            }
            Some(Token::LeftParen) => {
                let value = self.parse_addition()?;

                match self.take() {
                    Some(Token::RightParen) => Ok(value),
                    Some(token) => Err(format!(
                        "expected ')', found {}",
                        describe_token(&token)
                    )),
                    None => Err("missing ')'".to_string()),
                }
            }
            Some(token) => Err(format!(
                "expected a number, found {}",
                describe_token(&token)
            )),
            None => Err("expected a value".to_string()),
        }
    }

    fn parse_function(&mut self, name: &str) -> Result<f64, String> {
        let first = self.parse_addition()?;

        let second = if matches!(self.peek(), Some(Token::Comma)) {
            self.take();
            Some(self.parse_addition()?)
        } else {
            None
        };

        match self.take() {
            Some(Token::RightParen) => apply_function(name, first, second),
            Some(token) => Err(format!(
                "expected ')', found {}",
                describe_token(&token)
            )),
            None => Err(format!("missing ')' after {}", name)),
        }
    }

    fn constant(&self, name: &str) -> Result<f64, String> {
        match name {
            "pi" => Ok(std::f64::consts::PI),
            "tau" => Ok(std::f64::consts::TAU),
            "e" => Ok(std::f64::consts::E),
            "ans" => self
                .answer
                .ok_or_else(|| "there is no previous answer yet".to_string()),
            _ => Err(format!("unknown constant or function: {}", name)),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn take(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();

        if token.is_some() {
            self.position += 1;
        }

        token
    }
}

fn main() {
    print_banner();

    let mut history: Vec<String> = Vec::new();
    let mut last_result: Option<f64> = None;

    loop {
        let expression = match read_line("Expression") {
            Input::Value(value) => value,
            Input::Command(command) => {
                if !handle_command(command, &history) {
                    break;
                }
                continue;
            }
            Input::EndOfInput => break,
        };

        if expression.trim().is_empty() {
            println!("Please enter an expression.\n");
            continue;
        }

        match evaluate(&expression, last_result) {
            Ok(value) => {
                let record = format!("{} = {}", expression, value);
                println!("\nResult: {}\n", value);
                history.push(record);
                last_result = Some(value);
            }
            Err(message) => {
                let record = format!("{} -> Error: {}", expression, message);
                println!("\nError: {}\n", message);
                history.push(record);
            }
        }
    }

    println!("\nCalculator closed.");
}

fn evaluate(expression: &str, answer: Option<f64>) -> Result<f64, String> {
    let tokens = tokenize(expression)?;
    Parser::new(tokens, answer).parse()
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let characters: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < characters.len() {
        let current = characters[index];

        if current.is_whitespace() {
            index += 1;
            continue;
        }

        if current.is_ascii_digit() || current == '.' {
            let start = index;
            let mut decimal_points = 0;

            while index < characters.len()
                && (characters[index].is_ascii_digit() || characters[index] == '.')
            {
                if characters[index] == '.' {
                    decimal_points += 1;
                }
                index += 1;
            }

            if decimal_points > 1 {
                return Err("a number cannot contain multiple decimal points".to_string());
            }

            if index < characters.len() && matches!(characters[index], 'e' | 'E') {
                index += 1;

                if index < characters.len() && matches!(characters[index], '+' | '-') {
                    index += 1;
                }

                let exponent_start = index;
                while index < characters.len() && characters[index].is_ascii_digit() {
                    index += 1;
                }

                if exponent_start == index {
                    return Err("invalid scientific notation".to_string());
                }
            }

            let text: String = characters[start..index].iter().collect();
            let number = text
                .parse::<f64>()
                .map_err(|_| format!("invalid number: {}", text))?;

            if !number.is_finite() {
                return Err("numbers must be finite".to_string());
            }

            tokens.push(Token::Number(number));
            continue;
        }

        if current.is_ascii_alphabetic() || current == '_' {
            let start = index;

            while index < characters.len()
                && (characters[index].is_ascii_alphanumeric() || characters[index] == '_')
            {
                index += 1;
            }

            let name: String = characters[start..index]
                .iter()
                .collect::<String>()
                .to_ascii_lowercase();
            tokens.push(Token::Identifier(name));
            continue;
        }

        let token = match current {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' if characters.get(index + 1) == Some(&'*') => {
                index += 1;
                Token::Power
            }
            '*' | '×' => Token::Multiply,
            '/' if characters.get(index + 1) == Some(&'/') => {
                index += 1;
                Token::FloorDivide
            }
            '/' | '÷' => Token::Divide,
            '%' => Token::Modulo,
            '^' => Token::Power,
            '!' => Token::Factorial,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            ',' => Token::Comma,
            _ => return Err(format!("unsupported character: '{}'", current)),
        };

        tokens.push(token);
        index += 1;
    }

    Ok(tokens)
}

fn calculate_binary(left: f64, right: f64, operator: char) -> Result<f64, String> {
    if matches!(operator, '/' | '%' | '⌊') && right == 0.0 {
        return Err("division by zero is not allowed".to_string());
    }

    let result = match operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        '%' => left % right,
        '⌊' => (left / right).floor(),
        _ => unreachable!(),
    };

    finite(result)
}

fn apply_function(name: &str, first: f64, second: Option<f64>) -> Result<f64, String> {
    let result = match (name, second) {
        ("sqrt", None) => first.sqrt(),
        ("cbrt", None) => first.cbrt(),
        ("abs", None) => first.abs(),
        ("sin", None) => first.sin(),
        ("cos", None) => first.cos(),
        ("tan", None) => first.tan(),
        ("asin", None) => first.asin(),
        ("acos", None) => first.acos(),
        ("atan", None) => first.atan(),
        ("sinh", None) => first.sinh(),
        ("cosh", None) => first.cosh(),
        ("tanh", None) => first.tanh(),
        ("ln", None) => first.ln(),
        ("log", None) | ("log10", None) => first.log10(),
        ("log2", None) => first.log2(),
        ("exp", None) => first.exp(),
        ("floor", None) => first.floor(),
        ("ceil", None) => first.ceil(),
        ("round", None) => first.round(),
        ("sign", None) => first.signum(),
        ("percent", None) => first / 100.0,
        ("min", Some(value)) => first.min(value),
        ("max", Some(value)) => first.max(value),
        ("hypot", Some(value)) => first.hypot(value),
        ("atan2", Some(value)) => first.atan2(value),
        (name, Some(_)) => return Err(format!("{}() accepts one argument", name)),
        (name, None) => return Err(format!("unknown function: {}", name)),
    };

    finite(result)
}

fn factorial(value: f64) -> Result<f64, String> {
    if value < 0.0 || value.fract() != 0.0 {
        return Err("factorial requires a non-negative whole number".to_string());
    }

    if value > 170.0 {
        return Err("factorial is limited to 170! for finite results".to_string());
    }

    let mut result = 1.0;
    let mut number = 2.0;

    while number <= value {
        result *= number;
        number += 1.0;
    }

    Ok(result)
}

fn finite(value: f64) -> Result<f64, String> {
    if value.is_finite() {
        Ok(value)
    } else if value.is_nan() {
        Err("the operation produced an undefined number".to_string())
    } else {
        Err("the result is too large to represent".to_string())
    }
}

fn describe_token(token: &Token) -> String {
    match token {
        Token::Number(value) => value.to_string(),
        Token::Identifier(name) => name.clone(),
        Token::Plus => "+".to_string(),
        Token::Minus => "-".to_string(),
        Token::Multiply => "*".to_string(),
        Token::Divide => "/".to_string(),
        Token::Modulo => "%".to_string(),
        Token::FloorDivide => "//".to_string(),
        Token::Power => "^".to_string(),
        Token::Factorial => "!".to_string(),
        Token::LeftParen => "(".to_string(),
        Token::RightParen => ")".to_string(),
        Token::Comma => ",".to_string(),
    }
}

fn read_line(prompt: &str) -> Input<String> {
    print!("{}: ", prompt);

    if io::stdout().flush().is_err() {
        return Input::EndOfInput;
    }

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(0) | Err(_) => Input::EndOfInput,
        Ok(_) => {
            let value = input.trim();

            if let Some(command) = parse_command(value) {
                Input::Command(command)
            } else {
                Input::Value(value.to_owned())
            }
        }
    }
}

fn parse_command(input: &str) -> Option<Command> {
    match input.to_ascii_lowercase().as_str() {
        "q" | "quit" | "exit" => Some(Command::Quit),
        "h" | "help" | "?" => Some(Command::Help),
        "history" => Some(Command::History),
        "clear" | "cls" => Some(Command::Clear),
        _ => None,
    }
}

fn handle_command(command: Command, history: &[String]) -> bool {
    match command {
        Command::Quit => false,
        Command::Help => {
            print_help();
            true
        }
        Command::History => {
            print_history(history);
            true
        }
        Command::Clear => {
            clear_screen();
            print_banner();
            true
        }
    }
}

fn print_banner() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("                 Advanced Calculator");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Examples: 2 + 3 * 4   sqrt(81)   5!   sin(pi / 2)");
    println!("Type 'help' for operators or 'q' to quit.\n");
}

fn print_help() {
    println!(
        "\nOperators:\n\
         +  addition       -  subtraction\n\
         *  multiplication /  division\n\
         %  remainder      // floor division\n\
         ^ or ** power      !  factorial\n\
         ( ) parentheses\n\n\
         Functions:\n\
         sqrt, cbrt, abs, sin, cos, tan, asin, acos, atan\n\
         sinh, cosh, tanh, ln, log, log2, exp, floor, ceil\n\
         round, sign, percent, min(a,b), max(a,b), hypot(a,b), atan2(a,b)\n\n\
         Constants:\n\
         pi, tau, e, ans (the previous successful answer)\n\n\
         Commands: help, history, clear, quit\n"
    );
}

fn print_history(history: &[String]) {
    if history.is_empty() {
        println!("\nNo calculations yet.\n");
        return;
    }

    println!("\nCalculation history:");

    for (index, calculation) in history.iter().enumerate() {
        println!("{}. {}", index + 1, calculation);
    }

    println!();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}
