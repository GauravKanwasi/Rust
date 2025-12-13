fn main() {
    // 1. Array with explicit values (type [i32; 5] inferred by the compiler)
    let numbers = [10, 20, 30, 40, 50];

    // 2. Array using a repeat expression
    let zeros: [f64; 5] = [0.0; 5];

    // --- Accessing Elements ---
    let first_number = numbers[0];
    let fourth_number = numbers[3];

    println!("--- Array Initialization & Access ---");
    println!("'numbers' array: {:?}", numbers);
    println!("'zeros' array:   {:?}", zeros);
    println!("Length of 'numbers': {}", numbers.len());
    println!("First number (index 0): {}", first_number);
    println!("Fourth number (index 3): {}", fourth_number);

    // --- Iterating over an Array ---
    println!("\n--- Iterating with a 'for' loop ---");
    for (index, value) in numbers.iter().enumerate() {
        println!("Element at index {}: {}", index, value);
    }

    // --- Mutability ---
    let mut mutable_array = [1, 2, 3];
    println!("\nOriginal mutable array: {:?}", mutable_array);

    mutable_array[1] = 99; // Change the element at index 1

    println!("Modified mutable array: {:?}", mutable_array);
}
