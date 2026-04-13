use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

trait ReadWrite {
    fn write(&mut self, index: usize, value: u8) -> Result<(), BufferError>;
    fn read(&self, index: usize) -> Result<u8, BufferError>;
}

trait Resizable {
    fn resize(&mut self, new_size: usize);
    fn capacity(&self) -> usize;
}

#[derive(Debug)]
enum BufferError {
    OutOfBounds { index: usize, len: usize },
    BorrowError(String),
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferError::OutOfBounds { index, len } =>
                write!(f, "Index {} out of bounds for buffer of length {}", index, len),
            BufferError::BorrowError(msg) =>
                write!(f, "Borrow error: {}", msg),
        }
    }
}

#[derive(Debug)]
struct Buffer {
    label: String,
    data: Vec<u8>,
}

impl Buffer {
    fn new(label: impl Into<String>, size: usize) -> Self {
        let label = label.into();
        println!("[{}] Allocating buffer of size {}", label, size);
        Buffer { label, data: vec![0; size] }
    }

    fn fill(&mut self, value: u8) {
        self.data.iter_mut().for_each(|b| *b = value);
    }

    fn as_slice(&self) -> &[u8] {
        &self.data
    }

    fn iter(&self) -> impl Iterator<Item = &u8> {
        self.data.iter()
    }

    fn checksum(&self) -> u64 {
        self.data.iter().map(|&b| b as u64).sum()
    }
}

impl ReadWrite for Buffer {
    fn write(&mut self, index: usize, value: u8) -> Result<(), BufferError> {
        self.data.get_mut(index)
            .map(|slot| *slot = value)
            .ok_or(BufferError::OutOfBounds { index, len: self.data.len() })
    }

    fn read(&self, index: usize) -> Result<u8, BufferError> {
        self.data.get(index)
            .copied()
            .ok_or(BufferError::OutOfBounds { index, len: self.data.len() })
    }
}

impl Resizable for Buffer {
    fn resize(&mut self, new_size: usize) {
        println!("[{}] Resizing buffer: {} → {}", self.label, self.data.len(), new_size);
        self.data.resize(new_size, 0);
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Buffer('{}', len={}, checksum={})", self.label, self.data.len(), self.checksum())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!("[{}] Releasing buffer of size {}", self.label, self.data.len());
    }
}

type SharedVec = Rc<RefCell<Vec<i32>>>;

struct SharedBuffer {
    inner: SharedVec,
    name: String,
}

impl SharedBuffer {
    fn new(name: impl Into<String>, data: Vec<i32>) -> Self {
        SharedBuffer {
            inner: Rc::new(RefCell::new(data)),
            name: name.into(),
        }
    }

    fn clone_handle(&self, name: impl Into<String>) -> Self {
        SharedBuffer {
            inner: Rc::clone(&self.inner),
            name: name.into(),
        }
    }

    fn push(&self, value: i32) {
        self.inner.borrow_mut().push(value);
        println!("[{}] Pushed {}", self.name, value);
    }

    fn sum(&self) -> i32 {
        self.inner.borrow().iter().sum()
    }

    fn ref_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

impl fmt::Display for SharedBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SharedBuffer('{}') = {:?}", self.name, self.inner.borrow())
    }
}

fn ownership_demo() {
    println!("\n=== Ownership & Drop Demo ===");

    let mut buf = Buffer::new("main", 5);
    buf.fill(7);
    println!("{}", buf);

    buf.write(2, 99).expect("write failed");
    buf.write(4, 42).expect("write failed");

    println!("Read [2]: {}", buf.read(2).unwrap());
    println!("Read [4]: {}", buf.read(4).unwrap());
    println!("Checksum: {}", buf.checksum());

    match buf.read(10) {
        Ok(v)  => println!("Value: {}", v),
        Err(e) => println!("Expected error — {}", e),
    }

    buf.resize(8);
    println!("After resize: {:?}", buf.as_slice());
    println!("All bytes: {:?}", buf.iter().collect::<Vec<_>>());
}

fn borrow_demo() {
    println!("\n=== Borrowing & Slices Demo ===");

    let mut numbers = vec![10, 20, 30, 40];

    let snapshot: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled snapshot: {:?}", snapshot);

    {
        let r = &mut numbers;
        r.push(50);
        r.iter_mut().for_each(|x| *x += 1);
        println!("After mutation: {:?}", r);
    }

    let slice = &numbers[1..4];
    println!("Slice [1..4]: {:?}", slice);

    let sum: i32 = slice.iter().sum();
    println!("Slice sum: {}", sum);

    match numbers.as_slice() {
        [first, .., last] => println!("First: {}, Last: {}", first, last),
        _ => println!("Empty"),
    }
}

fn shared_memory_demo() {
    println!("\n=== Shared Ownership Demo (Rc + RefCell) ===");

    let owner = SharedBuffer::new("owner", vec![1, 2, 3]);
    println!("Initial: {}", owner);

    {
        let worker_a = owner.clone_handle("worker-A");
        worker_a.push(4);
        println!("Ref count inside scope: {}", worker_a.ref_count());
    }

    {
        let worker_b = owner.clone_handle("worker-B");
        worker_b.push(5);
        println!("{}", worker_b);
        println!("Sum: {}", worker_b.sum());
    }

    println!("Final owner view: {}", owner);
    println!("Ref count after scopes: {}", owner.ref_count());
}

fn main() {
    ownership_demo();
    borrow_demo();
    shared_memory_demo();
}
