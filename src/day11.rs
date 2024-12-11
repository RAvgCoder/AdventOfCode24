use aoc_utils_rust::day_setup::Utils;
use std::collections::HashMap;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/11).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 11, Some(194557));
    Utils::run_part(part2, 2, 11, Some(231532558973909));
}

fn part1(stones: Stones) -> u64 {
    stones.blink_n_times(25)
}

fn part2(stones: Stones) -> u64 {
    stones.blink_n_times(75)
}

#[derive(Debug)]
struct Stones {
    stones: Vec<i32>,
}

#[derive(Debug)]
enum NextDigit {
    Double(u64, u64),
    Single(u64),
}

impl Stones {
    fn blink_n_times(self, blink_times: u8) -> u64 {
        // (stone, blink_times) -> result
        let mut cache: HashMap<(u64, u8), u64> = HashMap::new();
        self.stones
            .into_iter()
            .map(|stone| Self::sim(&mut cache, stone as u64, blink_times))
            .sum()
    }

    fn next_digit(n: u64) -> NextDigit {
        if n == 0 {
            NextDigit::Single(1)
        } else {
            // Count num of digits in n
            let mut count = 0;

            {
                let mut temp = n;
                while temp > 0 {
                    temp /= 10;
                    count += 1;
                }
            }

            if count % 2 == 1 {
                NextDigit::Single(n * 2024)
            } else {
                let d = 10u64.pow(count / 2);
                let left = n / d;
                let right = n % d;
                NextDigit::Double(left, right)
            }
        }
    }

    fn sim(cache: &mut HashMap<(u64, u8), u64>, stone: u64, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }
        if let Some(&result) = cache.get(&(stone, depth)) {
            return result;
        }
        let new_digit = Self::next_digit(stone);
        let result = match new_digit {
            NextDigit::Single(n) => Self::sim(cache, n, depth - 1),
            NextDigit::Double(left, right) => {
                Self::sim(cache, left, depth - 1) + Self::sim(cache, right, depth - 1)
            }
        };
        cache.insert((stone, depth), result);
        result
    }
}

impl From<Vec<String>> for Stones {
    fn from(value: Vec<String>) -> Self {
        Self {
            stones: value
                .first()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}
