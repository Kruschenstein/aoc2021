use super::util::read_file_lines;
use std::collections::{HashMap, HashSet};
use std::io::Result;

const START: &str = "start";
const END: &str = "end";

pub fn solve_p1(filename: &str) -> Result<u32> {
    let graph = create_cave_graph(&read_file_lines(filename)?);
    let mut paths = 0;
    let mut not_ended_paths = vec![];

    for node in &graph[START] {
        not_ended_paths.push(Path {
            path: HashSet::from([START.to_string(), node.name.to_string()]),
            head: node.name.to_string(),
            has_double: false,
        });
    }

    while let Some(path) = not_ended_paths.pop() {
        let last = &path.head;
        for cave in &graph[last] {
            if !cave.is_small || !path.path.contains(&cave.name) {
                let mut new_path = path.path.clone();
                new_path.insert(cave.name.to_string());

                let new_path = Path {
                    path: new_path,
                    head: cave.name.to_string(),
                    has_double: false,
                };

                if cave.is_end {
                    paths += 1
                } else {
                    not_ended_paths.push(new_path);
                }
            }
        }
    }

    Ok(paths)
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().any(char::is_lowercase)
}

struct Path {
    path: HashSet<String>,
    has_double: bool,
    head: String,
}

impl Path {
    fn append(cave: &Cave, path: &Path) -> Path {
        let mut new_path = path.path.clone();
        new_path.insert(cave.name.to_string());
        Path {
            path: new_path,
            has_double: path.has_double || cave.is_small && path.path.contains(&cave.name),
            head: cave.name.to_string(),
        }
    }
}

pub fn solve_p2(filename: &str) -> Result<u32> {
    let graph = create_cave_graph(&read_file_lines(filename)?);
    let mut paths = 0;
    let mut not_ended_paths = vec![];

    for node in &graph[START] {
        not_ended_paths.push(Path {
            path: HashSet::from([START.to_string(), node.name.to_string()]),
            head: node.name.to_string(),
            has_double: false,
        });
    }

    while let Some(path) = not_ended_paths.pop() {
        let last = &path.head;
        for cave in &graph[last] {
            if !cave.is_small || !path.has_double || !path.path.contains(&cave.name) {
                let new_path = Path::append(cave, &path);
                if cave.is_end {
                    paths += 1
                } else {
                    not_ended_paths.push(new_path);
                }
            }
        }
    }

    Ok(paths)
}

#[derive(PartialEq, Eq, Hash)]
struct Cave {
    name: String,
    is_small: bool,
    is_end: bool,
}

impl Cave {
    fn new(name: String) -> Cave {
        Cave {
            is_small: is_small_cave(&name),
            is_end: name == END,
            name,
        }
    }
}

fn create_cave_graph(lines: &[String]) -> HashMap<String, Vec<Cave>> {
    let mut graph = HashMap::with_capacity(lines.len());

    for line in lines {
        let split = line.split('-').collect::<Vec<_>>();
        let right = split[0].to_string();
        let left = split[1].to_string();
        if left != START {
            graph
                .entry(right.clone())
                .or_insert_with(Vec::new)
                .push(Cave::new(left.clone()));
        }
        if right != START {
            graph
                .entry(left)
                .or_insert_with(Vec::new)
                .push(Cave::new(right));
        }
    }
    graph
}
