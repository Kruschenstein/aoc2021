use super::util::read_file_lines;
use std::collections::HashSet;
use std::io::Result;

type Point = (usize, usize);
type Grid = Vec<Vec<u32>>;

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
                get_at_coord_with_move(&grid, x, y, (0, -1)),
                get_at_coord_with_move(&grid, x, y, (0, 1)),
                get_at_coord_with_move(&grid, x, y, (-1, 0)),
                get_at_coord_with_move(&grid, x, y, (1, 0)),
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

fn get_coord(x: usize, y: usize, (x1, y1): (i8, i8)) -> Option<Point> {
    if (x == 0 && x1 < 0) || (y == 0 && y1 < 0) {
        None
    } else {
        let y = y as i8;
        let x = x as i8;
        Some(((x + x1) as usize, (y + y1) as usize))
    }
}

fn get_at<T: Copy>(grid: &Vec<Vec<T>>, (x, y): &Point) -> Option<T> {
    grid.get(*y).map(|g| g.get(*x)).flatten().map(|e| *e)
}

fn get_at_coord_with_move(grid: &Grid, x: usize, y: usize, (x1, y1): (i8, i8)) -> Option<u32> {
    get_coord(x, y, (x1, y1))
        .map(|(x, y)| get_at(grid, &(x, y)))
        .flatten()
}

pub fn solve_p2(filename: &str) -> Result<u32> {
    let grid = read_file_lines(filename)?
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut points = (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (j, i)))
        .filter(|(x, y)| grid[*y][*x] != 9)
        .collect::<HashSet<_>>();

    let mut groups = vec![];
    loop {
        let point = match points.iter().next() {
            Some(p) => *p,
            _ => break,
        };

        let mut group = HashSet::from([point]);
        points.remove(&point);

        let mut stack = vec![point];
        while let Some(point) = stack.pop() {
            let new_points = get_new_points(point, &group, &grid);

            for p in new_points {
                group.insert(p);
                points.remove(&p);
                stack.push(p);
            }
        }
        groups.push(group);
    }

    groups.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

    Ok(groups.iter().take(3).map(|g| g.len() as u32).product())
}

fn get_new_points((x, y): Point, group: &HashSet<Point>, grid: &Grid) -> Vec<Point> {
    let neighbors = [
        get_coord(x, y, (0, -1)),
        get_coord(x, y, (0, 1)),
        get_coord(x, y, (-1, 0)),
        get_coord(x, y, (1, 0)),
    ];
    neighbors
        .into_iter()
        .filter_map(|x| x)
        .filter(|p| !group.contains(p))
        .filter(|p| match get_at(&grid, p) {
            Some(p) => p != 9,
            _ => false,
        })
        .collect::<Vec<_>>()
}
