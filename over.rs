use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
struct Wrapper(String);

impl Wrapper {
    fn new(s: &str) -> Self {
        Wrapper(s.to_string())
    }
}

impl Add for Wrapper {
    type Output = Wrapper;
    fn add(self, rhs: Wrapper) -> Wrapper {
        Wrapper(self.0 + &rhs.0)
    }
}

impl Sub for Wrapper {
    type Output = Wrapper;
    fn sub(self, rhs: Wrapper) -> Wrapper {
        if rhs.0.is_empty() {
            self
        } else {
            Wrapper(self.0.replace(&rhs.0, ""))
        }
    }
}

impl Mul<usize> for Wrapper {
    type Output = Wrapper;
    fn mul(self, rhs: usize) -> Wrapper {
        Wrapper(self.0.repeat(rhs))
    }
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn sub<T: Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

fn mul<T: Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

fn main() {
    println!("=== Generic Calculator ===\n");

    println!("Integer addition:       {}", add(50, 100));
    println!("Integer subtraction:    {}", sub(100, 50));
    println!("Integer multiplication: {}", mul(10, 5));

    println!();

    println!("Float addition:         {}", add(10.5, 20.5));
    println!("Float subtraction:      {}", sub(30.5, 10.5));
    println!("Float multiplication:   {}", mul(4.5, 2.0));

    println!();

    let s1 = Wrapper::new("Generic ");
    let s2 = Wrapper::new("Programming");

    println!("String concatenation:   {}", add(s1.clone(), s2.clone()));
    println!("String subtraction:     {}", sub(s1, s2));
    println!("String repetition:      {}", Wrapper::new("Rust ") * 3);
}
