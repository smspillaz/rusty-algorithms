fn rot_90deg(input: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = input.len();
    let edge = len - 1;
    let mut output: Vec<Vec<i32>> = input.iter().map(|_| {
        input.iter().map(|_| { 0 }).collect()
    }).collect();

    /* Work inwards */
    for k in 0..len / 2 {
        let top_row: usize = k;
        let right_col: usize = edge - k;
        let bottom_row: usize = edge - k;
        let left_col: usize = k;

        /* Do rotations */
        for x in 0..len {
            output[right_col][x] = input[x][top_row];
            output[x][bottom_row] = input[right_col][edge - x];
            output[left_col][x] = input[x][bottom_row];
            output[x][top_row] = input[left_col][edge - x];
        }
    }

    /* Odd sized matrix - copy median element across */
    if len % 2 == 1 {
        let center = len / 2;
        output[center][center] = input[center][center];
    }

    return output;
}

fn main() {
    let mat = vec![
        vec![0, 0, 1],
        vec![0, 0, 0],
        vec![1, 0, 0]
    ];

    println!("{:?}", mat);
    println!("{:?}", rot_90deg(&mat));
}
