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

fn mean(collection: &[u32]) -> u32 {
    (collection.iter().map(|i| *i as f64).sum::<f64>() / collection.len() as f64) as u32
}

fn generate_fuel_costs(n: u32) -> Vec<u64> {
    let n = n as usize;
    let mut result = (0..n).map(|i| i as u64).collect::<Vec<u64>>();
    for i in 1..n {
        result[i] = result[i - 1] + i as u64;
    }
    result
}

pub fn solve_p2(filename: &str) -> Result<u64> {
    let positions = read_file_lines(filename)?[0]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mean = mean(&positions);
    let half = *positions.iter().max().unwrap();
    let costs = generate_fuel_costs(half);

    Ok(positions
        .iter()
        .map(|n| if *n < mean { mean - n } else { n - mean })
        .map(|i| costs[i as usize])
        .sum())
}
