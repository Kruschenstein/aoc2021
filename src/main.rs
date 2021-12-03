mod days;

use days::*;
use std::io::Result;

fn main() -> Result<()> {
    println!("day 1.1 {}", day1::solve_day_p1("resources/day1")?);
    println!("day 1.2 {}", day1::solve_day_p2("resources/day1")?);
    println!("day 2.1 {}", day2::solve_day_p1("resources/day2")?);
    println!("day 2.2 {}", day2::solve_day_p2("resources/day2")?);
    Ok(())
}
