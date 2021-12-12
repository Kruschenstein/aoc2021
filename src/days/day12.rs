use super::util::read_file_lines;
use std::collections::HashMap;
use std::io::Result;

const START: &'static str = "start";
const END: &'static str = "end";

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;
    let graph = create_graph(&lines);
    let mut paths = vec![];
    let mut not_ended_paths = vec![];

    for node in &graph[START] {
        not_ended_paths.push(vec![START.to_string(), node.to_string()]);
    }

    while !not_ended_paths.is_empty() {
        let mut new_paths = Vec::with_capacity(not_ended_paths.len());

        for path in not_ended_paths {
            let last = get_last(&path);
            for cave in &graph[&last] {
                if is_small_cave(&cave) && !path.contains(&cave) || !is_small_cave(&cave) {
                    let mut new_path = path.clone();
                    new_path.push(cave.to_string());
                    if cave == END {
                        paths.push(new_path);
                    } else {
                        new_paths.push(new_path);
                    }
                }
            }
        }
        not_ended_paths = new_paths;
    }
    Ok(paths.len() as u32)
}

fn create_graph(lines: &[String]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::with_capacity(lines.len());

    for line in lines {
        let split = line.split('-').collect::<Vec<_>>();
        let right = split[0].to_string();
        let left = split[1].to_string();
        graph
            .entry(right.clone())
            .or_insert(vec![])
            .push(left.clone());
        graph.entry(left).or_insert(vec![]).push(right);
    }
    graph
}

fn get_last(path: &[String]) -> String {
    path[path.len() - 1].to_string()
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(char::is_lowercase)
}
