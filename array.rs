//! Rust arrays: initialization, safe access, iteration, mutation,
//! slices, common operations, and 2D arrays.

const SIZE: usize = 5;

fn section(title: &str) {
    println!("\n--- {} ---", title);
}

/// Works on any slice (arrays coerce to slices automatically).
fn average(data: &[i32]) -> Option<f64> {
    if data.is_empty() {
        None
    } else {
        Some(data.iter().sum::<i32>() as f64 / data.len() as f64)
    }
}

/// Generic over the array dimensions using const generics.
fn print_matrix<const R: usize, const C: usize>(m: &[[i32; C]; R]) {
    for row in m {
        let cells: Vec<String> = row.iter().map(|v| format!("{:>3}", v)).collect();
        println!("[{}]", cells.join(", "));
    }
}

fn main() {
    // ---------------------------------------------------------------
    section("Initialization");
    let numbers: [i32; SIZE] = [10, 20, 30, 40, 50];
    let zeros: [f64; SIZE] = [0.0; SIZE]; // repeat syntax
    let squares: [usize; SIZE] = std::array::from_fn(|i| i * i); // computed

    println!("numbers : {:?}", numbers);
    println!("zeros   : {:?}", zeros);
    println!("squares : {:?}", squares);
    println!("length  : {}", numbers.len());
    println!("size    : {} bytes", std::mem::size_of_val(&numbers));

    // ---------------------------------------------------------------
    section("Access");
    println!("index 0 : {}", numbers[0]);
    println!("index 3 : {}", numbers[3]);
    println!("first   : {:?}", numbers.first());
    println!("last    : {:?}", numbers.last());

    // numbers[10] would panic; .get() returns an Option instead.
    match numbers.get(10) {
        Some(v) => println!("index 10: {}", v),
        None => println!("index 10: out of bounds (handled safely)"),
    }

    // ---------------------------------------------------------------
    section("Iterating");
    for (i, val) in numbers.iter().enumerate() {
        println!("[{}] = {}", i, val);
    }

    // ---------------------------------------------------------------
    section("Mutability");
    let mut arr = [1, 2, 3];
    println!("before   : {:?}", arr);

    arr[1] = 99;
    println!("set [1]  : {:?}", arr);

    arr.iter_mut().for_each(|v| *v *= 2);
    println!("doubled  : {:?}", arr);

    arr.sort_by(|a, b| b.cmp(a));
    println!("sorted ↓ : {:?}", arr);

    arr.swap(0, 2);
    println!("swapped  : {:?}", arr);

    // ---------------------------------------------------------------
    section("Statistics");
    println!("sum      : {}", numbers.iter().sum::<i32>());
    println!("min      : {:?}", numbers.iter().min());
    println!("max      : {:?}", numbers.iter().max());
    match average(&numbers) {
        Some(avg) => println!("average  : {:.2}", avg),
        None => println!("average  : n/a (empty)"),
    }
    println!("has 30?  : {}", numbers.contains(&30));
    println!("pos of 40: {:?}", numbers.iter().position(|&x| x == 40));

    // ---------------------------------------------------------------
    section("Slices, windows & chunks");
    let middle = &numbers[1..4];
    println!("slice [1..4] : {:?}", middle);

    let diffs: Vec<i32> = numbers.windows(2).map(|w| w[1] - w[0]).collect();
    println!("differences  : {:?}", diffs);

    for (i, chunk) in numbers.chunks(2).enumerate() {
        println!("chunk {}      : {:?}", i, chunk);
    }

    // ---------------------------------------------------------------
    section("Copy semantics & equality");
    let original = [1, 2, 3];
    let mut copy = original; // arrays of Copy types are copied, not moved
    copy[0] = 100;
    println!("original : {:?}", original);
    println!("copy     : {:?}", copy);
    println!("equal?   : {}", original == copy);

    // ---------------------------------------------------------------
    section("2D arrays");
    let grid: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("grid (2x3):");
    print_matrix(&grid);

    let mut transposed = [[0; 2]; 3];
    for r in 0..2 {
        for c in 0..3 {
            transposed[c][r] = grid[r][c];
        }
    }
    println!("transposed (3x2):");
    print_matrix(&transposed);
}
