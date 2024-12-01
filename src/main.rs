use crate::utils::day_setup::Utils;

mod utils;
mod day1;

fn main() {
    // Utils::new_day(1);
    let days = [day1::run];

    days.last().unwrap()();
}
