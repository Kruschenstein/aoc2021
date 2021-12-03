use super::util::{bytes_to_num, read_file_lines};
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;

    let count_1 = count_number_of_1_for_each_position(&lines);
    let (gamma, epsilon) = to_gamma_and_epsilon(lines.len(), &count_1);

    Ok(gamma * epsilon)
}

fn count_number_of_1_for_each_position(lines: &[String]) -> Vec<u32> {
    let sequence_len = lines[0].len();

    lines.iter().fold(vec![0; sequence_len], |mut acc, value| {
        for (i, elem) in value.chars().enumerate() {
            if elem == '1' {
                acc[i] += 1;
            }
        }
        acc
    })
}

fn to_gamma_and_epsilon(lines_len: usize, count: &[u32]) -> (u32, u32) {
    let lines_len_half = (lines_len / 2) as u32;
    let mut gamma = vec![0; count.len()];
    let mut epsilon = vec![1; count.len()];

    for (i, elem) in count.iter().enumerate() {
        if *elem > lines_len_half {
            gamma[i] = 1;
            epsilon[i] = 0;
        }
    }

    (bytes_to_num(&gamma), bytes_to_num(&epsilon))
}
