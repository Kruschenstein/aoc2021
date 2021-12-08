use super::util::read_file_lines;
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;
    Ok(lines
        .iter()
        .map(|line| line.split(" | ").collect::<Vec<_>>()[1])
        .flat_map(|digits| digits.split(" ").map(|d| d.len()))
        .filter(|d| *d >= 2 && *d <= 4 || *d == 7)
        .count() as u32)
}
