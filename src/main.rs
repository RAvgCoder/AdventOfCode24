use crate::utils::day_setup::Utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

fn main() {
    // Utils::new_day(4);
    let days = [day1::run, day2::run, day3::run, day4::run];

    days.last().unwrap()();
}
