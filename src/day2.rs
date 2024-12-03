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
    // 712 Too high
    // Low
    // 586
    // 591
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 2, Some(572));
    Utils::run_part_single(part2, 2, 0, None);
}

fn part1(input: Report) -> usize {
    input
        .levels
        .into_iter()
        .filter_map(|nums| {
            let mut ordering = nums[1].cmp(&nums[0]);
            if !(1..=3).contains(&nums[1].abs_diff(nums[0])) {
                return None;
            }
            nums[1..]
                .windows(2)
                .all(|window| match window {
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
                .then(|| ())
        })
        .count()
}

fn part2(input: Report) -> usize {
    input
        .levels
        .into_iter()
        .filter_map(|nums| {
            let mut decr_stack: Vec<u8> = vec![];
            let mut decr_pop = 0;
            let mut d_push = 0;
            for e in nums.iter().rev() {
                match decr_stack.last() {
                    Some(last) => {
                        match (last.cmp(e), (1..=3).contains(&(*last as i32 - *e as i32))) {
                            (Ordering::Greater, true) => {
                                d_push += 1;
                                decr_stack.push(*e);
                            }
                            (Ordering::Equal, _) => {
                                decr_pop += 1;
                                // d_push += 1;
                                // decr_stack.push(*e);
                            }
                            _ => {
                                decr_pop += 1;
                                decr_stack.pop();
                            }
                        }
                        //
                        // if *last > *e && (1..=3).contains(&(last - e)) {
                        //     d_push += 1;
                        //     decr_stack.push(*e);
                        // } else {
                        //     decr_pop += 1;
                        //     decr_stack.pop();
                        // }
                    }
                    // Base case empty
                    None => {
                        d_push += 1;
                        decr_stack.push(*e);
                    }
                }
                println!("decrC: {:?}, decrS: {:?}", (decr_pop, d_push), decr_stack);
            }

            // println!("-----------------------");
            let mut i_push = 0;
            let mut incr_stack = vec![];
            let mut incr_pop = 0;
            for e in nums.clone().into_iter().rev() {
                match incr_stack.last() {
                    Some(last) => {
                        match (e.cmp(last), (1..=3).contains(&(e as i32 - *last as i32))) {
                            (Ordering::Greater, true) => {
                                i_push += 1;
                                incr_stack.push(e);
                            }
                            (Ordering::Equal, _) => {
                                incr_pop += 1;
                                // i_push += 1;
                                // incr_stack.push(e);
                            }
                            _ => {
                                incr_pop += 1;
                                incr_stack.pop();
                            }
                        }
                        //
                        // if *last < e && (1..=3).contains(&(e - last)) {
                        //     i_push += 1;
                        //     incr_stack.push(e);
                        // } else {
                        //     incr_pop += 1;
                        //     incr_stack.pop();
                        // }
                    }
                    None => {
                        i_push += 1;
                        incr_stack.push(e);
                    }
                }
                println!("incrC: {:?}, incrS: {:?}", (incr_pop, i_push), incr_stack);
            }

            print!(
                "incr: ({:?}, {}), decr: ({:?}, {})",
                incr_pop, i_push, decr_pop, d_push
            );
            if incr_pop <= 1 || decr_pop <= 1 {
                println!(" GOOD\t| {:?}", nums.clone());
                println!("-----------------------");
                Some(())
            } else {
                println!(" BAD\t| {:?}", nums.clone());
                println!("-----------------------");
                None
            }
        })
        .count()
}

struct Report {
    levels: Vec<Vec<u8>>,
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
