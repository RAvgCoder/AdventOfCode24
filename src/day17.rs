use aoc_utils_rust::day_setup::Utils;
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
    Utils::run_part(part2, 2, 0, None);
}

fn part1(mut computer: Computer) -> String {
    while let Some(_) = computer.next() {}
    computer.final_result()
}

fn part2(mut computer: Computer) -> String {
    while let Some(_) = computer.next() {}
    computer.final_result()
}

#[derive(Debug, Clone)]
struct Computer {
    pc: usize,
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    output: Vec<u8>,
    instruction: Box<[u8]>,
}

impl Computer {
    fn extract_value_from_operand(&self, operand: u8) -> u32 {
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

    fn save_output(&mut self, output: u32) {
        if output == 0 {
            self.output.push(0);
        } else {
            let mut temp = output;
            while temp != 0 {
                self.output.push((temp % 10) as u8);
                temp /= 10;
            }
        }
    }

    fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
    }

    fn read_operand(&mut self) -> Option<u8> {
        let res = self.instruction.get(self.pc).copied();
        self.pc += 1;
        res
    }

    fn final_result(&self) -> String {
        self
            .output
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Iterator for Computer {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.should_halt() {
            return None;
        }

        fn get_dv(computer: &mut Computer) -> Option<u32> {
            computer.read_operand().map(|operand| {
                let operand = computer.extract_value_from_operand(operand);
                let pow = 2u32.pow(operand);
                let res = computer.reg_a / pow;
                res
            })
        }

        let instruction = self.read_opcode();

        match instruction {
            // 0 to 6
            0 => {
                get_dv(self).map(|res| self.reg_a = res);
            }
            1 => {
                self.read_operand().map(|operand| {
                    let res = self.reg_b ^ operand as u32;
                    self.reg_b = res;
                });
            }
            2 => {
                self.read_operand().map(|operand| {
                    let operand = self.extract_value_from_operand(operand);
                    let res = operand % 8;
                    self.reg_b = res;
                });
            }
            3 => {
                if self.reg_a != 0 {
                    self.read_operand()
                        .map(|operand| self.set_pc(operand as usize));
                }
            }
            4 => {
                let _ = self.read_operand(); // Read but never used
                let res = self.reg_b ^ self.reg_c;
                self.reg_b = res;
            }
            5 => {
                self.read_operand().map(|operand| {
                    let operand = self.extract_value_from_operand(operand);
                    let res = operand % 8;
                    self.save_output(res);
                });
            }
            6 => {
                get_dv(self).map(|res| self.reg_b = res);
            }
            7 => {
                get_dv(self).map(|res| self.reg_c = res);
            }
            _ => unreachable!("Invalid instruction: {}", instruction),
        }

        Some(())
    }
}

impl From<Vec<String>> for Computer {
    fn from(lines: Vec<String>) -> Self {
        let mut lines = lines.iter();
        fn reg_func_parse(lines: &mut Iter<String>) -> u32 {
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
            output: Vec::new(),
        }
    }
}
