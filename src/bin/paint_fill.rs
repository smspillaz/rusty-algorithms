fn fill_pixel(mut grid: &mut Vec<Vec<i32>>, pos: (usize, usize), color: i32, replace: i32) {
    if grid[pos.0][pos.1] != replace {
        return;
    }

    grid[pos.0][pos.1] = color;

    if pos.0 > 0 {
        fill_pixel(grid, (pos.0 - 1, pos.1), color, replace);
    }

    if pos.0 < grid.len() - 1 {
        fill_pixel(grid, (pos.0 + 1, pos.1), color, replace);
    }

    if pos.1 > 0 {
        fill_pixel(grid, (pos.0, pos.1 - 1), color, replace);
    }

    if pos.1 < grid[0].len() - 1 {
        fill_pixel(grid, (pos.0, pos.1 + 1), color, replace);
    }
}

fn main() {
    let mut grid = vec![
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 0]
    ];

    fill_pixel(&mut grid, (4, 4), 2, 1);
    println!("{:?}", grid);
}