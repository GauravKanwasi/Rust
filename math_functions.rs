use std::io::{self, Write};

fn get_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(val) => return val,
            Err(_) => println!("Invalid number, try again."),
        }
    }
}

fn get_two_inputs() -> (f64, f64) {
    let a = get_input("Enter first number: ");
    let b = get_input("Enter second number: ");
    (a, b)
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let limit = (n as f64).sqrt() as u64;
    !(3..=limit).step_by(2).any(|i| n % i == 0)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn print_menu() {
    println!("\n╔══════════════════════════════╗");
    println!("║      MATH FUNCTIONS MENU     ║");
    println!("╠══════════════════════════════╣");
    println!("║  1.  Addition                ║");
    println!("║  2.  Subtraction             ║");
    println!("║  3.  Multiplication          ║");
    println!("║  4.  Division                ║");
    println!("║  5.  Modulus                 ║");
    println!("║  6.  Power (a^b)             ║");
    println!("║  7.  Square Root             ║");
    println!("║  8.  Cube Root               ║");
    println!("║  9.  Absolute Value          ║");
    println!("║  10. Floor                   ║");
    println!("║  11. Ceiling                 ║");
    println!("║  12. Round                   ║");
    println!("║  13. Log (base 10)           ║");
    println!("║  14. Natural Log (ln)        ║");
    println!("║  15. Log (custom base)       ║");
    println!("║  16. Sin                     ║");
    println!("║  17. Cos                     ║");
    println!("║  18. Tan                     ║");
    println!("║  19. Factorial               ║");
    println!("║  20. Is Prime?               ║");
    println!("║  21. GCD                     ║");
    println!("║  22. LCM                     ║");
    println!("║  23. Hypotenuse              ║");
    println!("║  24. Degrees to Radians      ║");
    println!("║  25. Radians to Degrees      ║");
    println!("║  0.  Exit                    ║");
    println!("╚══════════════════════════════╝");
}

fn main() {
    println!("Welcome to the Rust Math Functions Calculator!");

    loop {
        print_menu();
        print!("\nEnter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(n) => n,
            Err(_) => { println!("Invalid choice."); continue; }
        };

        println!();

        match choice {
            0 => {
                println!("Goodbye!");
                break;
            }
            1 => {
                let (a, b) = get_two_inputs();
                println!("Result: {} + {} = {}", a, b, a + b);
            }
            2 => {
                let (a, b) = get_two_inputs();
                println!("Result: {} - {} = {}", a, b, a - b);
            }
            3 => {
                let (a, b) = get_two_inputs();
                println!("Result: {} × {} = {}", a, b, a * b);
            }
            4 => {
                let (a, b) = get_two_inputs();
                if b == 0.0 {
                    println!("Error: Division by zero!");
                } else {
                    println!("Result: {} ÷ {} = {}", a, b, a / b);
                }
            }
            5 => {
                let (a, b) = get_two_inputs();
                if b == 0.0 {
                    println!("Error: Modulus by zero!");
                } else {
                    println!("Result: {} % {} = {}", a, b, a % b);
                }
            }
            6 => {
                let (a, b) = get_two_inputs();
                println!("Result: {}^{} = {}", a, b, a.powf(b));
            }
            7 => {
                let a = get_input("Enter number: ");
                if a < 0.0 {
                    println!("Error: Square root of negative number!");
                } else {
                    println!("Result: √{} = {}", a, a.sqrt());
                }
            }
            8 => {
                let a = get_input("Enter number: ");
                println!("Result: ∛{} = {}", a, a.cbrt());
            }
            9 => {
                let a = get_input("Enter number: ");
                println!("Result: |{}| = {}", a, a.abs());
            }
            10 => {
                let a = get_input("Enter number: ");
                println!("Result: floor({}) = {}", a, a.floor());
            }
            11 => {
                let a = get_input("Enter number: ");
                println!("Result: ceil({}) = {}", a, a.ceil());
            }
            12 => {
                let a = get_input("Enter number: ");
                println!("Result: round({}) = {}", a, a.round());
            }
            13 => {
                let a = get_input("Enter number: ");
                if a <= 0.0 {
                    println!("Error: log10 undefined for <= 0!");
                } else {
                    println!("Result: log10({}) = {}", a, a.log10());
                }
            }
            14 => {
                let a = get_input("Enter number: ");
                if a <= 0.0 {
                    println!("Error: ln undefined for <= 0!");
                } else {
                    println!("Result: ln({}) = {}", a, a.ln());
                }
            }
            15 => {
                let a = get_input("Enter number: ");
                let base = get_input("Enter base: ");
                if a <= 0.0 || base <= 0.0 || base == 1.0 {
                    println!("Error: Invalid input for logarithm!");
                } else {
                    println!("Result: log_{}({}) = {}", base, a, a.log(base));
                }
            }
            16 => {
                let a = get_input("Enter angle in degrees: ");
                println!("Result: sin({}°) = {}", a, a.to_radians().sin());
            }
            17 => {
                let a = get_input("Enter angle in degrees: ");
                println!("Result: cos({}°) = {}", a, a.to_radians().cos());
            }
            18 => {
                let a = get_input("Enter angle in degrees: ");
                println!("Result: tan({}°) = {}", a, a.to_radians().tan());
            }
            19 => {
                let a = get_input("Enter a non-negative integer: ");
                if a < 0.0 || a.fract() != 0.0 || a > 20.0 {
                    println!("Error: Enter an integer between 0 and 20.");
                } else {
                    println!("Result: {}! = {}", a as u64, factorial(a as u64));
                }
            }
            20 => {
                let a = get_input("Enter a positive integer: ");
                if a < 0.0 || a.fract() != 0.0 {
                    println!("Error: Enter a positive integer.");
                } else {
                    let n = a as u64;
                    println!("Result: {} is {}prime", n, if is_prime(n) { "" } else { "not " });
                }
            }
            21 => {
                let (a, b) = get_two_inputs();
                if a < 0.0 || b < 0.0 || a.fract() != 0.0 || b.fract() != 0.0 {
                    println!("Error: Enter positive integers.");
                } else {
                    println!("Result: GCD({}, {}) = {}", a as u64, b as u64, gcd(a as u64, b as u64));
                }
            }
            22 => {
                let (a, b) = get_two_inputs();
                if a < 0.0 || b < 0.0 || a.fract() != 0.0 || b.fract() != 0.0 {
                    println!("Error: Enter positive integers.");
                } else {
                    println!("Result: LCM({}, {}) = {}", a as u64, b as u64, lcm(a as u64, b as u64));
                }
            }
            23 => {
                let (a, b) = get_two_inputs();
                println!("Result: hypotenuse({}, {}) = {}", a, b, a.hypot(b));
            }
            24 => {
                let a = get_input("Enter degrees: ");
                println!("Result: {}° = {} radians", a, a.to_radians());
            }
            25 => {
                let a = get_input("Enter radians: ");
                println!("Result: {} radians = {}°", a, a.to_degrees());
            }
            _ => println!("Invalid choice, please select from the menu."),
        }
    }
}
