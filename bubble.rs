use std::fmt::Debug;

fn bubble_sort<T: PartialOrd + Debug>(arr: &mut [T], verbose: bool) -> usize {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }

    if verbose {
        println!("Initial array: {:?}\n", arr);
    }

    let mut sorted_boundary = n;
    let mut total_swaps = 0;
    let mut pass = 0;

    loop {
        pass += 1;
        let mut last_swap = 0;
        let mut swapped = false;

        if verbose {
            println!("Pass {}:", pass);
        }

        for j in 0..sorted_boundary.saturating_sub(1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
                total_swaps += 1;
                last_swap = j + 1;

                if verbose {
                    println!("  Swap {} <-> {} -> {:?}", j, j + 1, arr);
                }
            }
        }

        if verbose {
            println!("  Result after pass {}: {:?}\n", pass, arr);
        }

        if !swapped {
            if verbose {
                println!("Array sorted after {} pass(es). Stopping early.\n", pass);
            }
            break;
        }

        sorted_boundary = last_swap;
    }

    if verbose {
        println!("Final sorted array: {:?}", arr);
        println!("Total swaps: {}", total_swaps);
    }

    total_swaps
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut numbers, true);

    let mut words = vec!["banana", "apple", "cherry", "date"];
    println!("\n---\n");
    bubble_sort(&mut words, true);

    let mut floats = vec![3.2, 1.1, 4.4, 1.5, 9.9, 2.6];
    println!("\n---\n");
    bubble_sort(&mut floats, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_integers() {
        let mut data = vec![5, 3, 8, 1, 9, 2];
        bubble_sort(&mut data, false);
        assert_eq!(data, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn handles_empty_and_single() {
        let mut empty: Vec<i32> = vec![];
        let mut single = vec![42];
        assert_eq!(bubble_sort(&mut empty, false), 0);
        assert_eq!(bubble_sort(&mut single, false), 0);
    }

    #[test]
    fn sorts_already_sorted_with_zero_swaps() {
        let mut data = vec![1, 2, 3, 4, 5];
        assert_eq!(bubble_sort(&mut data, false), 0);
    }

    #[test]
    fn sorts_strings() {
        let mut data = vec!["zebra", "apple", "mango"];
        bubble_sort(&mut data, false);
        assert_eq!(data, vec!["apple", "mango", "zebra"]);
    }
}
