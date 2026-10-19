use std::fmt::{self, Display};
use std::ops::{Add, Mul, Sub};

const LABEL_WIDTH: usize = 24;

trait Calculator: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Sized {
    fn combine(self, other: Self) -> Self {
        self + other
    }
    fn subtract(self, other: Self) -> Self {
        self - other
    }

    fn multiply(self, other: Self) -> Self {
        self * other
    }
}

impl<T> Calculator for T where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Sized
{
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Wrapper(String);

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

impl Add for Wrapper {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self.0.push_str(&rhs.0);
        self
    }
}

impl Sub for Wrapper {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if rhs.0.is_empty() {
            self
        } else {
            Wrapper(self.0.replace(&rhs.0, ""))
        }
    }
}

impl Mul for Wrapper {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Mul<usize> for Wrapper {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Wrapper(self.0.repeat(rhs))
    }
}

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.0)
    }
}

fn add<T: Calculator>(a: T, b: T) -> T {
    a.combine(b)
}

fn sub<T: Calculator>(a: T, b: T) -> T {
    a.subtract(b)
}

fn mul<T: Calculator>(a: T, b: T) -> T {
    a.multiply(b)
}

fn show(label: &str, value: impl Display) {
    println!("{label:<LABEL_WIDTH$} {value}");
}

fn main() {
    println!("=== Advanced Generic Calculator ===\n");

    show("Integer addition:", add(50, 100));
    show("Integer subtraction:", sub(100, 50));
    show("Integer multiplication:", mul(10, 5));

    println!();

    show("Float addition:", add(10.5, 20.5));
    show("Float subtraction:", sub(30.5, 10.5));
    show("Float multiplication:", mul(4.5, 2.0));

    println!();

    let s1 = Wrapper::from("Generic ");
    let s2 = Wrapper::from("Programming");

    show("String concatenation:", add(s1.clone(), s2.clone()));
    show("String subtraction:", sub(s1, s2));
    show("String repetition:", Wrapper::from("Rust ") * 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers() {
        assert_eq!(add(50, 100), 150);
        assert_eq!(sub(100, 50), 50);
        assert_eq!(mul(10, 5), 50);
    }
    #[test]
    fn floats() {
        assert_eq!(add(10.5, 20.5), 31.0);
        assert_eq!(sub(30.5, 10.5), 20.0);
        assert_eq!(mul(4.5, 2.0), 9.0);
    }
    #[test]
    fn wrapper_ops() {
        let a = Wrapper::from("Generic ");
        let b = Wrapper::from("Programming");
        assert_eq!(add(a.clone(), b.clone()), Wrapper::from("Generic Programming"));
        assert_eq!(sub(a.clone(), b), a);
        assert_eq!(Wrapper::from("ab") * 3, Wrapper::from("ababab"));
        assert_eq!(sub(Wrapper::from("abcabc"), Wrapper::from("")), Wrapper::from("abcabc"));
    }
}
