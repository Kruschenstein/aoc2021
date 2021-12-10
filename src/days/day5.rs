use super::util::read_file_lines;
use std::io::Result;
use std::ops::Add;

const GRID_SIZE: usize = 1000;
type Grid = [[u32; GRID_SIZE]; GRID_SIZE];
type Segment = (Point, Point);

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

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, (x, y): (i32, i32)) -> Point {
        Point {
            x: (self.x as i32 + x) as u32,
            y: (self.y as i32 + y) as u32,
        }
    }
}

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?
        .iter()
        .map(|line| to_segment(line))
        .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y)
        .collect::<Vec<_>>();

    let mut grid = new_grid();

    for segment in lines {
        mark_segment_position(&mut grid, &segment);
    }

    Ok(count_number_of_overlapped_segment(&grid))
}

pub fn solve_p2(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?
        .iter()
        .map(|line| to_segment(line))
        .collect::<Vec<_>>();

    let mut grid = new_grid();

    for segment in lines {
        if same_line_or_column(&segment) {
            mark_segment_position(&mut grid, &segment);
        } else {
            mark_diagonal_position(&mut grid, &segment);
        }
    }

    Ok(count_number_of_overlapped_segment(&grid))
}

fn new_grid() -> Grid {
    [[0; GRID_SIZE]; GRID_SIZE]
}

fn to_segment(line: &str) -> Segment {
    let points = line.split(" -> ").collect::<Vec<&str>>();
    let first = Point::from(points[0]);
    let second = Point::from(points[1]);

    if first.x + first.y <= second.x + second.y {
        (first, second)
    } else {
        (second, first)
    }
}

fn increase_at_position(grid: &mut Grid, x: u32, y: u32) {
    grid[y as usize][x as usize] += 1;
}

fn same_line_or_column((a, b): &Segment) -> bool {
    a.x == b.x || a.y == b.y
}

fn mark_segment_position(grid: &mut Grid, (a, b): &Segment) {
    for i in a.x..=b.x {
        for j in a.y..=b.y {
            increase_at_position(grid, i, j);
        }
    }
}

fn mark_diagonal_position(grid: &mut Grid, (a, b): &Segment) {
    let p: (i32, i32) = (
        if a.x <= b.x { 1 } else { -1 },
        if a.y <= b.y { 1 } else { -1 },
    );

    let mut point = Point { ..*a };
    while point.x != b.x || point.y != b.y {
        increase_at_position(grid, point.x, point.y);
        point = point + p;
    }
    increase_at_position(grid, point.x, point.y);
}

fn count_number_of_overlapped_segment(grid: &Grid) -> u32 {
    grid.iter()
        .flat_map(|e| e.iter())
        .filter(|i| **i > 1)
        .map(|_| 1)
        .sum()
}
