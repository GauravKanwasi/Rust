use std::io;

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
        props.push(("Property", if i % 2 == 0 { "Even".to_string() } else { "Odd".to_string() }));
        props.push(("Absolute", i.abs().to_string()));

        let is_prime = |x: i64| -> bool {
            if x < 2 { return false; }
            if x == 2 { return true; }
            if x % 2 == 0 { return false; }
            (3..=(x as f64).sqrt() as i64).step_by(2).all(|d| x % d != 0)
        };

        props.push(("Prime", if is_prime(i.abs()) { "Yes".to_string() } else { "No".to_string() }));

        let is_perfect = |x: i64| -> bool {
            if x < 2 { return false; }
            (1..x).filter(|&d| x % d == 0).sum::<i64>() == x
        };

        props.push(("Perfect Number", if is_perfect(i.abs()) { "Yes".to_string() } else { "No".to_string() }));

        let digits: Vec<u32> = i.abs().to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        let n_digits = digits.len() as u32;
        let is_armstrong = digits.iter().map(|&d| (d as i64).pow(n_digits)).sum::<i64>() == i.abs();
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

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    s.trim().to_string()
}

fn main() {
    println!("Enter a number:");
    let input = read_line();

    let number: f64 = match input.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid input. Please enter a numeric value.");
            return;
        }
    };

    let width = 20;
    println!("\n{:-<width$}", "", width = width * 2);
    for (label, value) in classify_number(number) {
        println!("  {:<width$} {}", format!("{}:", label), value, width = width);
    }
    println!("{:-<width$}\n", "", width = width * 2);
}
