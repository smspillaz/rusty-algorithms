fn floydwarshal(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    /* We keep two copies of the matrix around so that we can do
     * our multiplication */
    let mut arena = vec![matrix.clone(), matrix.clone()];
    let mut index = 0;

    let len = matrix.len();
    for k in 0..len {
        println!("Iteration {:?}", k);
        index = (index + 1) % 2;
        let current = index;
        let prev = (current + 1) % 2;

        for i in 0..len {
            for j in 0..len {
                if arena[prev][i][k] != std::i32::MAX && arena[prev][k][j] != std::i32::MAX {
                    arena[current][i][j] = std::cmp::min(arena[prev][i][j],
                                                         arena[prev][i][k] + arena[prev][k][j]);
                }
            }
        }
    }

    return arena[index].clone();
}

fn main() {
    let matrix = vec![
        vec![0, 1, 0],
        vec![0, 0, 1],
        vec![1, 0, 0]
    ];
    let compiled = matrix.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, col)| {
            if *col == 0 && i != j {
                return std::i32::MAX;
            }

            return *col
        }).collect()
    }).collect();

    println!("{:?}", floydwarshal(&compiled));
}