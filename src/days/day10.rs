use super::util::{median, read_file_lines};
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;

    let mut res = 0;
    for line in lines {
        let mut stack = vec![];
        for c in line.chars() {
            if is_open(c) {
                stack.push(c);
            } else if let Some(pair) = stack.pop() {
                if does_not_close(pair, c) {
                    res += get_score(c);
                    break;
                }
            } else {
                break;
            }
        }
    }

    Ok(res)
}

fn is_open(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn does_not_close(open: char, close: char) -> bool {
    open == '(' && close != ')'
        || open == '[' && close != ']'
        || open == '{' && close != '}'
        || open == '<' && close != '>'
}

fn get_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => {
            panic!("unexpected {}", c);
        }
    }
}

pub fn solve_p2(filename: &str) -> Result<u64> {
    let scores = read_file_lines(filename)?
        .iter()
        .filter_map(|line| get_line_score(line))
        .collect::<Vec<_>>();
    Ok(median(&scores))
}

fn get_line_score(line: &str) -> Option<u64> {
    get_valid_symbol_stack(line).map(|mut stack| get_remaining_symbol_score(&mut stack))
}

fn get_valid_symbol_stack(line: &str) -> Option<Vec<char>> {
    let mut stack = vec![];
    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        } else if let Some(pair) = stack.pop() {
            if does_not_close(pair, c) {
                return None;
            }
        }
    }
    Some(stack)
}

fn get_remaining_symbol_score(stack: &mut Vec<char>) -> u64 {
    let mut res = 0;
    while let Some(symbol) = stack.pop() {
        res *= 5;
        match symbol {
            '(' => res += 1,
            '[' => res += 2,
            '{' => res += 3,
            '<' => res += 4,
            _ => panic!("unexpected {}", symbol),
        }
    }
    res
}
