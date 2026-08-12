use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operand(String),
    Operator(char),
    LParen,
    RParen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Associativity {
    Left,
    Right,
}

#[derive(Debug)]
enum ExprError {
    MismatchedParenthesis(String),
    UnknownCharacter(char),
    EmptyExpression,
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprError::MismatchedParenthesis(msg) => write!(f, "Mismatched Parenthesis: {}", msg),
            ExprError::UnknownCharacter(c) => write!(f, "Unknown character encountered: '{}'", c),
            ExprError::EmptyExpression => write!(f, "Expression cannot be empty."),
        }
    }
}

/// Returns (precedence, associativity) for supported operators.
fn operator_info(op: char) -> Option<(i32, Associativity)> {
    match op {
        '+' | '-' => Some((1, Associativity::Left)),
        '*' | '/' => Some((2, Associativity::Left)),
        '^'       => Some((3, Associativity::Right)),
        _         => None,
    }
}

/// Tokenizes an raw input string into a sequence of structured Tokens.
fn tokenize(expr: &str) -> Result<Vec<Token>, ExprError> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            c if operator_info(c).is_some() => {
                tokens.push(Token::Operator(c));
                chars.next();
            }
            c if c.is_alphanumeric() || c == '.' => {
                let mut operand = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '.' {
                        operand.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Operand(operand));
            }
            _ => return Err(ExprError::UnknownCharacter(c)),
        }
    }

    if tokens.is_empty() {
        return Err(ExprError::EmptyExpression);
    }

    Ok(tokens)
}

/// Executes the core Dijkstra's Shunting-Yard Algorithm.
fn shunting_yard(tokens: &[Token], is_prefix: bool) -> Result<Vec<Token>, ExprError> {
    let mut output: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(_) => output.push(token.clone()),
            Token::LParen => stack.push(token.clone()),
            Token::RParen => {
                let mut found = false;
                while let Some(top) = stack.pop() {
                    if top == Token::LParen {
                        found = true;
                        break;
                    }
                    output.push(top);
                }
                if !found {
                    return Err(ExprError::MismatchedParenthesis("Missing '('".into()));
                }
            }
            Token::Operator(op) => {
                let (prec, assoc) = operator_info(*op).unwrap();

                while let Some(Token::Operator(top_op)) = stack.last() {
                    let (top_prec, _) = operator_info(*top_op).unwrap();

                    let should_pop = if is_prefix {
                        // Reverse operator precedence logic for prefix right-to-left processing
                        top_prec > prec || (top_prec == prec && assoc == Associativity::Right)
                    } else {
                        // Standard left-to-right processing for postfix
                        top_prec > prec || (top_prec == prec && assoc == Associativity::Left)
                    };

                    if should_pop {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(token.clone());
            }
        }
    }

    while let Some(top) = stack.pop() {
        if top == Token::LParen {
            return Err(ExprError::MismatchedParenthesis("Missing ')'".into()));
        }
        output.push(top);
    }

    Ok(output)
}

pub fn infix_to_postfix(expr: &str) -> Result<String, String> {
    let tokens = tokenize(expr).map_err(|e| e.to_string())?;
    let postfix_tokens = shunting_yard(&tokens, false).map_err(|e| e.to_string())?;

    Ok(format_tokens(&postfix_tokens))
}

pub fn infix_to_prefix(expr: &str) -> Result<String, String> {
    let mut tokens = tokenize(expr).map_err(|e| e.to_string())?;

    // 1. Reverse input token sequence and swap parenthetical meaning
    tokens.reverse();
    for token in &mut tokens {
        match token {
            Token::LParen => *token = Token::RParen,
            Token::RParen => *token = Token::LParen,
            _ => {}
        }
    }

    // 2. Run adjusted Shunting-Yard
    let mut prefix_tokens = shunting_yard(&tokens, true).map_err(|e| e.to_string())?;

    // 3. Reverse output to produce Prefix notation
    prefix_tokens.reverse();

    Ok(format_tokens(&prefix_tokens))
}

fn format_tokens(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|t| match t {
            Token::Operand(s) => s.clone(),
            Token::Operator(c) => c.to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn print_menu() {
    println!("╔══════════════════════════════════════╗");
    println!("║     Infix Expression Converter       ║");
    println!("╠══════════════════════════════════════╣");
    println!("║  1. Convert to Postfix (Infix → RPN) ║");
    println!("║  2. Convert to Prefix  (Infix → PN)  ║");
    println!("║  3. Convert Both                     ║");
    println!("║  4. Exit                             ║");
    println!("╚══════════════════════════════════════╝");
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    loop {
        println!();
        print_menu();
        let choice = prompt("\nSelect an option (1-4): ");

        if choice == "4" {
            println!("Goodbye!");
            break;
        }

        if !matches!(choice.as_str(), "1" | "2" | "3") {
            println!("Invalid option. Please choose 1-4.");
            continue;
        }

        let expr = prompt("Enter infix expression: ");

        if expr.is_empty() {
            println!("Expression cannot be empty.");
            continue;
        }

        println!("\nInput (Infix): {}", expr);
        println!("{}", "─".repeat(40));

        match choice.as_str() {
            "1" => match infix_to_postfix(&expr) {
                Ok(res) => println!("Postfix:       {}", res),
                Err(err) => println!("Error: {}", err),
            },
            "2" => match infix_to_prefix(&expr) {
                Ok(res) => println!("Prefix:        {}", res),
                Err(err) => println!("Error: {}", err),
            },
            "3" => {
                match infix_to_postfix(&expr) {
                    Ok(res) => println!("Postfix:       {}", res),
                    Err(err) => println!("Postfix Error: {}", err),
                }
                match infix_to_prefix(&expr) {
                    Ok(res) => println!("Prefix:        {}", res),
                    Err(err) => println!("Prefix Error:  {}", err),
                }
            }
            _ => {}
        }
    }
}
