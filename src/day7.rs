use aoc_utils_rust::day_setup::Utils;
use std::fmt;
use std::fmt::Formatter;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/7).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 7, Some(882304362421));
    Utils::run_part(part2, 2, 7, Some(145149066755184));
}

fn part1(op_seq: OperationSequence) -> u64 {
    op_seq.sum_valid_equations(&[
        Operation::Add,
        Operation::Multiply,
    ])
}

fn part2(op_seq: OperationSequence) -> u64 {
    op_seq.sum_valid_equations(&[
        Operation::Add,
        Operation::Multiply,
        Operation::Concat,
    ])
}


struct OperationSequence {
    operations: Vec<(u64, Vec<u32>)>, // (Target: u32, Sources: Vec<u32>)
}

enum Operation {
    Add,
    Multiply,
    Concat
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            Self::Concat => { 
                // Similar to format!("{}{}", a, b).parse().unwrap() but 2x slower
                let mut a = a;
                let mut b_temp = b;
                while b_temp != 0 {
                    a *= 10;
                    b_temp /= 10;
                }
                a + b
            },
        }
    }
}

impl OperationSequence {
    fn sum_valid_equations(&self, operations: &[Operation]) -> u64 {
        self.operations
            .iter()
            .filter(|(target, sources)| Self::is_possible(sources, *target, operations))
            .map(|(target, _)| *target)
            .sum()
    }

    fn is_possible(sources: &[u32], target: u64, operations: &[Operation]) -> bool {
        use std::collections::HashMap;
        
        fn check(
            i_ptr: usize,
            acc: u64,
            source: &[u32],
            target: u64,
            operations: &[Operation],
            memo: &mut HashMap<(usize, u64), bool>,
        ) -> bool {
            // Base case: If we've considered all sources, check if we hit the target
            if source.is_empty() {
                return target == acc;
            }

            // Early exit: If the current total already exceeds the target, no need to proceed
            if acc > target {
                return false;
            }

            // Check memoized results
            if let Some(&result) = memo.get(&(i_ptr, acc)) {
                return result;
            }

            // Explore both adding and multiplying the current source
            let new_source = &source[1..];
            let result = operations.iter().any(|op| {
                let new_acc = op.apply(acc, source[0] as u64);
                check(i_ptr + 1, new_acc, new_source, target, operations, memo)
            });
            
            memo.insert((i_ptr, acc), result);
            result
        }

        check(0, sources[0] as u64, &sources[1..], target, operations, &mut HashMap::new())
    }
}

impl fmt::Debug for OperationSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Operations:")?;
        for (target, sources) in &self.operations {
            writeln!(f, "{}: {:?}", target, sources)?;
        }
        Ok(())
    }
}

impl From<Vec<String>> for OperationSequence {
    fn from(value: Vec<String>) -> Self {
        Self {
            operations: value
                .into_iter()
                .map(|line| {
                    let (target, vals) = line.split_once(':').unwrap();
                    (
                        target.parse().unwrap(),
                        vals.split_whitespace()
                            .map(|e| e.parse().unwrap())
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}
