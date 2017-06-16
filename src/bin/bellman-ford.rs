fn bellmanford(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let len = matrix.len();
    let mut distances: Vec<i32> = (0..len).map(|_| { (std::i32::MAX >> 1) }).collect();

    /* Set distance for source vertex to zero, since we start here */
    distances[0] = 0;

    for _ in 0..len {
        for j in 0..len {
            for (k, weight) in matrix[j].iter().enumerate() {
                if *weight != 0 {
                    distances[k] = std::cmp::min(distances[k], distances[j] + *weight);
                }
            }
        }
    }

    for i in 0..len {
        for j in 0..len {
            if matrix[i][j] != 0 {
                if distances[j] > distances[i] + matrix[i][j] {
                    return (0..len).map(|_| { -1 }).collect();
                }
            }
        }
    }

    return distances;
}

fn main() {
    let matrix = vec![
        vec![0, -1, 1],
        vec![1, 0, 0],
        vec![0, 1, 0]
    ];

    println!("{:?}", bellmanford(matrix));
}