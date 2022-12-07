mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;

use counted_array::counted_array;

counted_array!(
    pub const ALL_DAYS: [fn() -> (String, String); _] = [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day7::run
    ]
);
