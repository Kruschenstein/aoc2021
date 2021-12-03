use super::util::read_file_lines;
use std::io::Result;

struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;

    let position = lines.iter().fold(
        Position {
            depth: 0,
            horizontal: 0,
            aim: 0,
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

pub fn solve_p2(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;

    let position = lines.iter().fold(
        Position {
            depth: 0,
            horizontal: 0,
            aim: 0,
        },
        |acc, val| {
            let instruction: Vec<&str> = val.split(' ').collect();
            let unit: u32 = instruction[1].parse().unwrap();
            match instruction[0] {
                "forward" => Position {
                    horizontal: acc.horizontal + unit,
                    depth: acc.depth + acc.aim * unit,
                    ..acc
                },
                "down" => Position {
                    aim: acc.aim + unit,
                    ..acc
                },
                "up" => Position {
                    aim: acc.aim - unit,
                    ..acc
                },
                _ => panic!("this must not happened"),
            }
        },
    );

    Ok(position.depth * position.horizontal)
}
