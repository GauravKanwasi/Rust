use std::io::{self, Write};

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '^')
}

fn infix_to_postfix(expr: &str) -> Result<String, String> {
    let mut output: Vec<String> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c == ' ' {
            chars.next();
            continue;
        }

        if c.is_alphanumeric() || c == '.' {
            let mut token = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '.' {
                    token.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            output.push(token);
        } else if c == '(' {
            stack.push(c);
            chars.next();
        } else if c == ')' {
            chars.next();
            let mut found = false;
            while let Some(top) = stack.pop() {
                if top == '(' {
                    found = true;
                    break;
                }
                output.push(top.to_string());
            }
            if !found {
                return Err("Mismatched parentheses: missing '('".to_string());
            }
        } else if is_operator(c) {
            while let Some(&top) = stack.last() {
                if top != '(' && (precedence(top) > precedence(c) || (precedence(top) == precedence(c) && c != '^')) {
                    output.push(top.to_string());
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(c);
            chars.next();
        } else {
            return Err(format!("Unknown character: '{}'", c));
        }
    }

    while let Some(top) = stack.pop() {
        if top == '(' {
            return Err("Mismatched parentheses: missing ')'".to_string());
        }
        output.push(top.to_string());
    }

    Ok(output.join(" "))
}

fn infix_to_prefix(expr: &str) -> Result<String, String> {
    let reversed: String = expr
        .chars()
        .rev()
        .map(|c| match c {
            '(' => ')',
            ')' => '(',
            other => other,
        })
        .collect();

    let postfix = infix_to_postfix_right_assoc(&reversed)?;

    let result: String = postfix
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ");

    Ok(result)
}

fn infix_to_postfix_right_assoc(expr: &str) -> Result<String, String> {
    let mut output: Vec<String> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c == ' ' {
            chars.next();
            continue;
        }

        if c.is_alphanumeric() || c == '.' {
            let mut token = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '.' {
                    token.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            output.push(token);
        } else if c == '(' {
            stack.push(c);
            chars.next();
        } else if c == ')' {
            chars.next();
            let mut found = false;
            while let Some(top) = stack.pop() {
                if top == '(' {
                    found = true;
                    break;
                }
                output.push(top.to_string());
            }
            if !found {
                return Err("Mismatched parentheses".to_string());
            }
        } else if is_operator(c) {
            while let Some(&top) = stack.last() {
                if top != '(' && precedence(top) > precedence(c) {
                    output.push(top.to_string());
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(c);
            chars.next();
        } else {
            return Err(format!("Unknown character: '{}'", c));
        }
    }

    while let Some(top) = stack.pop() {
        if top == '(' {
            return Err("Mismatched parentheses".to_string());
        }
        output.push(top.to_string());
    }

    Ok(output.join(" "))
}

fn print_menu() {
    println!("╔══════════════════════════════════════╗");
    println!("║     Infix Expression Converter        ║");
    println!("╠══════════════════════════════════════╣");
    println!("║  1. Convert to Postfix (Infix → RPN)  ║");
    println!("║  2. Convert to Prefix  (Infix → PN)   ║");
    println!("║  3. Convert Both                      ║");
    println!("║  4. Exit                              ║");
    println!("╚══════════════════════════════════════╝");
}

fn main() {
    loop {
        println!();
        print_menu();
        print!("\nSelect an option (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice == "4" {
            println!("Goodbye!");
            break;
        }

        if !matches!(choice, "1" | "2" | "3") {
            println!("Invalid option. Please choose 1-4.");
            continue;
        }

        print!("Enter infix expression: ");
        io::stdout().flush().unwrap();

        let mut expr = String::new();
        io::stdin().read_line(&mut expr).unwrap();
        let expr = expr.trim();

        if expr.is_empty() {
            println!("Expression cannot be empty.");
            continue;
        }

        println!("\nInput (Infix): {}", expr);
        println!("{}", "─".repeat(40));

        match choice {
            "1" => match infix_to_postfix(expr) {
                Ok(result) => println!("Postfix:       {}", result),
                Err(e) => println!("Error: {}", e),
            },
            "2" => match infix_to_prefix(expr) {
                Ok(result) => println!("Prefix:        {}", result),
                Err(e) => println!("Error: {}", e),
            },
            "3" => {
                match infix_to_postfix(expr) {
                    Ok(result) => println!("Postfix:       {}", result),
                    Err(e) => println!("Postfix Error: {}", e),
                }
                match infix_to_prefix(expr) {
                    Ok(result) => println!("Prefix:        {}", result),
                    Err(e) => println!("Prefix Error:  {}", e),
                }
            }
            _ => {}
        }
    }
}
