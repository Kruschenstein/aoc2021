use super::util::read_file_lines;
use std::collections::HashMap;
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u64> {
    let lines = read_file_lines(filename)?;
    Ok(count_max_min_elem_after_steps(&lines, 10))
}

pub fn solve_p2(filename: &str) -> Result<u64> {
    let lines = read_file_lines(filename)?;
    Ok(count_max_min_elem_after_steps(&lines, 40))
}

fn count_max_min_elem_after_steps(lines: &[String], steps: u32) -> u64 {
    let transformations = get_transformations(lines);

    let mut pair_count = get_pair_count(lines);
    let mut char_count = get_char_count(lines);

    for _ in 0..steps {
        let mut new_pair_count = HashMap::new();
        for (point @ (a, b), value) in pair_count.iter() {
            let new_elem = transformations[point];

            new_pair_count
                .entry((*a, new_elem))
                .and_modify(|e| *e += value)
                .or_insert(*value);
            new_pair_count
                .entry((new_elem, *b))
                .and_modify(|e| *e += value)
                .or_insert(*value);
            char_count
                .entry(new_elem)
                .and_modify(|e| *e += value)
                .or_insert(*value);
        }
        pair_count = new_pair_count;
    }

    char_count.values().max().unwrap() - char_count.values().min().unwrap()
}

fn get_transformations(lines: &[String]) -> HashMap<(char, char), char> {
    lines
        .iter()
        .skip(2)
        .map(|line| {
            let split = line.split(" -> ").collect::<Vec<_>>();
            let key = split[0].chars().collect::<Vec<_>>();
            let value = split[1].chars().next().unwrap();
            ((key[0], key[1]), value)
        })
        .collect()
}

fn get_pair_count(lines: &[String]) -> HashMap<(char, char), u64> {
    (&lines.get(0).unwrap().chars().collect::<Vec<_>>()[..])
        .windows(2)
        .map(|elem| ((elem[0], elem[1]), 1))
        .collect()
}

fn get_char_count(lines: &[String]) -> HashMap<char, u64> {
    lines
        .get(0)
        .unwrap()
        .chars()
        .fold(HashMap::new(), |mut acc, elem| {
            *acc.entry(elem).or_insert(0) += 1;
            acc
        })
}
