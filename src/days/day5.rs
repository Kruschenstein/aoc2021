use super::util::read_file_lines;
use std::io::Result;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn from(repr: &str) -> Point {
        let split_repr = repr.split(',').collect::<Vec<&str>>();
        Point {
            x: split_repr[0].parse::<u32>().unwrap(),
            y: split_repr[1].parse::<u32>().unwrap(),
        }
    }
}

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?
        .iter()
        .map(|line| {
            let points = line.split(" -> ").collect::<Vec<&str>>();
            let first = Point::from(points[0]);
            let second = Point::from(points[1]);

            if first.x + first.y <= second.x + second.y {
                (first, second)
            } else {
                (second, first)
            }
        })
        .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y)
        .collect::<Vec<_>>();

    let mut grid = [[0; 1000]; 1000];

    for (a, b) in lines {
        for i in a.x..=b.x {
            for j in a.y..=b.y {
                grid[j as usize][i as usize] += 1
            }
        }
    }

    Ok(grid
        .iter()
        .flat_map(|e| e.iter())
        .filter(|i| **i > 1)
        .map(|_| 1)
        .sum())
}
