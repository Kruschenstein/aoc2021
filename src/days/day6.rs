use super::util::read_file_lines;
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let fishes = read_file_lines(filename)?[0]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut generation = [0; 9];
    let mut next_generation = [0; 9];

    for fish in fishes {
        generation[fish as usize] += 1;
    }

    for _days in 0..80 {
        for i in (0..generation.len()).rev() {
            if i == 0 {
                let day0 = generation[0];
                next_generation[6] += day0;
                next_generation[8] = day0;
            } else {
                next_generation[i - 1] = generation[i];
            }
        }
        generation = next_generation;
        next_generation = [0; 9];
    }

    Ok(generation.iter().sum())
}
