
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

#[derive(Debug, Clone, PartialEq)]
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
        let result = self.0.replace(&rhs.0, "");
        Wrapper(result)
    }
}

impl Mul<usize> for Wrapper {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Wrapper(self.0.repeat(rhs))
    }
}

impl AdvancedCalculator for Wrapper {
    fn multiply(self, other: Self) -> Self {
        Wrapper(format!("{}{}", self.0, other.0))
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

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("=== Advanced Generic Calculator ===\n");

    let int_add = execute_addition(50, 100);
    let int_sub = execute_subtraction(100, 50);
    let int_mul = execute_multiplication(10, 5);

    display_result("Integer addition:", int_add);
    display_result("Integer subtraction:", int_sub);
    display_result("Integer multiplication:", int_mul);

    println!();

    let float_add = execute_addition(10.5, 20.5);
    let float_sub = execute_subtraction(30.5, 10.5);
    let float_mul = execute_multiplication(4.5, 2.0);

    display_result("Float addition:", float_add);
    display_result("Float subtraction:", float_sub);
    display_result("Float multiplication:", float_mul);

    println!();

    let s1 = Wrapper("Generic ".to_string());
    let s2 = Wrapper("Programming".to_string());

    let string_add = execute_addition(s1.clone(), s2.clone());
    let string_sub = execute_subtraction(s1.clone(), s2.clone());

    display_result("String concatenation:", string_add);
    display_result("String subtraction:", string_sub);

    let repeated = Wrapper("Rust ".to_string()) * 3;
    display_result("String repetition:", repeated);
}
