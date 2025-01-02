use aoc_utils_rust::day_setup::Utils;
use std::iter::Sum;
use std::ops::{Add, Deref};
use std::slice::Iter;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/17).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 17, Some("4,1,5,3,1,5,3,5,7".into()));
    Utils::run_part(part2, 2, 17, Some(164542125272765));
}

fn part1(mut computer: Computer) -> String {
    computer.run_all().display()
}

fn part2(mut computer: Computer) -> u64 {
    computer.find_starting_a_reg()
}

#[derive(Debug, Clone)]
struct Computer {
    pc: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    instruction: Box<[u8]>,
}

impl Computer {
    fn extract_value_from_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as _,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!("Invalid operand: {}", operand),
        }
    }

    fn should_halt(&self) -> bool {
        self.pc >= self.instruction.len()
    }

    fn read_opcode(&mut self) -> u8 {
        let res = self.instruction[self.pc];
        self.pc += 1;
        res
    }

    fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
    }

    fn read_operand(&mut self) -> Result<u8, ()> {
        let res = self.instruction.get(self.pc).copied().ok_or(());
        self.pc += 1;
        res
    }

    fn run_all(&mut self) -> Output {
        self.flatten().sum::<Output>()
    }

    fn reset(&mut self, reg_a: u64) {
        self.reg_a = reg_a;
        self.reg_b = 0;
        self.reg_c = 0;
        self.pc = 0;
    }

    fn find_starting_a_reg(&mut self) -> u64 {
        let mut valid_values = vec![0];

        for &instr in self.instruction.clone().iter().rev() {
            let mut next_vals = Vec::new();

            for a in &valid_values {
                let shifted_a = a * 8;

                for candidate in shifted_a..shifted_a + 8 {
                    self.reset(candidate);
                    let out = self.run_all();
                    if let Some(&first) = out.first() {
                        if first == instr {
                            next_vals.push(candidate);
                        }
                    }
                }
            }

            valid_values = next_vals
        }

        *valid_values.iter().min().unwrap()
    }
}

#[derive(Debug)]
struct Output(Vec<u8>);

impl Output {
    fn new(result: u64) -> Self {
        if result == 0 {
            Self(vec![0])
        } else {
            let mut temp = result;
            let mut result = Vec::new();
            while temp != 0 {
                result.push((temp % 10) as u8);
                temp /= 10;
            }
            Self(result)
        }
    }

    fn display(self) -> String {
        self.0
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Deref for Output {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sum for Output {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Output(Vec::new()), |acc, e| acc + e)
    }
}

impl Add for Output {
    type Output = Output;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0.extend(rhs.0);
        self
    }
}

impl Iterator for Computer {
    type Item = Option<Output>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.should_halt() {
            return None;
        }

        fn dv(computer: &mut Computer) -> Result<u64, ()> {
            computer.read_operand().map(|operand| {
                let operand = computer.extract_value_from_operand(operand);
                computer.reg_a / 2u64.pow(operand as u32)
            })
        }

        let mut result = None;
        let instruction = self.read_opcode();

        match instruction {
            0 => {
                let _ = dv(self).map(|res| self.reg_a = res);
            }
            1 => {
                let _ = self.read_operand().map(|operand| {
                    let res = self.reg_b ^ operand as u64;
                    self.reg_b = res;
                });
            }
            2 => {
                let _ = self.read_operand().map(|operand| {
                    let operand = self.extract_value_from_operand(operand);
                    self.reg_b = operand % 8;
                });
            }
            3 => {
                if self.reg_a != 0 {
                    let _ = self
                        .read_operand()
                        .map(|operand| self.set_pc(operand as usize));
                }
            }
            4 => {
                let _ = self.read_operand(); // Read but never used
                self.reg_b ^= self.reg_c;
            }
            5 => {
                let _ = self.read_operand().map(|operand| {
                    let operand = self.extract_value_from_operand(operand);
                    result = Some(Output::new(operand % 8));
                });
            }
            6 => {
                let _ = dv(self).map(|res| self.reg_b = res);
            }
            7 => {
                let _ = dv(self).map(|res| self.reg_c = res);
            }
            _ => unreachable!("Invalid instruction: {}", instruction),
        }

        Some(result)
    }
}

impl From<Vec<String>> for Computer {
    fn from(lines: Vec<String>) -> Self {
        let mut lines = lines.iter();
        fn reg_func_parse(lines: &mut Iter<String>) -> u64 {
            lines
                .next()
                .unwrap()
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .parse()
                .unwrap()
        }

        let reg_a = reg_func_parse(&mut lines);
        let reg_b = reg_func_parse(&mut lines);
        let reg_c = reg_func_parse(&mut lines);
        let _ = lines.next(); // Skip the empty line
        let instruction = lines
            .next()
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .split(',')
            .map(|line| line.trim().parse().unwrap())
            .collect::<Box<[u8]>>();

        Self {
            reg_a,
            reg_b,
            reg_c,
            instruction,
            pc: 0,
        }
    }
}
