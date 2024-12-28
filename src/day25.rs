use aoc_utils_rust::day_setup::Utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/25).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 25, Some(2691));
    // Utils::run_part(part2, 2, 25, Some("Completed AOC 2024!"));
}

fn part1(input: Vec<String>) -> u32 {
    let (lock, key) = parse_input(input);
    lock.iter().map(|l| l.count_keys_that_match(&key)).sum()
}

fn part2(_: Vec<String>) -> &'static str {
    "Completed AOC 2024!"
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Lock([u8; 5]);

impl Lock {
    fn count_keys_that_match(&self, keys: &[Key]) -> u32 {
        keys.iter()
            .filter(|&key| self.0.iter().zip(key.0).map(|(a, b)| a + b).all(|x| x <= 5))
            .count() as u32
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Key([u8; 5]);

fn parse_input(input: Vec<String>) -> (Vec<Lock>, Vec<Key>) {
    let mut input = input.into_iter();
    let mut locks = vec![];
    let mut keys = vec![];
    loop {
        let first = input.next().unwrap();

        let mut structure = [0; 5];

        for line in [
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        ] {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    structure[i] += 1;
                }
            }
        }

        if first.chars().nth(0).unwrap() == '.' {
            keys.push(Key(structure));
        } else {
            locks.push(Lock(structure));
        }

        let _ = input.next().unwrap();

        if input.next().is_none() {
            break;
        }
    }

    (locks, keys)
}
