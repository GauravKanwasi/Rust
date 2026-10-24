use std::ops::{Add, Mul, Sub};

fn add<A: Add<B>, B>(a: A, b: B) -> A::Output { a + b }
fn sub<A: Sub<B>, B>(a: A, b: B) -> A::Output { a - b }
fn mul<A: Mul<B>, B>(a: A, b: B) -> A::Output { a * b }

fn main() {
    println!("{} {} {}", add(50, 100), sub(100, 50), mul(10, 5));
    println!("{} {} {}", add(10.5, 20.5), sub(30.5, 10.5), mul(4.5, 2.0));
    println!("{}", add("Generic ".to_string(), "Programming"));
}
