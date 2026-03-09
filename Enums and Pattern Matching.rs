use std::fmt;

#[derive(Debug, Clone)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Echo(String),
    Resize { width: u32, height: u32 },
    Color(u8, u8, u8),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Quit => write!(f, "[QUIT] Exiting application"),
            Message::Move { x, y } => write!(f, "[MOVE] Navigating to ({}, {})", x, y),
            Message::Echo(text) => write!(f, "[ECHO] {}", text),
            Message::Resize { width, height } => write!(f, "[RESIZE] {}x{}", width, height),
            Message::Color(r, g, b) => write!(f, "[COLOR] rgb({}, {}, {})", r, g, b),
        }
    }
}

fn process(msg: Message) {
    println!("{}", msg);
}

fn process_batch(messages: Vec<Message>) {
    messages.into_iter().for_each(process);
}

fn main() {
    let messages = vec![
        Message::Move { x: 3, y: 4 },
        Message::Echo("Hello".to_string()),
        Message::Resize { width: 1920, height: 1080 },
        Message::Color(255, 128, 0),
        Message::Quit,
    ];

    process_batch(messages);
}
