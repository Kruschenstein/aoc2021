use super::util::read_file_lines;
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;
    let grid = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut paths = grid.clone();

    for y in (0..grid.len()).rev() {
        for x in (0..grid[y].len()).rev() {
            if y == height - 1 && x == width - 1 {
                continue;
            }
            let mut res = vec![];
            if y + 1 < height {
                res.push(paths[y + 1][x]);
            }
            if x + 1 < width {
                res.push(paths[y][x + 1]);
            }
            paths[y][x] += *res.iter().min().unwrap();
        }
    }

    Ok(paths[0][0] - grid[0][0])
}
