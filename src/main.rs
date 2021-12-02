mod day2;

use day2::solve_day_2p1;

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

fn solve_day_1p2(filename: &str) -> Result<u32> {
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

fn main() -> Result<()> {
    println!("day 1.1 {}", solve_day_1p1("resources/day1")?);
    println!("day 1.2 {}", solve_day_1p2("resources/day1")?);
    println!("day 2.1 {}", solve_day_2p1("resources/day2")?);
    Ok(())
}
