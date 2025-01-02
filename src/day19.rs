use aoc_utils_rust::day_setup::Utils;
use std::collections::HashMap;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/19).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 19, Some(353));
    Utils::run_part(part2, 2, 19, Some(880877787214477));
}

fn part1(input: Vec<String>) -> usize {
    let patterns = input[0].split(",").map(str::trim).collect::<Vec<_>>();
    let towels = &input[2..];

    let mut map = HashMap::new();

    towels
        .iter()
        .filter(|towel| is_valid_design(towel, &patterns, &mut map))
        .count()
}

fn part2(input: Vec<String>) -> u64 {
    let patterns = input[0].split(",").map(str::trim).collect::<Vec<_>>();
    let towels = &input[2..];

    let mut map = HashMap::new();

    towels
        .iter()
        .map(|towel| count_valid_designs(towel, &patterns, &mut map))
        .sum()
}

fn count_valid_designs<'a>(
    towel: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(result) = cache.get(towel) {
        return *result;
    } else if towel.is_empty() {
        return 1;
    }

    let mut count = 0;
    for pattern in patterns {
        if towel.starts_with(pattern) {
            let rest = &towel[pattern.len()..];
            count += count_valid_designs(rest, patterns, cache);
        }
    }

    cache.insert(towel, count);
    count
}

fn is_valid_design<'a>(
    towel: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(result) = cache.get(towel) {
        return *result;
    } else if towel.is_empty() {
        return true;
    }

    for pattern in patterns {
        if towel.starts_with(pattern) {
            let rest = &towel[pattern.len()..];
            if is_valid_design(rest, patterns, cache) {
                cache.insert(towel, true);
                return true;
            }
        }
    }

    cache.insert(towel, false);
    false
}
