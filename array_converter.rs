pub fn array2d_to_3d<T: Clone>(
    arr2d: Vec<Vec<T>>,
    depth: usize,
    rows: usize,
    cols: usize,
) -> Vec<Vec<Vec<T>>> {
    let flat: Vec<T> = arr2d.into_iter().flatten().collect();
    assert_eq!(flat.len(), depth * rows * cols, "Dimensions do not match element count");

    flat.chunks(rows * cols)
        .map(|layer| layer.chunks(cols).map(|row| row.to_vec()).collect())
        .collect()
}

pub fn array3d_to_2d<T>(arr3d: Vec<Vec<Vec<T>>>) -> Vec<Vec<T>> {
    arr3d.into_iter().flatten().collect()
}

fn main() {
    let array_2d = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];

    let array_3d = array2d_to_3d(array_2d, 2, 2, 3);
    println!("Converted to 3D:\n{:?}", array_3d);

    let back_to_2d = array3d_to_2d(array_3d);
    println!("Converted back to 2D:\n{:?}", back_to_2d);
}
