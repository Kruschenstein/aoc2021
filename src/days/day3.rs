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

pub fn solve_p2(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;
    let line_len = lines[0].len();
    let mut oxygen = lines;
    let mut co2 = oxygen.clone();

    for i in 0..line_len {
        let count1_oxygen = get_number_of_1_at_position(&oxygen, i);
        let count1_co2 = get_number_of_1_at_position(&co2, i);
        let oxygen_len = oxygen.len() as u32;
        let co2_len = co2.len() as u32;
        let keep1_oxygen = count1_oxygen * 2 >= oxygen_len;
        let keep1_co2 = count1_co2 < co2_len / 2;

        if oxygen_len > 1 {
            oxygen = oxygen
                .into_iter()
                .filter(|elem| {
                    let c = elem.chars().nth(i).unwrap();
                    keep1_oxygen && c == '1' || !keep1_oxygen && c == '0'
                })
                .collect();
        }
        if co2_len > 1 {
            co2 = co2
                .into_iter()
                .filter(|elem| {
                    let c = elem.chars().nth(i).unwrap();
                    keep1_co2 && c == '1' || !keep1_co2 && c == '0'
                })
                .collect();
        }
    }

    let oxygen_num = oxygen[0]
        .chars()
        .map(|e| if e == '1' { 1 } else { 0 })
        .collect::<Vec<_>>();
    let co2_num = co2[0]
        .chars()
        .map(|e| if e == '1' { 1 } else { 0 })
        .collect::<Vec<_>>();

    Ok(bytes_to_num(&oxygen_num) * bytes_to_num(&co2_num))
}

fn get_number_of_1_at_position(lines: &[String], position: usize) -> u32 {
    lines
        .iter()
        .map(|line| line.chars().nth(position).unwrap())
        .map(|e| if e == '1' { 1 } else { 0 })
        .sum()
}
