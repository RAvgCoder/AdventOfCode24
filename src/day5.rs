use aoc_utils_rust::day_setup::Utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/202021/day/5).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 0, None);
    Utils::run_part(part2, 2, 0, None);
}

fn part1(input: Vec<String>) -> u64 {
    println!("Part 1: {:#?}", input);
    0
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}
