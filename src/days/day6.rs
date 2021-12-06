use super::util::read_file_lines;
use std::io::Result;

fn compute_number_of_fishes_after_days(filename: &str, nb_of_days: u32) -> Result<u64> {
    let fishes = read_file_lines(filename)?[0]
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut generation = [0; 9];
    let mut next_generation = [0; 9];

    for fish in fishes {
        generation[fish as usize] += 1;
    }

    for _days in 0..nb_of_days {
        for i in (0..generation.len()).rev() {
            if i == 0 {
                let fishes_who_reproduce = generation[0];
                next_generation[6] += fishes_who_reproduce;
                next_generation[8] = fishes_who_reproduce;
            } else {
                next_generation[i - 1] = generation[i];
            }
        }
        generation = next_generation;
        next_generation = [0; 9];
    }

    Ok(generation.iter().sum())
}

pub fn solve_p1(filename: &str) -> Result<u64> {
    compute_number_of_fishes_after_days(filename, 80)
}

pub fn solve_p2(filename: &str) -> Result<u64> {
    compute_number_of_fishes_after_days(filename, 256)
}
