use std::fmt;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

/// Severity levels for log-style messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Level {
    Info,
    Warn,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Echo(String),
    Resize { width: u32, height: u32 },
    Color(u8, u8, u8),
    Log { level: Level, text: String },
}

/// Errors that can occur while validating or constructing a `Message`.
#[derive(Debug)]
enum MessageError {
    EmptyEcho,
    ZeroDimension,
    DimensionTooLarge(u32),
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageError::EmptyEcho => write!(f, "echo message cannot be empty"),
            MessageError::ZeroDimension => write!(f, "width and height must both be non-zero"),
            MessageError::DimensionTooLarge(d) => {
                write!(f, "dimension {} exceeds maximum allowed size (8192)", d)
            }
        }
    }
}

impl Error for MessageError {}

impl Message {
    /// Validates a message's invariants before it's processed.
    /// Keeps `process` itself infallible while still catching bad input early.
    fn validate(&self) -> Result<(), MessageError> {
        const MAX_DIMENSION: u32 = 8192;
        match self {
            Message::Echo(text) if text.trim().is_empty() => Err(MessageError::EmptyEcho),
            Message::Resize { width, height } => {
                if *width == 0 || *height == 0 {
                    Err(MessageError::ZeroDimension)
                } else if *width > MAX_DIMENSION || *height > MAX_DIMENSION {
                    Err(MessageError::DimensionTooLarge((*width).max(*height)))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    /// True if this message should stop batch processing (e.g. Quit).
    fn is_terminal(&self) -> bool {
        matches!(self, Message::Quit)
    }

    fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Quit => write!(f, "[QUIT] Exiting application"),
            Message::Move { x, y } => write!(f, "[MOVE] Navigating to ({}, {})", x, y),
            Message::Echo(text) => write!(f, "[ECHO] {}", text),
            Message::Resize { width, height } => write!(f, "[RESIZE] {}x{}", width, height),
            Message::Color(r, g, b) => write!(f, "[COLOR] rgb({}, {}, {})", r, g, b),
            Message::Log { level, text } => write!(f, "[LOG:{}] {}", level, text),
        }
    }
}

/// Processes a single message: validates it, logs it with a timestamp,
/// and reports whether the batch should stop after this one.
fn process(msg: &Message) -> Result<bool, MessageError> {
    msg.validate()?;
    println!("[{}] {}", Message::timestamp(), msg);
    Ok(msg.is_terminal())
}

/// Processes a batch of messages in order, stopping early on a terminal
/// message (e.g. Quit) or the first validation error. Returns how many
/// messages were successfully processed.
fn process_batch(messages: Vec<Message>) -> Result<usize, (usize, MessageError)> {
    let mut processed = 0;
    for msg in &messages {
        match process(msg) {
            Ok(should_stop) => {
                processed += 1;
                if should_stop {
                    break;
                }
            }
            Err(e) => return Err((processed, e)),
        }
    }
    Ok(processed)
}

fn main() {
    let messages = vec![
        Message::Move { x: 3, y: 4 },
        Message::Echo("Hello".to_string()),
        Message::Resize { width: 1920, height: 1080 },
        Message::Color(255, 128, 0),
        Message::Log { level: Level::Warn, text: "low battery".to_string() },
        Message::Quit,
    ];

    match process_batch(messages) {
        Ok(count) => println!("\nProcessed {} message(s) successfully.", count),
        Err((count, err)) => {
            eprintln!(
                "\nStopped after {} message(s): validation error: {}",
                count, err
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_are_correct() {
        assert_eq!(Message::Quit.to_string(), "[QUIT] Exiting application");
        assert_eq!(
            Message::Move { x: 1, y: 2 }.to_string(),
            "[MOVE] Navigating to (1, 2)"
        );
        assert_eq!(
            Message::Color(10, 20, 30).to_string(),
            "[COLOR] rgb(10, 20, 30)"
        );
    }

    #[test]
    fn empty_echo_is_rejected() {
        let msg = Message::Echo("   ".to_string());
        assert!(matches!(msg.validate(), Err(MessageError::EmptyEcho)));
    }

    #[test]
    fn zero_dimension_resize_is_rejected() {
        let msg = Message::Resize { width: 0, height: 100 };
        assert!(matches!(msg.validate(), Err(MessageError::ZeroDimension)));
    }

    #[test]
    fn oversized_resize_is_rejected() {
        let msg = Message::Resize { width: 9000, height: 100 };
        assert!(matches!(
            msg.validate(),
            Err(MessageError::DimensionTooLarge(9000))
        ));
    }

    #[test]
    fn quit_is_terminal() {
        assert!(Message::Quit.is_terminal());
        assert!(!Message::Move { x: 0, y: 0 }.is_terminal());
    }

    #[test]
    fn batch_stops_at_quit() {
        let messages = vec![
            Message::Move { x: 0, y: 0 },
            Message::Quit,
            Message::Move { x: 5, y: 5 }, // should never run
        ];
        let processed = process_batch(messages).unwrap();
        assert_eq!(processed, 2);
    }

    #[test]
    fn batch_stops_on_error_and_reports_progress() {
        let messages = vec![
            Message::Move { x: 0, y: 0 },
            Message::Echo("".to_string()),
            Message::Quit,
        ];
        let result = process_batch(messages);
        assert!(matches!(result, Err((1, MessageError::EmptyEcho))));
    }
}
