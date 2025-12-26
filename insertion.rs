use std::io;

fn insertion_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    println!("Initial array: {:?}", arr);
    println!("--------------------------------");

    for i in 1..n {
        let key = arr[i];
        let mut j = i as isize - 1;

        println!("Step {}:", i);
        println!("Key selected: {}", key);

        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }

        arr[(j + 1) as usize] = key;

        println!("Array after inserting key: {:?}", arr);
        println!("--------------------------------");
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter numbers separated by spaces:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please enter valid integers"))
        .collect();

    insertion_sort(&mut numbers);

    println!("Final sorted array: {:?}", numbers);
}
