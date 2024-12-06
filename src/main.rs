use aoc_utils_rust::day_setup::Utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    // Utils::new_day(6, 2024);

    let days = [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day6::run,
        day5::run,
    ];

    days.last().unwrap()();
}
