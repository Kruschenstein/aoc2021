use super::util::read_file_lines;
use std::collections::HashSet;
use std::io::Result;

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;
    let points = get_points(&lines);
    let fold = lines
        .iter()
        .skip_while(|s| !s.is_empty())
        .nth(1)
        .map(|s| map_fold(s))
        .unwrap();

    Ok(fold_along(&fold, &points).len() as u32)
}

fn get_points(lines: &[String]) -> HashSet<(u32, u32)> {
    lines
        .iter()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let split = s.split(',').collect::<Vec<_>>();
            let x = split[0].parse::<u32>().unwrap();
            let y = split[1].parse::<u32>().unwrap();
            (x, y)
        })
        .collect::<HashSet<_>>()
}

fn fold_along(fold: &Fold, points: &HashSet<(u32, u32)>) -> HashSet<(u32, u32)> {
    match fold {
        Fold::X(value) => {
            let max = value * 2;
            points
                .iter()
                .map(|p @ (x, y)| if *x > *value { (max - *x, *y) } else { *p })
                .collect::<HashSet<_>>()
        }
        Fold::Y(value) => {
            let max = value * 2;
            points
                .iter()
                .map(|p @ (x, y)| if *y > *value { (*x, max - *y) } else { *p })
                .collect::<HashSet<_>>()
        }
    }
}

fn map_fold(s: &str) -> Fold {
    let s = s[11..].split('=').collect::<Vec<_>>();
    let value = s[1].parse::<u32>().unwrap();
    match s[0] {
        "x" => Fold::X(value),
        "y" => Fold::Y(value),
        _ => panic!(),
    }
}

pub fn solve_p2(filename: &str) -> Result<String> {
    let lines = read_file_lines(filename)?;
    let mut points = get_points(&lines);
    let folds = lines
        .iter()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| map_fold(s))
        .collect::<Vec<_>>();

    for fold in folds {
        points = fold_along(&fold, &points);
    }

    let mut res = "\n".to_string();

    let x_max = points.iter().max_by(|(x, _), (x1, _)| x.cmp(x1)).unwrap().0 + 1;
    let y_max = points.iter().max_by(|(_, y), (_, y1)| y.cmp(y1)).unwrap().1 + 1;
    for y in 0..y_max {
        for x in 0..x_max {
            if points.contains(&(x, y)) {
                res.push('#');
            } else {
                res.push(' ');
            }
        }
        if y < y_max - 1 {
            res.push('\n')
        }
    }

    Ok(res)
}
