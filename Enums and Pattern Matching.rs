enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Echo(String),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Exiting"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Echo(text) => println!("Message: {}", text),
    }
}

fn main() {
    process(Message::Move { x: 3, y: 4 });
    process(Message::Echo("Hello".to_string()));
    process(Message::Quit);
}
