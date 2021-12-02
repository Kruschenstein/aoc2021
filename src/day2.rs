use std::fs::File;
use std::io::{BufRead, BufReader, Result};

struct Position {
    depth: u32,
    horizontal: u32,
}

pub fn solve_day_2p1(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);
    let lines = buffer
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let position = lines.iter().fold(
        Position {
            depth: 0,
            horizontal: 0,
        },
        |acc, val| {
            let instruction: Vec<&str> = val.split(' ').collect();
            let unit: u32 = instruction[1].parse().unwrap();
            match instruction[0] {
                "forward" => Position {
                    horizontal: acc.horizontal + unit,
                    ..acc
                },
                "down" => Position {
                    depth: acc.depth + unit,
                    ..acc
                },
                "up" => Position {
                    depth: acc.depth - unit,
                    ..acc
                },
                _ => panic!("this must not happened"),
            }
        },
    );

    Ok(position.depth * position.horizontal)
}
