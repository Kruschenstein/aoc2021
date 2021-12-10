use super::util::read_file_lines;

use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Ok(lines
        .iter()
        .zip(lines.iter().skip(1))
        .map(|(previous, next)| if previous < next { 1 } else { 0 })
        .sum())
}

pub fn solve_p2(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?
        .iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let windows: Vec<u32> = lines.windows(3).map(|elem| elem.iter().sum()).collect();

    Ok(windows
        .iter()
        .zip(windows.iter().skip(1))
        .map(|(previous, next)| if previous < next { 1 } else { 0 })
        .sum())
}
