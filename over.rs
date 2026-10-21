use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Wrapper(String);

impl Wrapper {
    fn new(s: impl Into<String>) -> Self {
        Wrapper(s.into())
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Wrapper {
    fn from(s: &str) -> Self {
        Wrapper(s.to_owned())
    }
}

impl From<String> for Wrapper {
    fn from(s: String) -> Self {
        Wrapper(s)
    }
}

impl AsRef<str> for Wrapper {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Add for Wrapper {
    type Output = Wrapper;

    fn add(mut self, rhs: Wrapper) -> Wrapper {
        self.0.push_str(&rhs.0);
        self
    }
}

impl Add<&Wrapper> for &Wrapper {
    type Output = Wrapper;

    fn add(self, rhs: &Wrapper) -> Wrapper {
        let mut out = String::with_capacity(self.0.len() + rhs.0.len());
        out.push_str(&self.0);
        out.push_str(&rhs.0);
        Wrapper(out)
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

impl Sub<&Wrapper> for &Wrapper {
    type Output = Wrapper;

    fn sub(self, rhs: &Wrapper) -> Wrapper {
        if rhs.0.is_empty() {
            self.clone()
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

impl Mul<usize> for &Wrapper {
    type Output = Wrapper;

    fn mul(self, rhs: usize) -> Wrapper {
        Wrapper(self.0.repeat(rhs))
    }
}

impl AddAssign<&Wrapper> for Wrapper {
    fn add_assign(&mut self, rhs: &Wrapper) {
        self.0.push_str(&rhs.0);
    }
}

impl SubAssign<&Wrapper> for Wrapper {
    fn sub_assign(&mut self, rhs: &Wrapper) {
        if !rhs.0.is_empty() {
            self.0 = self.0.replace(&rhs.0, "");
        }
    }
}

impl MulAssign<usize> for Wrapper {
    fn mul_assign(&mut self, rhs: usize) {
        self.0 = self.0.repeat(rhs);
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
    let s2 = Wrapper::from("Programming");

    println!("String concatenation:   {}", add(&s1, &s2));
    println!("String subtraction:     {}", sub(&s1, &s2));
    println!("String repetition:      {}", mul(&Wrapper::new("Rust "), 3));

    println!();

    let mut acc = Wrapper::default();
    acc += &s1;
    acc += &s2;
    acc -= &Wrapper::new("Generic ");
    acc *= 2;
    println!("Compound assignment:    {}", acc);
}
