use std::io::{self, Write};

fn classify_number(n: f64) -> Vec<(&'static str, String)> {
    let mut props = vec![
        ("Nature", match n.partial_cmp(&0.0).unwrap() {
            std::cmp::Ordering::Greater => "Positive".to_string(),
            std::cmp::Ordering::Less    => "Negative".to_string(),
            std::cmp::Ordering::Equal   => "Zero".to_string(),
        }),
        ("Type", if n.fract() == 0.0 { "Integer".to_string() } else { "Decimal (floating-point)".to_string() }),
    ];

    if n.fract() == 0.0 {
        let i = n as i64;
        let abs_i = i.abs();

        props.push(("Property", if i % 2 == 0 { "Even".to_string() } else { "Odd".to_string() }));
        props.push(("Absolute", abs_i.to_string()));

        let is_prime = |x: i64| -> bool {
            if x < 2 { return false; }
            if x == 2 { return true; }
            if x % 2 == 0 { return false; }
            (3..=(x as f64).sqrt() as i64).step_by(2).all(|d| x % d != 0)
        };

        props.push(("Prime", if is_prime(abs_i) { "Yes".to_string() } else { "No".to_string() }));

        let is_perfect = |x: i64| -> bool {
            if x < 2 { return false; }
            (1..x).filter(|&d| x % d == 0).sum::<i64>() == x
        };

        props.push(("Perfect Number", if is_perfect(abs_i) { "Yes".to_string() } else { "No".to_string() }));

        let digits: Vec<u32> = abs_i.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let n_digits = digits.len() as u32;
        let is_armstrong = digits.iter().map(|&d| (d as i64).pow(n_digits)).sum::<i64>() == abs_i;
        props.push(("Armstrong", if is_armstrong { "Yes".to_string() } else { "No".to_string() }));

        let digit_sum: u32 = digits.iter().sum();
        props.push(("Digit Sum", digit_sum.to_string()));
        props.push(("Digits", digits.len().to_string()));
    } else {
        props.push(("Absolute", format!("{:.6}", n.abs())));
        props.push(("Floor", (n.floor() as i64).to_string()));
        props.push(("Ceil",  (n.ceil()  as i64).to_string()));
        props.push(("Rounded", (n.round() as i64).to_string()));
    }

    props
}

fn prompt(name: &str) -> io::Result<String> {
    print!("{}", name);
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn main() {
    loop {
        let input = match prompt("Enter a number (or type 'exit' to quit): ") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Error reading input.");
                break;
            }
        };

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        let number: f64 = match input.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("\x1b[31mInvalid input. Please enter a valid numeric value.\x1b[0m\n");
                continue;
            }
        };

        let results = classify_number(number);
        
        // Dynamically compute max label width for aligned formatting
        let max_label_len = results.iter().map(|(l, _)| l.len()).max().unwrap_or(10);
        let border_width = max_label_len + 25;

        println!("\n┌{:─<width$}┐", "", width = border_width);
        println!("│ {:^width$} │", "NUMBER CLASSIFICATION REPORT", width = border_width - 2);
        println!("├{:─<width$}┤", "", width = border_width);

        for (label, value) in results {
            println!("│  {:<width$} : {:<val_width$} │", label, value, width = max_label_len, val_width = border_width - max_label_len - 6);
        }

        println!("└{:─<width$}\n", "", width = border_width);
    }
}
