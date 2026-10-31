use std::io;

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: f64 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    if n > 0.0 {
        println!("Positive");
    } else if n < 0.0 {
        println!("Negative");
    } else {
        println!("Zero");
    }

    if n.fract() == 0.0 {
        let i = n as i64;
        println!("Integer");
        println!("{}", if i % 2 == 0 { "Even" } else { "Odd" });
        println!("Prime: {}", if is_prime(i.abs()) { "Yes" } else { "No" });
    } else {
        println!("Decimal");
        println!("Floor: {}", n.floor());
        println!("Ceil: {}", n.ceil());
        println!("Rounded: {}", n.round());
    }
}
