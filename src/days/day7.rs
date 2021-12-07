use super::util::read_file_lines;
use std::io::Result;

fn median(collection: &Vec<u32>) -> u32 {
    let mut sorted = collection.clone();
    sorted.sort();
    sorted[sorted.len() / 2]
}

pub fn solve_p1(filename: &str) -> Result<u32> {
    let positions = read_file_lines(filename)?[0]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let median = median(&positions);

    Ok(positions
        .iter()
        .map(|n| if *n < median { median - n } else { n - median })
        .sum())
}
