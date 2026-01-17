use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random printable ASCII number (32–126)
    let ascii: u8 = rng.gen_range(32..=126);
    let character = ascii as char;

    println!("Random Number (ASCII): {}", ascii);
    println!("ASCII Character      : {}", character);
}
