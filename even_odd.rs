use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = input.trim().parse().unwrap();

    if num % 2 == 0 {
        println!("Even number");
    } else {
        println!("Odd number");
    }
}
