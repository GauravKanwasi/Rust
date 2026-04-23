use std::io;

fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn parse_number(input: &str) -> Result<i64, String> {
    input.parse::<i64>().map_err(|_| format!("'{}' is not a valid integer", input))
}

fn main() {
    loop {
        let input = read_line("Enter a number (or 'quit' to exit):");

        if input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        match parse_number(&input) {
            Ok(number) => {
                println!("┌─────────────────────────────┐");
                println!("│ Number     : {:>15} │", number);
                println!("│ Squared    : {:>15} │", number.pow(2));
                println!("│ Doubled    : {:>15} │", number * 2);
                println!("│ Is Even    : {:>15} │", number % 2 == 0);
                println!("│ Is Positive: {:>15} │", number > 0);
                println!("└─────────────────────────────┘");
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
