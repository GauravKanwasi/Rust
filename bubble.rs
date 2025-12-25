fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    let mut step = 1;

    println!("Initial array: {:?}\n", arr);

    for i in 0..n {
        println!("Pass {}:", i + 1);

        let mut swapped = false;

        for j in 0..n - i - 1 {
            println!(
                "  Step {}: Compare {} and {}",
                step, arr[j], arr[j + 1]
            );

            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;

                println!(
                    "           Swapped -> {:?}",
                    arr
                );
            } else {
                println!(
                    "           No swap -> {:?}",
                    arr
                );
            }

            step += 1;
        }

        println!("  Result after pass {}: {:?}\n", i + 1, arr);

        // Optimization: stop if no swaps occurred
        if !swapped {
            println!("Array is already sorted. Stopping early.\n");
            break;
        }
    }

    println!("Final sorted array: {:?}", arr);
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];

    bubble_sort(&mut numbers);
}
