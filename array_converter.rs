struct ArrayConverter;

impl ArrayConverter {
    pub fn array2d_to_3d<T: Clone>(
        arr2d: Vec<Vec<T>>,
        depth: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<Vec<Vec<T>>> {
        let flat: Vec<T> = arr2d.into_iter().flatten().collect();

        assert_eq!(flat.len(), depth * rows * cols);

        let mut index = 0;
        let mut result = Vec::with_capacity(depth);

        for _ in 0..depth {
            let mut layer = Vec::with_capacity(rows);

            for _ in 0..rows {
                let mut row = Vec::with_capacity(cols);

                for _ in 0..cols {
                    row.push(flat[index].clone());
                    index += 1;
                }

                layer.push(row);
            }

            result.push(layer);
        }

        result
    }

    pub fn array3d_to_2d<T>(arr3d: Vec<Vec<Vec<T>>>) -> Vec<Vec<T>> {
        let mut result = Vec::new();

        for layer in arr3d {
            for row in layer {
                result.push(row);
            }
        }

        result
    }
}

fn main() {
    let array_2d = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];

    let array_3d = ArrayConverter::array2d_to_3d(array_2d.clone(), 2, 2, 3);

    println!("Converted to 3D:");
    println!("{:?}", array_3d);

    let back_to_2d = ArrayConverter::array3d_to_2d(array_3d);

    println!("Converted back to 2D:");
    println!("{:?}", back_to_2d);
}