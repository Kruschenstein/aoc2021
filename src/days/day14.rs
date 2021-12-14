use super::util::read_file_lines;
use std::collections::{HashMap, LinkedList};
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u64> {
    let lines = read_file_lines(filename)?;
    let polymere = get_polymere(&lines);
    let transformations = get_transformations(&lines);

    Ok(difference_between_the_most_and_the_least_common_element(
        &get_polymere_after_steps(10, &polymere, &transformations),
    ))
}

fn get_polymere(lines: &[String]) -> LinkedList<char> {
    lines
        .iter()
        .next()
        .unwrap()
        .chars()
        .collect::<LinkedList<_>>()
}

fn get_transformations(lines: &[String]) -> HashMap<u32, char> {
    lines
        .iter()
        .skip(2)
        .map(|line| {
            let split = line.split(" -> ").collect::<Vec<_>>();
            let key = split[0].chars().collect::<Vec<_>>();
            let key = hash(key[0], key[1]);
            let value = split[1].chars().next().unwrap();
            (key, value)
        })
        .collect::<HashMap<_, _>>()
}

fn hash(a: char, b: char) -> u32 {
    a as u32 - 'A' as u32 + (b as u32 - 'A' as u32) * 100
}

fn get_polymere_after_steps(
    nb_step: u32,
    polymere: &LinkedList<char>,
    transformations: &HashMap<u32, char>,
) -> LinkedList<char> {
    let mut polymere = polymere.clone();
    for _ in 0..nb_step {
        let mut first = polymere.pop_front().unwrap();

        let mut new_polymere = LinkedList::from([first]);
        while let Some(current) = polymere.pop_front() {
            let new_elem = transformations[&hash(first, current)];
            new_polymere.push_back(new_elem);
            new_polymere.push_back(current);
            first = current;
        }
        polymere = new_polymere;
    }
    polymere
}

fn difference_between_the_most_and_the_least_common_element(polymere: &LinkedList<char>) -> u64 {
    let count = polymere.iter().fold(HashMap::new(), |mut acc, elem| {
        *acc.entry(elem).or_insert(0) += 1;
        acc
    });
    count.values().max().unwrap() - count.values().min().unwrap()
}
