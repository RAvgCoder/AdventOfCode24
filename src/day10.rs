use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::{Grid, GridMut};
use std::collections::{HashSet, VecDeque};

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/10).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 10, Some(459));
    Utils::run_part(part2, 2, 10, Some(1034));
}

fn part1(topographic_map: TopographicMap) -> u16 {
    topographic_map.count_trail_heads()
}

fn part2(topographic_map: TopographicMap) -> u16 {
    topographic_map.count_rating()
}

#[derive(Debug)]
struct TopographicMap {
    map: UnsizedGrid<u8>,
}

impl TopographicMap {
    fn count_rating(&self) -> u16 {
        fn dfs_rating(
            curr: Coordinate,
            map: &UnsizedGrid<u8>,
            visited: &mut [bool; 9],
            queue: &mut VecDeque<Coordinate>,
        ) -> u16 {
            if !map.is_valid_coordinate(&curr) {
                return 0;
            }

            let curr_num = *map.get(&curr).unwrap();
            if curr_num == 9 {
                return 1;
            }

            if visited[curr_num as usize] {
                return 0;
            }

            visited[curr_num as usize] = true;

            let mut rating = 0;
            Direction::direction_list()
                .map(|dir| curr + dir)
                .into_iter()
                .filter(|next_coord| {
                    if let Some(&next_num) = map.get(next_coord) {
                        next_num == curr_num + 1
                    } else {
                        false
                    }
                })
                .for_each(|next| {
                    rating += dfs_rating(next, map, visited, queue);
                });

            visited[curr_num as usize] = false;
            rating
        }

        self.map
            .iter()
            .flatten()
            .filter(|(_, &e)| e == 0)
            .map(|(coord, _)| dfs_rating(coord, &self.map, &mut [false; 9], &mut VecDeque::new()))
            .sum()
    }

    fn count_trail_heads(&self) -> u16 {
        let mut trail_heads = 0;
        let map = &self.map;
        let mut visited: HashSet<Coordinate> =
            HashSet::with_capacity(self.map.num_cols() * self.map.num_rows());
        let mut queue = VecDeque::new();
        for row in self.map.iter() {
            for (trail_start, &e) in row {
                if e == 0 {
                    queue.push_back(trail_start);
                    while let Some(cord) = queue.pop_front() {
                        if !visited.insert(cord) {
                            continue;
                        }

                        let curr_num = *map.get(&cord).unwrap();
                        if curr_num == 9 {
                            trail_heads += 1;
                            continue;
                        }

                        Direction::direction_list()
                            .map(|dir| cord + dir)
                            .into_iter()
                            .filter(|c| map.is_valid_coordinate(c))
                            .filter(|cord| curr_num + 1 == *map.get(cord).unwrap())
                            .for_each(|next| queue.push_back(next))
                    }
                    queue.clear();
                    visited.clear();
                }
            }
        }

        trail_heads
    }
}

impl From<Vec<String>> for TopographicMap {
    fn from(value: Vec<String>) -> Self {
        let (row, col) = (value.len(), value[0].len());
        let mut map = UnsizedGrid::new(row, col, 0);
        for (i, row) in value.iter().enumerate() {
            for (j, e) in row.chars().enumerate() {
                *map.get_mut(&Coordinate::new(i as i32, j as i32)).unwrap() =
                    e.to_digit(10).unwrap() as _
            }
        }
        Self { map }
    }
}
