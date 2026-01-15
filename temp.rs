use std::io;

fn main() {
    println!("Enter the temperature in Celsius:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let temperature: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature input.");
            return;
        }
    };

    let weather = if temperature < 0.0 {
        "Freezing cold"
    } else if temperature < 10.0 {
        "Very cold"
    } else if temperature < 20.0 {
        "Cool"
    } else if temperature < 30.0 {
        "Warm"
    } else if temperature < 40.0 {
        "Hot"
    } else {
        "Extremely hot"
    };

    println!("Weather condition: {}", weather);
}
