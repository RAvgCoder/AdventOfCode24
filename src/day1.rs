use crate::utils::day_setup::Utils;
use std::collections::HashMap;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/1).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 1, Some(3574690));
    Utils::run_part(part2, 2, 1, Some(22565391));
}

fn part1(input: Vec<String>) -> u32 {
    let (mut list1, mut list2) = parse_input(input);

    list1.sort_unstable();
    list2.sort_unstable();

    list1.iter().zip(list2).map(|(a, b)| b.abs_diff(*a)).sum()
}

fn parse_input(input: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    input
        .iter()
        .map(|x| {
            let mut split = x.split_whitespace();
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .fold((vec![], vec![]), |(mut list1, mut list2), (a, b)| {
            list1.push(a);
            list2.push(b);
            (list1, list2)
        })
}

fn part2(input: Vec<String>) -> u32 {
    let (list1, list2) = parse_input(input);

    let mut tally_map = HashMap::with_capacity(list1.len());

    for e in list1 {
        tally_map
            .entry(e)
            .and_modify(|(tally, _)| {
                *tally += 1;
            })
            .or_insert((1, 0));
    }

    for e in list2 {
        tally_map.entry(e).and_modify(|(_, count)| {
            *count += 1;
        });
    }

    tally_map
        .iter()
        .map(|(key, (tally, count))| (key * count) * tally)
        .sum()
}
