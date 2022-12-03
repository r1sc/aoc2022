mod aoc;
mod days;

use clap::Parser;

#[derive(Parser, Debug)]
enum DaySelection {
    Day1,
    Day2,
    Day3
}

fn main() {
    let day = DaySelection::parse();

    let now = std::time::Instant::now();

    let result = match day {
        DaySelection::Day1 => days::day1(),
        DaySelection::Day2 => days::day2(),
        DaySelection::Day3 => days::day3(),
    };

    let elapsed = now.elapsed();

    println!("Completed in {:?}. Result: {:?}", elapsed, result);
}
