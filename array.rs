fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let zeros: [f64; 5] = [0.0; 5];

    let first = numbers[0];
    let fourth = numbers[3];

    println!("--- Array Initialization & Access ---");
    println!("numbers : {:?}", numbers);
    println!("zeros   : {:?}", zeros);
    println!("length  : {}", numbers.len());
    println!("index 0 : {}", first);
    println!("index 3 : {}", fourth);

    println!("\n--- Iterating ---");
    for (i, val) in numbers.iter().enumerate() {
        println!("[{}] = {}", i, val);
    }

    println!("\n--- Mutability ---");
    let mut arr = [1, 2, 3];
    println!("before : {:?}", arr);
    arr[1] = 99;
    println!("after  : {:?}", arr);
}
