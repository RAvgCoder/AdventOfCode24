use aoc_utils_rust::day_setup::Utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    // Utils::new_day(7, 24);

    let days = [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
        day7::run,
    ];

    days.last().unwrap()();
}
