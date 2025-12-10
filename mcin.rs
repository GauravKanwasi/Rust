fn mcin() {
    // --- Addition Example (using i32 integers) ---
    let num1: i32 = 15;
    let num2: i32 = 7;

    // Perform the addition
    let sum = num1 + num2;

    // Print the result of the addition
    println!("--- Addition ---");
    println!("The sum of {} and {} is: {}", num1, num2, sum); // Output: 22

    // Add a separator for clarity
    println!("\n---------------------\n");

    // --- Subtraction Example (using f64 floating-point numbers) ---
    let num3: f64 = 45.5; 
    let num4: f64 = 10.2;

    // Perform the subtraction
    let difference = num3 - num4;

    // Print the result of the subtraction
    println!("--- Subtraction ---");
    // :.2 formats the floating-point number to two decimal places
    println!("The difference between {} and {} is: {:.2}", num3, num4, difference); // Output: 35.30
}
