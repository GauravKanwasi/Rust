fn loopj() {
    let mut count = 0;

    println!("Starting the loop!");

    // This is the simplest loop in Rust
    loop {
        count += 1;
        println!("Count is: {}", count);

        if count == 5 {
            println!("Reached 5, breaking out of the loop.");
            break; // This exits the loop
        }
    }
}
