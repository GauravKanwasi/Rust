use std::io::{self, Write};

fn main() -> io::Result<()> {
    print!("Hello! Please tell me your name: ");
    io::stdout().flush()?;

    let mut user_name = String::with_capacity(32);
    io::stdin().read_line(&mut user_name)?;

    let user_name = user_name.trim();

    if user_name.is_empty() {
        println!("It's great to meet you, stranger!");
    } else {
        println!("It's great to meet you, {}!", user_name);
    }

    Ok(())
}
