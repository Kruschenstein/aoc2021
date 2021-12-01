use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn solve_day_1p1(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);
    let lines = buffer
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Ok(lines
        .iter()
        .zip(lines.iter().skip(1))
        .map(|(previous, next)| if previous < next { 1 } else { 0 })
        .sum())
}

fn main() -> Result<()> {
    println!("day 1.1 {}", solve_day_1p1("resources/day1")?);
    Ok(())
}
