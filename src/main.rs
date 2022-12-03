mod aoc;
mod days;

use clap::Parser;

#[derive(Parser, Debug)]
struct DaySelection {
    day: usize
}

fn main() {
    let day = DaySelection::parse().day - 1;

    let now = std::time::Instant::now();
    
    let result = days::ALL_DAYS[day]();

    let elapsed = now.elapsed();

    println!("Completed in {:?}. Result: {:?}", elapsed, result);
}
