use std::io;

fn is_sparse(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len().saturating_sub(1) {
        if chars[i] == chars[i + 1] {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();

    println!("Enter a string:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    if is_sparse(input) {
        println!("The string \"{}\" is SPARSE.", input);
    } else {
        println!("The string \"{}\" is NOT sparse.", input);
    }
}
