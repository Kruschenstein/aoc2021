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
    Ok(())
}
