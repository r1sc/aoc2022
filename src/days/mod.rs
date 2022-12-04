mod day1;
mod day2;
mod day3;

mod day4;

use counted_array::counted_array;

counted_array!(
    pub const ALL_DAYS: [fn() -> (u32, u32); _] = [
        day1::run,
        day2::run,
        day3::run,
        day4::run
    ]
);
