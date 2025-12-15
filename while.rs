fn main() {
    // Initialize a mutable variable for the counter
    let mut counter = 1;

    // The 'while' loop continues as long as the condition (counter <= 5) is true
    while counter <= 5 {
        println!("Current count: {}", counter);

        // Increment the counter to eventually break the loop
        counter = counter + 1;
    }

    // This code runs after the loop has finished
    println!("\nLoop finished! Counter is now: {}", counter);
}
