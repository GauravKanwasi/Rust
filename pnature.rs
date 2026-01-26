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

    if number > 0.0 {
        println!("The number is Positive.");
    } else if number < 0.0 {
        println!("The number is Negative.");
    } else {
        println!("The number is Zero.");
    }
}
