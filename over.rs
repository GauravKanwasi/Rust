use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Wrapper(String);

impl Wrapper {
    pub fn new(s: &str) -> Self {
        Wrapper(s.to_string())
    }
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// a + b  -> concatenate
impl Add<&Wrapper> for &Wrapper {
    type Output = Wrapper;
    fn add(self, rhs: &Wrapper) -> Wrapper {
        Wrapper(format!("{}{}", self.0, rhs.0))
    }
}

// a - b  -> remove every occurrence of b from a
impl Sub<&Wrapper> for &Wrapper {
    type Output = Wrapper;
    fn sub(self, rhs: &Wrapper) -> Wrapper {
        Wrapper(self.0.replace(&rhs.0, ""))
    }
}

// a * n  -> repeat n times
impl Mul<usize> for &Wrapper {
    type Output = Wrapper;
    fn mul(self, n: usize) -> Wrapper {
        Wrapper(self.0.repeat(n))
    }
}

impl AddAssign<&Wrapper> for Wrapper {
    fn add_assign(&mut self, rhs: &Wrapper) {
        self.0.push_str(&rhs.0);
    }
}

impl SubAssign<&Wrapper> for Wrapper {
    fn sub_assign(&mut self, rhs: &Wrapper) {
        self.0 = self.0.replace(&rhs.0, "");
    }
}

impl MulAssign<usize> for Wrapper {
    fn mul_assign(&mut self, n: usize) {
        self.0 = self.0.repeat(n);
    }
}

fn add<A: Add<B>, B>(a: A, b: B) -> A::Output {
    a + b
}

fn sub<A: Sub<B>, B>(a: A, b: B) -> A::Output {
    a - b
}

fn mul<A: Mul<B>, B>(a: A, b: B) -> A::Output {
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

    println!("String concatenation:   {}", add(&s1, &s2));
    println!("String subtraction:     {}", sub(&s1, &s2));
    println!("String repetition:      {}", mul(&Wrapper::new("Rust "), 3));

    println!();

    let mut acc = Wrapper::default();
    acc += &s1;
    acc += &s2;
    acc -= &s1;
    acc *= 2;
    println!("Compound assignment:    {}", acc);
}
