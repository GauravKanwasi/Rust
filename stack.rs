use std::io::{self, Write};

struct Stack {
    data: Vec<i32>,
    capacity: usize,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn push(&mut self, value: i32) {
        if self.is_full() {
            println!("Stack Overflow");
        } else {
            self.data.push(value);
            println!("Pushed {}", value);
        }
    }

    fn pop(&mut self) {
        if self.is_empty() {
            println!("Stack Underflow");
        } else {
            let value = self.data.pop().unwrap();
            println!("Popped {}", value);
        }
    }

    fn peek(&self) {
        if self.is_empty() {
            println!("Stack is Empty");
        } else {
            println!("Top Element {}", self.data[self.data.len() - 1]);
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn is_full(&self) -> bool {
        self.data.len() == self.capacity
    }

    fn size(&self) {
        println!("Stack Size {}", self.data.len());
    }

    fn display(&self) {
        if self.is_empty() {
            println!("Stack is Empty");
        } else {
            for value in self.data.iter().rev() {
                println!("{}", value);
            }
        }
    }
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut stack = Stack::new(5);

    loop {
        println!("\n1 Push\n2 Pop\n3 Peek\n4 Is Empty\n5 Is Full\n6 Size\n7 Display\n8 Exit");
        print!("Enter Choice: ");
        io::stdout().flush().unwrap();

        let choice = read_input();

        match choice {
            1 => {
                print!("Enter Value: ");
                io::stdout().flush().unwrap();
                let value = read_input();
                stack.push(value);
            }
            2 => stack.pop(),
            3 => stack.peek(),
            4 => println!("{}", stack.is_empty()),
            5 => println!("{}", stack.is_full()),
            6 => stack.size(),
            7 => stack.display(),
            8 => break,
            _ => println!("Invalid Choice"),
        }
    }
}
