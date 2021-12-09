use super::util::read_file_lines;
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let grid = read_file_lines(filename)?
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let actual = grid[y][x];
            let neighbors = [
                get_at(&grid, x, y, (0, -1)),
                get_at(&grid, x, y, (0, 1)),
                get_at(&grid, x, y, (-1, 0)),
                get_at(&grid, x, y, (1, 0)),
            ]
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<_>>();
            if neighbors.iter().all(|n| *n > actual) {
                res += actual + 1;
            }
        }
    }
    Ok(res)
}

fn get_at(grid: &Vec<Vec<u32>>, x: usize, y: usize, (x1, y1): (i8, i8)) -> Option<u32> {
    if (x == 0 && x1 < 0) || (y == 0 && y1 < 0) {
        None
    } else {
        let y = y as i8;
        let x = x as i8;
        grid.get((y + y1) as usize)
            .map(|g| g.get((x + x1) as usize))
            .flatten()
            .map(|e| *e)
    }
}
