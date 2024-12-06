use aoc_utils_rust::day_setup::Utils;
use std::iter::Peekable;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/4).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 4, Some(2414));
    Utils::run_part_single(part2, 2, 4, Some(1871));
}

fn part1(word_search: WordSearch) -> u16 {
    word_search.find_all_xmas_instances()
}

fn part2(word_search: WordSearch) -> u16 {
    word_search.find_all_x_mas_instances()
}

#[derive(Debug)]
struct WordSearch {
    words: Vec<String>,
}

impl WordSearch {
    fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    fn find_all_x_mas_instances(&self) -> u16 {
        (0..self.words.len())
            .map(|i| Self::find_sub_string_instances_x_mas_diagonally(i, &self.words))
            .sum()
    }

    fn find_sub_string_instances_x_mas_diagonally(row: usize, input: &[String]) -> u16 {
        let mut counter = 0;
        const VALID: [char; 2] = ['M', 'S'];
        // Top
        let higher_bound = 1;
        let lower_bound = input.len() - 1;
        if (higher_bound..lower_bound).contains(&row) {
            // Left
            for (i, e) in input[row]
                .chars()
                .enumerate()
                .take(lower_bound)
                .skip(higher_bound)
            {
                if e != 'A' {
                    continue;
                }
                let mut x_mas_set = [[false, false], [false, false]]; // [(top_left to bottom_right)[M, S], (top_right to bottom left)[M, S]]

                // top_left -> bottom_right
                let tl = input[row - 1].chars().nth(i - 1).unwrap();
                let br = input[row + 1].chars().nth(i + 1).unwrap();
                if VALID.contains(&tl) && VALID.contains(&br) {
                    if tl == 'M' {
                        x_mas_set[0][0] = true
                    } else {
                        x_mas_set[0][1] = true
                    }

                    if br == 'M' {
                        x_mas_set[0][0] = true
                    } else {
                        x_mas_set[0][1] = true
                    }
                } else {
                    continue;
                }

                // top_right -> bottom_left
                let tr = input[row - 1].chars().nth(i + 1).unwrap();
                let bl = input[row + 1].chars().nth(i - 1).unwrap();
                if VALID.contains(&tr) && VALID.contains(&bl) {
                    if tr == 'M' {
                        x_mas_set[1][0] = true
                    } else {
                        x_mas_set[1][1] = true
                    }

                    if bl == 'M' {
                        x_mas_set[1][0] = true
                    } else {
                        x_mas_set[1][1] = true
                    }
                } else {
                    continue;
                }

                // If forms an X_MAS
                if x_mas_set.iter().flatten().all(|x| *x) {
                    counter += 1
                }
            }
        }

        counter
    }

    const XMAS: &'static str = "XMAS";
    fn find_all_xmas_instances(&self) -> u16 {
        self.words
            .iter()
            .enumerate()
            .map(|(i, word)| {
                Self::find_sub_string_instances_row(word.chars().peekable())
                    + Self::find_sub_string_instances_row(word.chars().rev().peekable())
                    + Self::find_sub_string_instances_col(i, &self.words)
                    + Self::find_sub_string_instances_xmas_diagonally(i, &self.words)
            })
            .sum()
    }

    fn find_sub_string_instances_row<T>(mut word: Peekable<T>) -> u16
    where
        T: Iterator<Item = char>,
    {
        let mut counter = 0;

        'instance_search: while let Some(curr_char) = word.next() {
            if curr_char == Self::XMAS.chars().next().unwrap() {
                for x_char in Self::XMAS.chars().skip(1) {
                    match word.peek() {
                        Some(&curr_char) => {
                            if curr_char != x_char {
                                continue 'instance_search;
                            }
                            word.next();
                        }
                        None => break 'instance_search,
                    }
                }
                counter += 1;
            }
        }

        counter
    }

    fn find_sub_string_instances_xmas_diagonally(row: usize, input: &[String]) -> u16 {
        let mut counter = 0;
        // Top
        if row >= 3 {
            // Left
            for (i, e) in input[row].chars().enumerate().skip(3) {
                if e == Self::XMAS.chars().next().unwrap() {
                    if Self::XMAS
                        == format!(
                            "X{}{}{}",
                            input[row - 1].chars().nth(i - 1).unwrap(),
                            input[row - 2].chars().nth(i - 2).unwrap(),
                            input[row - 3].chars().nth(i - 3).unwrap(),
                        )
                    {
                        counter += 1;
                    }
                }
            }

            // Right
            for (i, e) in input[row].chars().enumerate().take(input.len() - 3) {
                if e == Self::XMAS.chars().next().unwrap() {
                    if Self::XMAS
                        == format!(
                            "X{}{}{}",
                            input[row - 1].chars().nth(i + 1).unwrap(),
                            input[row - 2].chars().nth(i + 2).unwrap(),
                            input[row - 3].chars().nth(i + 3).unwrap(),
                        )
                    {
                        counter += 1;
                    }
                }
            }
        }

        // Bottom
        if row < input.len() - 3 {
            // Left
            for (i, e) in input[row].chars().enumerate().skip(3) {
                if e == Self::XMAS.chars().next().unwrap() {
                    if Self::XMAS
                        == format!(
                            "X{}{}{}",
                            input[row + 1].chars().nth(i - 1).unwrap(),
                            input[row + 2].chars().nth(i - 2).unwrap(),
                            input[row + 3].chars().nth(i - 3).unwrap(),
                        )
                    {
                        counter += 1;
                    }
                }
            }

            // Right
            for (i, e) in input[row].chars().enumerate().take(input.len() - 3) {
                if e == Self::XMAS.chars().next().unwrap() {
                    if Self::XMAS
                        == format!(
                            "X{}{}{}",
                            input[row + 1].chars().nth(i + 1).unwrap(),
                            input[row + 2].chars().nth(i + 2).unwrap(),
                            input[row + 3].chars().nth(i + 3).unwrap(),
                        )
                    {
                        counter += 1;
                    }
                }
            }
        }

        counter
    }

    fn find_sub_string_instances_col(row: usize, input: &[String]) -> u16 {
        let mut counter = 0;

        // Check UP
        if row >= 3 {
            for (i, e) in input[row].chars().enumerate() {
                if e == Self::XMAS.chars().next().unwrap() {
                    if Self::XMAS
                        == format!(
                            "X{}{}{}",
                            input[row - 1].chars().nth(i).unwrap(),
                            input[row - 2].chars().nth(i).unwrap(),
                            input[row - 3].chars().nth(i).unwrap(),
                        )
                    {
                        counter += 1;
                    }
                }
            }
        }

        // Check DOWN
        if row < input.len() - 3 {
            for (i, e) in input[row].chars().enumerate() {
                if e == Self::XMAS.chars().next().unwrap() {
                    if Self::XMAS
                        == format!(
                            "X{}{}{}",
                            input[row + 1].chars().nth(i).unwrap(),
                            input[row + 2].chars().nth(i).unwrap(),
                            input[row + 3].chars().nth(i).unwrap(),
                        )
                    {
                        counter += 1;
                    }
                }
            }
        }

        counter
    }
}

impl From<Vec<String>> for WordSearch {
    fn from(value: Vec<String>) -> Self {
        Self::new(value)
    }
}
