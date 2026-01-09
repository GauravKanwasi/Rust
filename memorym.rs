
use std::rc::Rc;
use std::cell::RefCell;

// A simple struct to demonstrate ownership and Drop behavior
struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        println!("Allocating buffer of size {}", size);
        Buffer { data: vec![0; size] }
    }

    fn write(&mut self, index: usize, value: u8) {
        if index < self.data.len() {
            self.data[index] = value;
        }
    }

    fn read(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!("Releasing buffer of size {}", self.data.len());
    }
}

// Demonstrates shared ownership using Rc and interior mutability with RefCell
fn shared_memory_demo() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    {
        let s1 = Rc::clone(&shared);
        s1.borrow_mut().push(4);
    }

    {
        let s2 = Rc::clone(&shared);
        println!("Shared Vec: {:?}", s2.borrow());
    }

    println!("Strong reference count = {}", Rc::strong_count(&shared));
}

// Demonstrates borrowing rules and slices
fn borrow_demo() {
    let mut numbers = vec![10, 20, 30, 40];

    {
        let r = &numbers;
        println!("Immutable borrow: {:?}", r);
    }

    {
        let r = &mut numbers;
        r.push(50);
        println!("Mutable borrow: {:?}", r);
    }

    let slice = &numbers[1..4];
    println!("Slice: {:?}", slice);
}

fn main() {
    println!("=== Ownership Demo ===");
    {
        let mut buffer = Buffer::new(5);
        buffer.write(2, 99);
        println!("Read index 2: {:?}", buffer.read(2));
    } // buffer goes out of scope here, Drop is called

    println!("\n=== Borrowing and Slices Demo ===");
    borrow_demo();

    println!("\n=== Shared Ownership Demo (Rc + RefCell) ===");
    shared_memory_demo();
}
