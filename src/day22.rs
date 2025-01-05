use aoc_utils_rust::day_setup::Utils;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/22).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 22, Some(13429191512));
    Utils::run_part(part2, 2, 22, Some(1582));
}

fn part1(mut buyers: Buyers) -> u64 {
    buyers.iter_mut().map(SecretNumber::next_2000).sum()
}

fn part2(mut buyers: Buyers) -> u64 {
    let mut dict = HashMap::new();
    buyers.iter_mut().for_each(|secret_number| {
        add_to_diff_dict(secret_number, &mut dict);
    });
    *dict.values().max().unwrap() as u64
}

fn add_to_diff_dict(secret_number: &mut SecretNumber, diff_dict: &mut HashMap<[i8; 4], i32>) {
    const NUM_OF_BUYERS: usize = 2000;
    const WINDOW_SIZE: usize = 4;

    let mut diffs = [0i8; NUM_OF_BUYERS];
    let mut nums = [0i8; NUM_OF_BUYERS - WINDOW_SIZE + 1];

    let start = (secret_number.0 % 10) as i8;
    secret_number
        .take(NUM_OF_BUYERS)
        .map(|e| (e % 10) as i8)
        .enumerate()
        .fold(start, |acc, (i, e)| {
            diffs[i] = e - acc;
            if i as isize - (WINDOW_SIZE as isize - 1) >= 0 {
                nums[i - (WINDOW_SIZE - 1)] = e
            }
            e
        });

    let mut acc = HashMap::with_capacity(NUM_OF_BUYERS - WINDOW_SIZE + 1);

    diffs
        .windows(WINDOW_SIZE)
        .zip(nums)
        .for_each(|(window, e)| {
            if let &[a, b, c, d] = window {
                acc.entry([a, b, c, d]).or_insert(e);
            }
        });

    acc.into_iter().for_each(|(k, v)| {
        *diff_dict.entry(k).or_insert(0) += v as i32;
    });
}

struct Buyers(Box<[SecretNumber]>);
impl Deref for Buyers {
    type Target = [SecretNumber];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Buyers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
struct SecretNumber(u64);

impl Deref for SecretNumber {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SecretNumber {
    fn next_2000(&mut self) -> u64 {
        self.take(2000).last().expect("No numbers generated")
    }
}

impl Iterator for SecretNumber {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        #[inline(always)]
        fn mix(secret_num: &mut SecretNumber, next_secret_num: u64) {
            secret_num.0 ^= next_secret_num
        }

        #[inline(always)]
        fn prune(secret_num: &mut SecretNumber) {
            secret_num.0 %= 16777216;
        }

        fn mix_and_prune(secret_num: &mut SecretNumber, next_secret_num: u64) {
            mix(secret_num, next_secret_num);
            prune(secret_num);
        }

        mix_and_prune(self, self.0 * 64);
        mix_and_prune(self, self.0 / 32);
        mix_and_prune(self, self.0 * 2048);

        Some(self.0)
    }
}

impl From<Vec<String>> for Buyers {
    fn from(input: Vec<String>) -> Self {
        Buyers(
            input
                .iter()
                .map(|e| SecretNumber(e.parse().unwrap()))
                .collect(),
        )
    }
}
