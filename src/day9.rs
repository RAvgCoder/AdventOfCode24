use aoc_utils_rust::day_setup::Utils;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/9).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 9, Some(6421128769094));
    Utils::run_part(part2, 2, 0, None);
}

fn part1(disk_map: DiskMap) -> usize {
    disk_map.check_sum()
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

#[derive(Debug)]
struct DiskMap {
    maps: Vec<u8>,
}

impl DiskMap {
    fn check_sum(mut self) -> usize {
        let mut check_sum = 0;

        let mut l_ptr = 0;
        let mut r_ptr = self.maps.len() - 1;

        let mut l_num = 0;
        let mut r_num = self.maps.len() / 2;

        let mut count = 0;

        while l_ptr <= r_ptr {
            // Skip odd numbers coming from the left
            if r_ptr % 2 == 1 {
                r_ptr -= 1;
                continue;
            }

            let l = self.maps[l_ptr];
            let r = self.maps[r_ptr];

            match (l, r) {
                (0, _) => {
                    l_ptr += 1;
                    // Increment the number of the left
                    if l_ptr % 2 == 0 {
                        l_num += 1;
                    }
                }
                (_, 0) => {
                    r_ptr -= 1;
                    // Decrement the number of the right
                    r_num -= 1;
                }
                _ => {
                    if l_ptr % 2 == 1 {
                        // Add nums from the right to checksum
                        check_sum += r_num * count;
                        self.maps[r_ptr] -= 1;
                        self.maps[l_ptr] -= 1;
                        count += 1;
                    } else {
                        // Add nums from the left to checksum
                        check_sum += l_num * count;
                        self.maps[l_ptr] -= 1;
                        count += 1;
                    }
                }
            }
        }

        check_sum
    }
}

impl From<Vec<String>> for DiskMap {
    fn from(input: Vec<String>) -> Self {
        let maps = input[0]
            .chars()
            .map(|s| s as u8 - '0' as u8)
            .collect::<Vec<_>>();
        Self { maps }
    }
}
