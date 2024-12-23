mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    // (16..=25).for_each(|i| {
    //     Utils::new_day(i, 24);
    // });
    let days = [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
        day7::run,
        day8::run,
        day9::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run, // INCOMPLETE
        day17::run,
        day18::run,
    ];

    days.into_iter().last().into_iter().for_each(|f| {
        f();
        println!()
    });
}
