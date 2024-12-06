use aoc_utils_rust::day_setup::Utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/3).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 3, Some(187825547));
    Utils::run_part_single(part2, 2, 3, None);
}

fn part1(memory: Memory) -> u64 {
    memory
        .fix_corruption()
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul { x, y } => x * y,
            _ => 0,
        })
        .sum()
}

fn part2(memory: Memory) -> u64 {
    let mut state = Instruction::Include;
    let mut result = 0;
    for instruction in memory.fix_corruption() {
        match instruction {
            Instruction::Ignore => {
                state = Instruction::Ignore;
            }
            Instruction::Include => {
                state = Instruction::Include;
            }
            Instruction::Mul { x, y } => match state {
                Instruction::Include => {
                    result += x * y;
                }
                Instruction::Mul { .. } => {
                    unreachable!("Mul instruction should not be set as a state")
                }
                Instruction::Ignore => {}
            },
        }
    }

    result
}

struct Memory {
    memory: Vec<String>,
}

#[derive(Debug)]
enum Instruction {
    Ignore,
    Include,
    Mul { x: u64, y: u64 },
}

impl Memory {
    fn fix_corruption(&self) -> Vec<Instruction> {
        let mut instruction = vec![Instruction::Include];
        for line in &self.memory {
            Self::decode_line(line, &mut instruction)
        }
        instruction
    }

    fn decode_line(line: &str, instruction: &mut Vec<Instruction>) {
        let mut line_iter = line.chars().peekable();
        while let Some(c) = line_iter.next() {
            match c {
                'm' => {
                    if let Some('u') = line_iter.peek() {
                        line_iter.next();
                        if let Some('l') = line_iter.peek() {
                            line_iter.next();
                            if let Some('(') = line_iter.peek() {
                                line_iter.next();
                                if let Some(x) = Self::parse_number(&mut line_iter) {
                                    if let Some(',') = line_iter.peek() {
                                        line_iter.next();
                                        if let Some(y) = Self::parse_number(&mut line_iter) {
                                            if let Some(')') = line_iter.peek() {
                                                line_iter.next();
                                                instruction.push(Instruction::Mul { x, y });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                'd' => {
                    if let Some('o') = line_iter.peek() {
                        line_iter.next();
                        match line_iter.peek() {
                            Some('(') => {
                                line_iter.next();
                                if let Some(')') = line_iter.peek() {
                                    line_iter.next();
                                    instruction.push(Instruction::Include);
                                }
                            }
                            Some('n') => {
                                // don't()
                                line_iter.next();
                                if let Some('\'') = line_iter.peek() {
                                    line_iter.next();
                                    if let Some('t') = line_iter.peek() {
                                        line_iter.next();
                                        if let Some('(') = line_iter.peek() {
                                            line_iter.next();
                                            if let Some(')') = line_iter.peek() {
                                                line_iter.next();
                                                instruction.push(Instruction::Ignore);
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn parse_number(line_iter: &mut std::iter::Peekable<std::str::Chars>) -> Option<u64> {
        let mut number = None;
        while let Some(c) = line_iter.peek() {
            if c.is_digit(10) {
                if number.is_none() {
                    number = Some(0);
                }
                number = Some(number.unwrap() * 10 + c.to_digit(10).unwrap() as u64);
                line_iter.next();
            } else {
                return number;
            }
        }
        number
    }
}

impl From<Vec<String>> for Memory {
    fn from(value: Vec<String>) -> Self {
        Self { memory: value }
    }
}
