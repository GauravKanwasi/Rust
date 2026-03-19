fn main() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    println!("Array elements: {:?}", arr);

    if let Some(first) = arr.first() {
        println!("First element: {}", first);
    }

    if let Some(last) = arr.last() {
        println!("Last element: {}", last);
    }

    println!("Length: {}", arr.len());
    println!("Sum: {}", arr.iter().sum::<i32>());
    println!("Average: {:.1}", arr.iter().sum::<i32>() as f64 / arr.len() as f64);

    if let (Some(min), Some(max)) = (arr.iter().min(), arr.iter().max()) {
        println!("Min: {}, Max: {}", min, max);
    }

    println!("\nIndexed elements:");
    for (i, val) in arr.iter().enumerate() {
        println!("  arr[{}] = {}", i, val);
    }

    let target = 30;
    println!("\nContains {}: {}", target, arr.contains(&target));

    let evens: Vec<i32> = arr.iter().copied().filter(|x| x % 2 == 0).collect();
    println!("Even elements: {:?}", evens);
}
