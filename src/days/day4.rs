use super::util::read_file_lines;
use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
struct Marked {
    elem: u32,
    marked: bool,
}

impl Marked {
    fn new(elem: u32) -> Marked {
        Marked {
            elem,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

type Grid = Vec<Vec<Marked>>;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;
    let nums = read_numbers_line(&lines);

    let mut grids = read_grid(&lines);

    for num in nums {
        for mut grid in grids.iter_mut() {
            mark_grid_elem(num, &mut grid);
            if grid_has_line(&grid) {
                let all_unmarked = get_all_unmarked_number(&grid);
                return Ok(all_unmarked.iter().sum::<u32>() * num);
            }
        }
    }
    Err(Error::new(ErrorKind::Other, "Nothing found"))
}

fn read_numbers_line(lines: &[String]) -> Vec<u32> {
    lines[0]
        .split(',')
        .map(|elem| elem.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn read_grid(lines: &[String]) -> Vec<Grid> {
    lines.iter().skip(1).fold(vec![], |mut acc, elem| {
        if elem.is_empty() {
            acc.push(vec![]);
        } else {
            let grid_num = elem
                .split(' ')
                .filter(|e| !e.is_empty())
                .map(|e| e.parse::<u32>().unwrap())
                .map(Marked::new)
                .collect::<Vec<Marked>>();
            acc.last_mut().unwrap().push(grid_num);
        }
        acc
    })
}

fn mark_grid_elem(num: u32, grid: &mut Grid) {
    for line in grid {
        for e in line {
            if num == e.elem {
                e.mark();
            }
        }
    }
}

fn grid_has_line(grid: &Grid) -> bool {
    for (i, line) in grid.iter().enumerate() {
        if is_line_entirely_marked(&line) || is_column_entirely_marked(i, &grid) {
            return true;
        }
    }
    return false;
}

fn is_line_entirely_marked(line: &[Marked]) -> bool {
    line.iter().all(|elem| elem.marked)
}

fn is_column_entirely_marked(column_index: usize, grid: &Grid) -> bool {
    grid.iter()
        .map(|line| &line[column_index])
        .all(|elem| elem.marked)
}

fn get_all_unmarked_number(grid: &Grid) -> Vec<u32> {
    grid.iter()
        .flat_map(|line| line.iter())
        .filter(|elem| !elem.marked)
        .map(|elem| elem.elem)
        .collect()
}
