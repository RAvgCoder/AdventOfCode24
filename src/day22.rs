use aoc_utils_rust::day_setup::Utils;
use std::ops::Deref;

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
    Utils::run_part(part2, 2, 0, None);
}

fn part1(input: Vec<String>) -> u64 {
    const NTH_SECRET_NUMBER: usize = 2000;

    let mut buyers_secret_number = input
        .iter()
        .map(|e| SecretNumber(e.parse().unwrap()))
        .collect::<Vec<_>>();

    buyers_secret_number.iter_mut().for_each(|secret_number| {
        secret_number.take(NTH_SECRET_NUMBER).for_each(drop);
    });

    buyers_secret_number.iter().map(SecretNumber::deref).sum()
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

struct SecretNumber(u64);

impl Deref for SecretNumber {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Iterator for SecretNumber {
    type Item = ();

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

        Some(())
    }
}
