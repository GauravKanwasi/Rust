// Define a trait for the "overloaded" behavior
trait Calculator {
    fn add(&self, other: Self) -> Self;
}

// Implement the trait for integers
impl Calculator for i32 {
    fn add(&self, other: i32) -> i32 {
        self + other
    }
}

// Implement the same trait for strings to concatenate them
impl Calculator for String {
    fn add(&self, other: String) -> String {
        format!("{}{}", self, other)
    }
}

// A generic function that uses the "overloaded" method
fn combine<T: Calculator>(a: T, b: T) -> T {
    a.add(b)
}

fn main() {
    // Overloading behavior with Integers
    let num1: i32 = 10;
    let num2: i32 = 20;
    println!("Integer Addition: {}", combine(num1, num2));

    // Overloading behavior with Strings
    let str1 = String::from("Hello, ");
    let str2 = String::from("Rust!");
    println!("String Concatenation: {}", combine(str1, str2));
}
