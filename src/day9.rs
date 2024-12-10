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
        // (idx_in_disc, num_of_vals, nums)
        let mut map_num_with_indices: Vec<(usize, (u8, u32))> =
            Vec::with_capacity(self.maps.len() / 2);
        let mut map_spaces_with_indices: Vec<(usize, u8)> = Vec::with_capacity(self.maps.len() / 2);
        let _ =
            self.maps
                .iter()
                .enumerate()
                .fold((0, 0), |(acc_idx, gen_num), (map_idx, &count)| {
                    if map_idx % 2 == 1 {
                        map_spaces_with_indices.push((acc_idx, count));
                    } else {
                        map_num_with_indices.push((acc_idx, (count, gen_num)));
                    }
                    (
                        acc_idx + count as usize,
                        if map_idx % 2 == 1 {
                            gen_num + 1 // Increment the number if it's on a space to prepare for the next number
                        } else {
                            gen_num // Keep the number the same if it's on a number
                        },
                    )
                });

        let mut map_num_ptr = map_num_with_indices.iter().rev();

        let mut check_sum = 0;

        while let Some(&(r_idx, (r_count, num))) = map_num_ptr.next() {
            if let Some((l_map_idx, &(disc_idx, _))) = map_spaces_with_indices
                .iter()
                .enumerate()
                .find(|&(_, &(l_idx, l_count))| l_count >= r_count && l_idx < r_idx)
            // Find a space that can hold the values
            {
                // Add the moved value
                for i in (disc_idx..).take(r_count as usize) {
                    check_sum += num as u64 * i as u64;
                }

                let (l_idx, l_count) = &mut map_spaces_with_indices[l_map_idx];
                *l_idx += r_count as usize;
                *l_count -= r_count;
                // remove it from consideration if it's empty for the next round
                if l_count == &0 {
                    map_spaces_with_indices.swap_remove(l_map_idx);
                }
            } else {
                // If there are no spaces that can hold the values, then compute them
                for i in (r_idx..).take(r_count as usize) {
                    check_sum += num as u64 * i as u64;
                }
            }
        }

        check_sum
    }

    fn fragmented_check_sum(mut self) -> usize {
        let mut check_sum = 0;

        let mut l_ptr = 0;
        let mut r_ptr = self.maps.len() - 1;

        let mut l_num = 0;
        let mut r_num = self.maps.len() / 2;

        let mut virtual_list_idx = 0;

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
                        check_sum += r_num * virtual_list_idx;
                        self.maps[r_ptr] -= 1;
                        self.maps[l_ptr] -= 1;
                        virtual_list_idx += 1;
                    } else {
                        // Add nums from the left to checksum
                        check_sum += l_num * virtual_list_idx;
                        self.maps[l_ptr] -= 1;
                        virtual_list_idx += 1;
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
