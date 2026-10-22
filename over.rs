use std::borrow::Borrow;
use std::fmt;
use std::ops::{Add, AddAssign, Deref, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Wrapper(String);

impl Wrapper {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    fn concat(&self, rhs: &str) -> Self {
        let mut out = String::with_capacity(self.0.len() + rhs.len());
        out.push_str(&self.0);
        out.push_str(rhs);
        Self(out)
    }

    fn remove(&self, pat: &str) -> Self {
        Self(self.0.replace(pat, ""))
    }

    fn repeated(&self, n: usize) -> Self {
        Self(self.0.repeat(n))
    }
}

impl Deref for Wrapper {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Wrapper {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Wrapper {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Wrapper {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for Wrapper {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Wrapper> for String {
    fn from(w: Wrapper) -> Self {
        w.0
    }
}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

macro_rules! forward_binop {
    ($Trait:ident, $method:ident, $helper:ident) => {
        impl $Trait<&Wrapper> for &Wrapper {
            type Output = Wrapper;
            fn $method(self, rhs: &Wrapper) -> Wrapper {
                self.$helper(&rhs.0)
            }
        }

        impl $Trait<Wrapper> for &Wrapper {
            type Output = Wrapper;
            fn $method(self, rhs: Wrapper) -> Wrapper {
                self.$helper(&rhs.0)
            }
        }

        impl $Trait<&Wrapper> for Wrapper {
            type Output = Wrapper;
            fn $method(self, rhs: &Wrapper) -> Wrapper {
                self.$helper(&rhs.0)
            }
        }

        impl $Trait<Wrapper> for Wrapper {
            type Output = Wrapper;
            fn $method(self, rhs: Wrapper) -> Wrapper {
                self.$helper(&rhs.0)
            }
        }

        impl $Trait<&str> for &Wrapper {
            type Output = Wrapper;
            fn $method(self, rhs: &str) -> Wrapper {
                self.$helper(rhs)
            }
        }

        impl $Trait<&str> for Wrapper {
            type Output = Wrapper;
            fn $method(self, rhs: &str) -> Wrapper {
                self.$helper(rhs)
            }
        }
    };
}

forward_binop!(Add, add, concat);
forward_binop!(Sub, sub, remove);

impl Mul<usize> for &Wrapper {
    type Output = Wrapper;

    fn mul(self, rhs: usize) -> Wrapper {
        self.repeated(rhs)
    }
}

impl Mul<usize> for Wrapper {
    type Output = Wrapper;

    fn mul(self, rhs: usize) -> Wrapper {
        self.repeated(rhs)
    }
}

impl AddAssign<&str> for Wrapper {
    fn add_assign(&mut self, rhs: &str) {
        self.0.push_str(rhs);
    }
}

impl AddAssign<&Wrapper> for Wrapper {
    fn add_assign(&mut self, rhs: &Wrapper) {
        self.0.push_str(&rhs.0);
    }
}

impl AddAssign<Wrapper> for Wrapper {
    fn add_assign(&mut self, rhs: Wrapper) {
        self.0.push_str(&rhs.0);
    }
}

impl SubAssign<&str> for Wrapper {
    fn sub_assign(&mut self, rhs: &str) {
        self.0 = self.0.replace(rhs, "");
    }
}

impl SubAssign<&Wrapper> for Wrapper {
    fn sub_assign(&mut self, rhs: &Wrapper) {
        self.0 = self.0.replace(&rhs.0, "");
    }
}

impl SubAssign<Wrapper> for Wrapper {
    fn sub_assign(&mut self, rhs: Wrapper) {
        self.0 = self.0.replace(&rhs.0, "");
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
    acc -= "Generic ";
    acc *= 2;
    println!("Compound assignment:    {}", acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_all_forms() {
        let a = Wrapper::new("ab");
        let b = Wrapper::new("cd");
        assert_eq!((&a + &b).as_ref() as &str, "abcd");
        assert_eq!((a.clone() + &b).into_inner(), "abcd");
        assert_eq!((&a + "cd").into_inner(), "abcd");
    }

    #[test]
    fn sub_removes_all_occurrences() {
        let a = Wrapper::new("abcabc");
        assert_eq!((&a - "b").into_inner(), "acac");
        assert_eq!((&a - "").into_inner(), "abcabc");
    }

    #[test]
    fn mul_and_assign_ops() {
        let mut w = Wrapper::new("x");
        w *= 3;
        w += "y";
        w -= "x";
        assert_eq!(&*w, "y");
        assert_eq!((Wrapper::new("ab") * 0).into_inner(), "");
    }

    #[test]
    fn deref_gives_str_methods() {
        let w = Wrapper::new("hello");
        assert_eq!(w.len(), 5);
        assert!(w.starts_with("he"));
    }
}
