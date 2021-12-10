use super::util::read_file_lines;
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
                if pair == '(' && c != ')'
                    || pair == '[' && c != ']'
                    || pair == '{' && c != '}'
                    || pair == '<' && c != '>'
                {
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
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
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
