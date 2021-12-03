use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn solve_day_p1(filename: &str) -> Result<u32> {
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

pub fn solve_day_p2(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);
    let lines = buffer
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let windows: Vec<u32> = lines
        .windows(3)
        .map(|elem| elem.into_iter().sum())
        .collect();

    Ok(windows
        .iter()
        .zip(windows.iter().skip(1))
        .map(|(previous, next)| if previous < next { 1 } else { 0 })
        .sum())
}
