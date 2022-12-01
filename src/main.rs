mod aoc;
mod days;

use clap::Parser;

#[derive(Parser, Debug)]
enum DaySelection {
    Day1,
}

fn main() {
    let day = DaySelection::parse();

    let now = std::time::Instant::now();

    let result = match day {
        DaySelection::Day1 => days::day1(),
    };

    let elapsed = now.elapsed();

    println!("Completed in {:?}. Result: {:?}", elapsed, result);
}
