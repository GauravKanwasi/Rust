fn loopj() {
    let mut count = 0;

    println!("Starting the loop!");

    let result = loop {
        count += 1;
        println!("Count is: {}", count);

        if count == 5 {
            println!("Reached 5, exiting loop.");
            break count;
        }
    };

    println!("Final count: {}", result);
}
