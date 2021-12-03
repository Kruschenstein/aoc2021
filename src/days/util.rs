use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn read_file_lines(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    BufReader::new(file).lines().collect()
}

pub fn bytes_to_num(buf: &[u8]) -> u32 {
    buf.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &val)| acc + 2_u32.pow(i as u32) * val as u32)
}
