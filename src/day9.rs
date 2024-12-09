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
    Utils::run_part(part2, 2, 9, Some(6448168620520));
}

fn part1(disk_map: DiskMap) -> usize {
    disk_map.fragmented_check_sum()
}

fn part2(disk_map: DiskMap) -> u64 {
    disk_map.un_fragmented_check_sum()
}

#[derive(Debug)]
struct DiskMap {
    maps: Vec<u8>,
}

impl DiskMap {
    fn un_fragmented_check_sum(self) -> u64 {
        let mut disc = self.generate_disk();
        // (idx_in_disc, num_of_vals, nums)
        let mut map_with_indices: Vec<(usize, (u8, u32))> = Vec::with_capacity(self.maps.len());
        let _ =
            self.maps
                .iter()
                .enumerate()
                .fold((0, 0), |(acc_idx, gen_num), (map_idx, &count)| {
                    map_with_indices.push((
                        acc_idx,
                        (
                            count,
                            if map_idx % 2 == 1 {
                                // Skip numbers
                                0
                            } else {
                                gen_num
                            },
                        ),
                    ));
                    (
                        acc_idx + count as usize,
                        if map_idx % 2 == 1 {
                            gen_num + 1 // Increment the number if it's on a space to prepare for the next number
                        } else {
                            gen_num // Keep the number the same if it's on a number
                        },
                    )
                });

        let mut r_map_ptr = map_with_indices.len() - 1;

        while r_map_ptr > 0 {
            let (r_idx, (r_count, num)) = map_with_indices[r_map_ptr];
            if let Some((l_map_idx, &(disc_idx, (_, _)))) = map_with_indices[..r_map_ptr]
                .iter()
                .enumerate()
                .skip(1) // Start at odd index
                .step_by(2) // Only loop through odd indices
                .find(|&(_, &(_, (l_count, _)))| l_count >= r_count)
            // Find a space that can hold the values
            {
                // Clear the moved values
                for c in disc[r_idx..].iter_mut().take(r_count as usize) {
                    *c = 0
                }

                // Add the moved value
                for c in disc[disc_idx..].iter_mut().take(r_count as usize) {
                    *c = num
                }
                let (l_idx, (l_count, _)) = &mut map_with_indices[l_map_idx];
                *l_idx += r_count as usize;
                *l_count -= r_count;
            }
            r_map_ptr -= 2;
        }

        disc.iter()
            .enumerate()
            .map(|(i, e)| if *e == 0 { 0 } else { i as u64 * *e as u64 })
            .sum()
    }

    fn generate_disk(&self) -> Vec<u32> {
        let mut disk = vec![0; self.maps.iter().map(|e| *e as usize).sum()];
        let mut num = 0;
        let mut idx = 0_usize;
        for (i, &e) in self.maps.iter().enumerate() {
            if i % 2 == 1 {
                // Skips spaces
                idx += e as usize
            } else {
                for _ in 0..e {
                    disk[idx] = num;
                    idx += 1
                }
                num += 1;
            }
        }

        disk
    }

    fn fragmented_check_sum(mut self) -> usize {
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
        let maps = input[0].chars().map(|s| s as u8 - b'0').collect::<Vec<_>>();
        Self { maps }
    }
}
