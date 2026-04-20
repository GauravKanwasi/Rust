use std::ops::Add;

trait AdvancedCalculator: Add<Output = Self> + Sized {
    fn combine(self, other: Self) -> Self {
        self + other
    }
}

impl AdvancedCalculator for i32 {}
impl AdvancedCalculator for f64 {}

struct Wrapper(String);

impl Add for Wrapper {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Wrapper(format!("{}{}", self.0, rhs.0))
    }
}

impl AdvancedCalculator for Wrapper {}

fn execute_addition<T: AdvancedCalculator>(a: T, b: T) -> T {
    a.combine(b)
}

fn main() {
    let int_res = execute_addition(50, 100);
    let float_res = execute_addition(10.5, 20.5);
    
    let s1 = Wrapper("Generic ".to_string());
    let s2 = Wrapper("Programming".to_string());
    let string_res = execute_addition(s1, s2);

    println!("Integer: {}", int_res);
    println!("Float:   {}", float_res);
    println!("String:  {}", string_res.0);
}
