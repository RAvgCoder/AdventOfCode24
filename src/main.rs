use crate::utils::day_setup::Utils;

mod day1;
mod utils;

fn main() {
    // Utils::new_day(1);
    let days = [day1::run];

    days.last().unwrap()();
}
