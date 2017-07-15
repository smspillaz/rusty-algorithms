fn tictactoe(grid: &Vec<Vec<i32>>, pos: (usize, usize), direction: (i32, i32), search: i32) -> i32 {
    if grid[pos.0][pos.1] == search {
        let next = (pos.0 as i32 + direction.0, pos.1 as i32 + direction.1);
        if next.0 < 0 || next.0 == grid.len() as i32 || next.1 < 0 || next.1 > grid.len() as i32 {
            return search;
        }

        return tictactoe(grid, (next.0 as usize, next.1 as usize), direction, search);
    }

    return -1;
}

fn main() {
    let grid = vec![
        vec![1, 0, 0],
        vec![1, 0, 1],
        vec![0, 1, 1]
    ];

    let combinations = vec![
        ((0, 0), (0, 1)),
        ((1, 0), (0, 1)),
        ((2, 0), (0, 1)),
        ((0, 0), (1, 0)),
        ((0, 1), (1, 0)),
        ((0, 2), (1, 0)),
        ((0, 0), (1, 1)),
        ((0, 2), (-1, -1))
    ];
    
    for pos_direction in combinations {
        let (pos, direction) = pos_direction;
        let result = tictactoe(&grid, pos, direction, grid[pos.0][pos.1]);
        if result != -1 {
            println!("{:?} wins!", result);
            break;
        }
    };
}