fn loopj() -> u32 {
    let mut count = 0;

    println!("Starting the loop!");

    let result = loop {
        count += 1;
        println!("Count is: {count}");

        if count == 5 {
            println!("Reached 5, exiting loop.");
            break count * 2;
        }
    };

    println!("Final count: {result}");
    result
}

fn main() {
    let final_result = loopj();
    println!("Returned from loopj: {final_result}");
}
