use super::util::read_file_lines;
use std::io::Result;

type Point = (usize, usize);

pub fn solve_p1(filename: &str) -> Result<u32> {
    let mut current_generation = read_file_lines(filename)?
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = current_generation.len();
    let width = current_generation[0].len();

    let mut res = 0;

    for _ in 0..100 {
        let mut next_generation =
            vec![vec![1; current_generation[0].len()]; current_generation.len()];

        for y in 0..current_generation.len() {
            for x in 0..current_generation[y].len() {
                next_generation[y][x] += current_generation[y][x];
            }
        }
        let mut stack = vec![];
        for (y, generation) in next_generation.iter().enumerate() {
            for (x, octopus) in generation.iter().enumerate() {
                if *octopus == 10 {
                    stack.push((x, y));
                    res += 1
                }
            }
        }

        while let Some((x, y)) = stack.pop() {
            let neighbors = get_neighbors_coordinates((x, y), width, height);
            for (x1, y1) in &neighbors {
                next_generation[*y1][*x1] += 1;
                if next_generation[*y1][*x1] == 10 {
                    stack.push((*x1, *y1));
                    res += 1;
                }
            }
        }
        current_generation = next_generation
            .iter()
            .map(|line| line.iter().map(|e| if *e > 9 { 0 } else { *e }).collect())
            .collect();
    }

    Ok(res)
}

fn get_neighbors_coordinates((x, y): Point, width: usize, height: usize) -> Vec<Point> {
    let coord: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    coord
        .iter()
        .filter(|(x1, y1)| x > 0 || *x1 >= 0 && x == 0 || y > 0 || *y1 >= 0 && y == 0)
        .map(|(x1, y1)| {
            let x = x as i8;
            let y = y as i8;
            ((x1 + x) as usize, (y1 + y) as usize)
        })
        .filter(|(x, y)| *x < width && *y < height)
        .collect()
}
