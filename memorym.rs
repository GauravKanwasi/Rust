use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum BufferError {
    OutOfBounds { index: usize, len: usize },
    Borrow(String),
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds { index, len } => {
                write!(f, "index {index} out of bounds for buffer of length {len}")
            }
            Self::Borrow(msg) => write!(f, "borrow error: {msg}"),
        }
    }
}

impl Error for BufferError {}

trait ReadWrite {
    fn write(&mut self, index: usize, value: u8) -> Result<(), BufferError>;
    fn read(&self, index: usize) -> Result<u8, BufferError>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

trait Resizable {
    fn resize(&mut self, new_size: usize);
}

#[derive(Debug)]
struct Buffer {
    label: String,
    data: Vec<u8>,
}

impl Buffer {
    fn new(label: impl Into<String>, size: usize) -> Self {
        let label = label.into();
        println!("[{label}] Allocating buffer of size {size}");
        Self {
            label,
            data: vec![0; size],
        }
    }

    fn filled(label: impl Into<String>, size: usize, value: u8) -> Self {
        let mut buffer = Self::new(label, size);
        buffer.fill(value);
        buffer
    }

    fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }

    fn as_slice(&self) -> &[u8] {
        &self.data
    }

    fn iter(&self) -> impl Iterator<Item = &u8> {
        self.data.iter()
    }

    fn checksum(&self) -> u64 {
        self.data.iter().map(|&b| u64::from(b)).sum()
    }
}

impl ReadWrite for Buffer {
    fn write(&mut self, index: usize, value: u8) -> Result<(), BufferError> {
        let len = self.data.len();
        let slot = self
            .data
            .get_mut(index)
            .ok_or(BufferError::OutOfBounds { index, len })?;
        *slot = value;
        Ok(())
    }

    fn read(&self, index: usize) -> Result<u8, BufferError> {
        self.data
            .get(index)
            .copied()
            .ok_or(BufferError::OutOfBounds {
                index,
                len: self.data.len(),
            })
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl Resizable for Buffer {
    fn resize(&mut self, new_size: usize) {
        println!(
            "[{}] Resizing buffer: {} -> {}",
            self.label,
            self.data.len(),
            new_size
        );
        self.data.resize(new_size, 0);
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Buffer('{}', len={}, checksum={})",
            self.label,
            self.data.len(),
            self.checksum()
        )
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        println!(
            "[{}] Releasing buffer of size {}",
            self.label,
            self.data.len()
        );
    }
}

type SharedVec = Rc<RefCell<Vec<i32>>>;

struct SharedBuffer {
    inner: SharedVec,
    name: String,
}

impl SharedBuffer {
    fn new(name: impl Into<String>, data: Vec<i32>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(data)),
            name: name.into(),
        }
    }

    fn clone_handle(&self, name: impl Into<String>) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
            name: name.into(),
        }
    }

    fn push(&self, value: i32) -> Result<(), BufferError> {
        self.inner
            .try_borrow_mut()
            .map_err(|e| BufferError::Borrow(e.to_string()))?
            .push(value);
        println!("[{}] Pushed {value}", self.name);
        Ok(())
    }

    fn sum(&self) -> Result<i32, BufferError> {
        let data = self
            .inner
            .try_borrow()
            .map_err(|e| BufferError::Borrow(e.to_string()))?;
        Ok(data.iter().sum())
    }

    fn ref_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

impl fmt::Display for SharedBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.try_borrow() {
            Ok(data) => write!(f, "SharedBuffer('{}') = {:?}", self.name, data),
            Err(_) => write!(f, "SharedBuffer('{}') = <mutably borrowed>", self.name),
        }
    }
}

fn ownership_demo() -> Result<(), Box<dyn Error>> {
    println!("\n=== Ownership & Drop Demo ===");

    let mut buf = Buffer::filled("main", 5, 7);
    println!("{buf}");

    buf.write(2, 99)?;
    buf.write(4, 42)?;

    println!("Read [2]: {}", buf.read(2)?);
    println!("Read [4]: {}", buf.read(4)?);
    println!("Checksum: {}", buf.checksum());

    match buf.read(10) {
        Ok(v) => println!("Value: {v}"),
        Err(e) => println!("Expected error - {e}"),
    }

    buf.resize(8);
    println!("After resize: {:?}", buf.as_slice());
    println!("All bytes: {:?}", buf.iter().collect::<Vec<_>>());
    println!("Empty? {}", buf.is_empty());

    Ok(())
}

fn borrow_demo() {
    println!("\n=== Borrowing & Slices Demo ===");

    let mut numbers = vec![10, 20, 30, 40];

    let snapshot: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled snapshot: {snapshot:?}");

    numbers.push(50);
    numbers.iter_mut().for_each(|x| *x += 1);
    println!("After mutation: {numbers:?}");

    let slice = &numbers[1..4];
    println!("Slice [1..4]: {slice:?}");
    println!("Slice sum: {}", slice.iter().sum::<i32>());

    if let [first, .., last] = numbers.as_slice() {
        println!("First: {first}, Last: {last}");
    }
}

fn shared_memory_demo() -> Result<(), Box<dyn Error>> {
    println!("\n=== Shared Ownership Demo (Rc + RefCell) ===");

    let owner = SharedBuffer::new("owner", vec![1, 2, 3]);
    println!("Initial: {owner}");

    {
        let worker_a = owner.clone_handle("worker-A");
        worker_a.push(4)?;
        println!("Ref count inside scope: {}", worker_a.ref_count());
    }

    {
        let worker_b = owner.clone_handle("worker-B");
        worker_b.push(5)?;
        println!("{worker_b}");
        println!("Sum: {}", worker_b.sum()?);
    }

    {
        let guard = owner.inner.borrow();
        println!("Holding a read borrow of {} items", guard.len());
        if let Err(e) = owner.push(6) {
            println!("Expected error - {e}");
        }
    }

    println!("Final owner view: {owner}");
    println!("Ref count after scopes: {}", owner.ref_count());

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    ownership_demo()?;
    borrow_demo();
    shared_memory_demo()?;
    Ok(())
}
