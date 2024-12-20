use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::sized_grid::SizedGrid;
use aoc_utils_rust::grid::{Grid, GridMut};
use std::collections::VecDeque;
use std::fmt::Debug;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/18).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 18, Some(262));
    Utils::run_part(part2, 2, 0, None);
}
const GRID_SIZE: usize = 71;
fn part1(mut ram_map: RamMap) -> u32 {
    ram_map.corrupt_n_bytes(1024);
    ram_map.find_shortest_path()
}

fn part2(input: Vec<String>) -> u64 {
    // println!("Part 2 {:#?}", input);
    0
}

#[derive(Debug)]
struct RamMap {
    map: SizedGrid<Spots, GRID_SIZE, GRID_SIZE>,
    byte_rain_coord: Box<[Coordinate]>,
}

impl RamMap {
    fn corrupt_n_bytes(&mut self, n: usize) {
        for coord in self.byte_rain_coord.iter().take(n) {
            *self.map.get_mut(coord).unwrap() = Spots::CorruptedByte;
        }
    }

    fn find_shortest_path(&mut self) -> u32 {
        let end_coord = self.map.bottom_right_coordinate();
        let mut queue = VecDeque::with_capacity(self.map.num_cols());
        queue.push_back((Coordinate::ORIGIN, 0));
        while let Some((next_coord, steps)) = queue.pop_front() {
            if next_coord == end_coord {
                return steps;
            }
            match *self.map.get(&next_coord).unwrap() {
                Spots::CorruptedByte | Spots::Visited => continue,
                Spots::FreeSpace => {
                    *self.map.get_mut(&next_coord).unwrap() = Spots::Visited;
                    Direction::direction_list()
                        .map(|dir| next_coord + dir)
                        .iter()
                        .filter(|coord| self.map.get(&coord).is_some())
                        .for_each(|&next_coord| {
                            queue.push_back((next_coord, steps + 1));
                        });
                }
            }
        }
        unreachable!("No path found")
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Spots {
    CorruptedByte,
    FreeSpace,
    Visited,
}

impl Debug for Spots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spots::CorruptedByte => write!(f, "#"),
            Spots::FreeSpace => write!(f, "."),
            Spots::Visited => write!(f, "_"),
        }
    }
}

impl From<Vec<String>> for RamMap {
    #[inline]
    fn from(input: Vec<String>) -> Self {
        use std::str::FromStr;
        Self {
            map: SizedGrid::new(Spots::FreeSpace),
            byte_rain_coord: input
                .iter()
                .map(|str| Coordinate::from_str(str).unwrap())
                .collect::<Box<_>>(),
        }
    }
}
