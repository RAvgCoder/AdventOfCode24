use crate::utils::day_setup::Utils;
use std::cmp::Ordering;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/2).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 2, Some(572));
    Utils::run_part_single(part2, 2, 2, Some(612));
}

fn part1(input: Report) -> usize {
    input
        .levels
        .into_iter()
        .filter(|nums| Report::is_level_ok(&nums))
        .count()
}

fn part2(input: Report) -> usize {
    input
        .levels
        .into_iter()
        .filter(|nums| {
            // Check if the report is valid as-is
            if Report::is_level_ok(nums) {
                return true;
            }
            // Otherwise, check by removing one level at a time
            nums.iter().enumerate().any(|(i, _)| {
                let mut temp = nums.clone();
                temp.remove(i);
                Report::is_level_ok(&temp)
            })
        })
        .count()
}

struct Report {
    levels: Vec<Vec<u8>>,
}

impl Report {
    fn is_level_ok(level: &[u8]) -> bool {
        let mut ordering = level[1].cmp(&level[0]);
        if !(1..=3).contains(&level[1].abs_diff(level[0])) {
            return false;
        }
        level[1..].windows(2).all(|window| match window {
            [a, b] => match (ordering, b.cmp(a)) {
                (Ordering::Greater, Ordering::Greater) => {
                    ordering = Ordering::Greater;
                    (1..=3).contains(&(b - a))
                }
                (Ordering::Less, Ordering::Less) => {
                    ordering = Ordering::Less;
                    (1..=3).contains(&(a - b))
                }
                _ => false,
            },
            _ => unreachable!(),
        })
    }
}

impl From<Vec<String>> for Report {
    fn from(value: Vec<String>) -> Self {
        Self {
            levels: value
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|e| e.parse::<u8>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
}
