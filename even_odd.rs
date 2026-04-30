use std::io;

fn main() {
    loop {
        println!("Enter a number (or type 'exit' to quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        match trimmed.parse::<i32>() {
            Ok(num) => {
                if num % 2 == 0 {
                    println!("{} is an even number", num);
                } else {
                    println!("{} is an odd number", num);
                }
            }
            Err(_) => {
                println!("Invalid input, please enter a valid integer");
            }
        }

        println!();
    }
}
