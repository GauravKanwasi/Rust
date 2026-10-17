use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

trait AdvancedCalculator:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Sized
{
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

impl AdvancedCalculator for i32 {}
impl AdvancedCalculator for f64 {}

#[derive(Clone)]
struct Wrapper(String);

impl Add for Wrapper {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Wrapper(format!("{}{}", self.0, rhs.0))
    }
}

impl Sub for Wrapper {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Wrapper(self.0.replace(&rhs.0, ""))
    }
}

impl Mul for Wrapper {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Wrapper(format!("{}{}", self.0, rhs.0))
    }
}

impl Mul<usize> for Wrapper {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Wrapper(self.0.repeat(rhs))
    }
}

impl AdvancedCalculator for Wrapper {}

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn execute_addition<T: AdvancedCalculator>(a: T, b: T) -> T {
    a.combine(b)
}

fn execute_subtraction<T: AdvancedCalculator>(a: T, b: T) -> T {
    a.subtract(b)
}

fn execute_multiplication<T: AdvancedCalculator>(a: T, b: T) -> T {
    a.multiply(b)
}

fn display_result<T: Display>(label: &str, value: T) {
    println!("{:<18} {}", label, value);
}

fn main() {
    println!("=== Advanced Generic Calculator ===\n");

    display_result("Integer addition:", execute_addition(50, 100));
    display_result("Integer subtraction:", execute_subtraction(100, 50));
    display_result("Integer multiplication:", execute_multiplication(10, 5));

    println!();

    display_result("Float addition:", execute_addition(10.5, 20.5));
    display_result("Float subtraction:", execute_subtraction(30.5, 10.5));
    display_result("Float multiplication:", execute_multiplication(4.5, 2.0));

    println!();

    let s1 = Wrapper("Generic ".to_string());
    let s2 = Wrapper("Programming".to_string());

    display_result("String concatenation:", execute_addition(s1.clone(), s2.clone()));
    display_result("String subtraction:", execute_subtraction(s1, s2));
    display_result("String repetition:", Wrapper("Rust ".to_string()) * 3);
}
