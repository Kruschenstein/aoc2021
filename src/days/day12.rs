use super::util::read_file_lines;
use std::collections::HashMap;
use std::io::Result;

const START: &str = "start";
const END: &str = "end";

pub fn solve_p1(filename: &str) -> Result<u32> {
    let graph = create_cave_graph(&read_file_lines(filename)?);
    Ok(counter_p1(&graph, &mut vec![], hash(START)))
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().any(char::is_lowercase)
}

fn counter_p1(grid: &HashMap<u32, Vec<Cave>>, path: &mut Vec<u32>, head: u32) -> u32 {
    if head == hash(END) {
        return 1;
    }
    let mut res = 0;
    for cave in &grid[&head] {
        path.push(head);
        let name = cave.name;

        if !cave.is_small || !path.contains(&name) {
            res += counter_p1(grid, path, name);
        }
        path.pop();
    }

    res
}

pub fn solve_p2(filename: &str) -> Result<u32> {
    let graph = create_cave_graph(&read_file_lines(filename)?);
    Ok(counter_p2(&graph, &mut vec![], &Cave::new(START), false))
}

fn counter_p2(
    grid: &HashMap<u32, Vec<Cave>>,
    path: &mut Vec<u32>,
    head: &Cave,
    has_double: bool,
) -> u32 {
    if head.is_end {
        return 1;
    }
    let mut res = 0;
    for cave in &grid[&head.name] {
        path.push(head.name);
        let name = cave.name;

        if cave.is_small && (!has_double || !path.contains(&name)) {
            res += counter_p2(grid, path, cave, has_double || path.contains(&name));
        } else if !cave.is_small {
            res += counter_p2(grid, path, cave, has_double);
        }
        path.pop();
    }

    res
}

struct Cave {
    name: u32,
    is_small: bool,
    is_end: bool,
}

impl Cave {
    fn new(name: &str) -> Cave {
        Cave {
            is_small: is_small_cave(name),
            is_end: name == END,
            name: hash(name),
        }
    }
}

fn create_cave_graph(lines: &[String]) -> HashMap<u32, Vec<Cave>> {
    let mut graph = HashMap::with_capacity(lines.len());

    for line in lines {
        let split = line.split('-').collect::<Vec<_>>();
        let right = split[0].to_string();
        let left = split[1].to_string();
        if left != START {
            graph
                .entry(hash(&right))
                .or_insert_with(Vec::new)
                .push(Cave::new(&left));
        }
        if right != START {
            graph
                .entry(hash(&left))
                .or_insert_with(Vec::new)
                .push(Cave::new(&right));
        }
    }
    graph
}

fn hash(name: &str) -> u32 {
    name.chars().zip([2, 3]).map(|(c, p)| p * c as u32).sum()
}
