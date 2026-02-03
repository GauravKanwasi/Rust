use std::io;

fn read_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; cols]; rows];
    for i in 0..rows {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let values: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 0..cols {
            matrix[i][j] = values[j];
        }
    }
    matrix
}

fn multiply_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = a.len();
    let cols = b[0].len();
    let common = b.len();

    let mut result = vec![vec![0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            for k in 0..common {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn main() {
    let mut input = String::new();

    println!("Enter rows and columns of first matrix:");
    io::stdin().read_line(&mut input).unwrap();
    let dims1: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (r1, c1) = (dims1[0], dims1[1]);

    println!("Enter rows and columns of second matrix:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let dims2: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (r2, c2) = (dims2[0], dims2[1]);

    if c1 != r2 {
        println!("Matrix multiplication not possible!");
        return;
    }

    println!("Enter first matrix:");
    let m1 = read_matrix(r1, c1);

    println!("Enter second matrix:");
    let m2 = read_matrix(r2, c2);

    let result = multiply_matrices(&m1, &m2);

    println!("Result matrix:");
    for row in result {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}
