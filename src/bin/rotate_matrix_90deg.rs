fn rot_90deg(mut matrix: &mut Vec<Vec<i32>>) -> &Vec<Vec<i32>> {
    let len = matrix.len();
    let edge = len - 1;

    /* Work inwards */
    for k in 0..len / 2 {
        let top_row: usize = k;
        let right_col: usize = edge - k;
        let bottom_row: usize = edge - k;
        let left_col: usize = k;

        /* Do rotations */
        for x in 0..(len - (k * 2 + 1)) {
            let src = vec![
                matrix[top_row][x + k],
                matrix[x + k][right_col],
                matrix[bottom_row][edge - (x + k)],
                matrix[edge - (x + k)][left_col]
            ];

            matrix[x + k][right_col] = src[0];
            matrix[bottom_row][edge - (x + k)] = src[1];
            matrix[edge - (x + k)][left_col] = src[2];
            matrix[top_row][x + k] = src[3];
        }
    }

    return matrix;
}

fn main() {
    let mut mat = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];

    println!("{:?}", mat);
    println!("{:?}", rot_90deg(&mut mat));

    let mut mat44 = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16]
    ];

    println!("{:?}", mat44);
    println!("{:?}", rot_90deg(&mut mat44));

    let mut mat55 = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25]
    ];

    println!("{:?}", mat55);
    println!("{:?}", rot_90deg(&mut mat55));
}
