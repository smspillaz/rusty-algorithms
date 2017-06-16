fn exponentiate_shortest_paths(lhs: &Vec<Vec<i32>>, rhs: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = lhs.len();
    return (0..len).map(|i| {
        (0..len).map(|j| {
            /* Take the current value on lhs and do a_ik + b_kj
             * for k in 0 to len */
            let mut result = std::i32::MAX;
            for k in 0..len {
                if lhs[i][k] != std::i32::MAX && rhs[k][j] != std::i32::MAX {
                    result = std::cmp::min(result, lhs[i][k] + rhs[k][j]);
                }
            }

            return result;
        }).collect()
    }).collect();
}

fn main() {
    let matrix = vec![
        vec![0, 1, 0],
        vec![0, 0, 1],
        vec![1, 0, 0]
    ];
    let product: Vec<Vec<i32>> = matrix.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, col)| {
            if i != j && *col == 0 { std::i32::MAX } else { *col }
        }).collect()
    }).collect();

    println!("{:?}", (0..matrix.len()).fold(product.clone(), |result, _| {
        exponentiate_shortest_paths(&result, &product)
    }));
}