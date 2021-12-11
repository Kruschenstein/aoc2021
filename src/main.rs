mod days;

use days::*;
use std::io::Result;

fn main() -> Result<()> {
    println!("day 1.1 {}", day1::solve_p1("resources/day1")?);
    println!("day 1.2 {}", day1::solve_p2("resources/day1")?);
    println!("day 2.1 {}", day2::solve_p1("resources/day2")?);
    println!("day 2.2 {}", day2::solve_p2("resources/day2")?);
    println!("day 3.1 {}", day3::solve_p1("resources/day3")?);
    println!("day 3.2 {}", day3::solve_p2("resources/day3")?);
    println!("day 4.1 {}", day4::solve_p1("resources/day4")?);
    println!("day 4.2 {}", day4::solve_p2("resources/day4")?);
    println!("day 5.1 {}", day5::solve_p1("resources/day5")?);
    println!("day 5.2 {}", day5::solve_p2("resources/day5")?);
    println!("day 6.1 {}", day6::solve_p1("resources/day6")?);
    println!("day 6.2 {}", day6::solve_p2("resources/day6")?);
    println!("day 7.1 {}", day7::solve_p1("resources/day7")?);
    println!("day 7.2 {}", day7::solve_p2("resources/day7")?);
    println!("day 8.1 {}", day8::solve_p1("resources/day8")?);
    println!("day 8.2 {}", day8::solve_p2("resources/day8")?);
    println!("day 9.1 {}", day9::solve_p1("resources/day9")?);
    println!("day 9.2 {}", day9::solve_p2("resources/day9")?);
    println!("day 10.1 {}", day10::solve_p1("resources/day10")?);
    println!("day 10.2 {}", day10::solve_p2("resources/day10")?);
    println!("day 11.1 {}", day11::solve_p1("resources/day11")?);
    println!("day 11.2 {}", day11::solve_p2("resources/day11")?);

    Ok(())
}
