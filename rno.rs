use rand::Rng;

fn main() {
    let ascii: u8 = rand::thread_rng().gen_range(32..=126);
    println!("Random Number (ASCII): {}", ascii);
    println!("ASCII Character      : {}", ascii as char);
}
