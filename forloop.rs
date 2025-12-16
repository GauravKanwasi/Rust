
fn main() {
    // 1. Iterating over a range of numbers (inclusive start, exclusive end)
    println!("### Iterating over a range (1 to 5) ###");
    for number in 1..6 {
        println!("Current number is: {}", number);
    }

    // 2. Iterating over the elements of an array/vector
    println!("\n### Iterating over a vector ###");
    let programming_languages = vec!["Rust", "Python", "C++", "JavaScript", "Go"];

    for (index, lang) in programming_languages.iter().enumerate() {
        // .enumerate() gives both the index and the value
        println!("Index {}: Language is {}", index, lang);
    }

    // 3. Reverse iteration over a range
    println!("\n### Reverse iteration (counting down) ###");
    // The .rev() adapter reverses the order of the iterator
    for i in (1..4).rev() {
        println!("Countdown: {}", i);
    }
    println!("LIFTOFF!!!");
}
