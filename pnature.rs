use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a numeric value.");
            return;
        }
    };

    // Sign check
    if number > 0.0 {
        println!("Nature: Positive");
    } else if number < 0.0 {
        println!("Nature: Negative");
    } else {
        println!("Nature: Zero");
    }

    // Integer or decimal check
    if number.fract() == 0.0 {
        println!("Type: Integer");

        let int_number = number as i64;

        if int_number % 2 == 0 {
            println!("Property: Even");
        } else {
            println!("Property: Odd");
        }
    } else {
        println!("Type: Decimal (floating-point)");
    }
}
