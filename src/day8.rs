use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use std::collections::{HashMap, HashSet};

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/8).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 8, Some(323));
    Utils::run_part(part2, 2, 0, None);
}

fn part1(frequency_map: AntennaMap) -> u64 {
    frequency_map.create_anti_node().len() as u64
}

fn part2(_input: Vec<String>) -> u64 {
    // println!("Part 2 {:#?}", input);
    0
}

#[derive(Debug)]
struct AntennaMap {
    antenna_map: HashMap<char, Vec<Coordinate>>,
    grid_size: (u32, u32),
}

impl AntennaMap {
    fn create_anti_node(&self) -> HashSet<Coordinate> {
        let mut anti_nodes = HashSet::new();

        for (_, coords) in self.antenna_map.iter() {
            for (i, c1) in coords.iter().enumerate() {
                for c2 in coords[i + 1..].iter() {
                    self.find_anti_node(*c1, *c2, &mut anti_nodes);
                }
            }
        }

        anti_nodes
    }

    fn find_anti_node(&self, c1: Coordinate, c2: Coordinate, anti_node: &mut HashSet<Coordinate>) {
        let (dx, dy) = c1.slope_relative(c2);
        if dx == 0 {
            panic!("dx is 0 for c1: {:#?}, c2: {:#?}", c1, c2);
        }

        let dx_dy_coord = Coordinate::new(dx, dy);
        let c1_temp = c1 - dx_dy_coord;
        let c2_temp = c2 + dx_dy_coord;

        if self.is_valid(c1_temp) {
            anti_node.insert(c1_temp);
        }
        if self.is_valid(c2_temp) {
            anti_node.insert(c2_temp);
        }
    }

    fn is_valid(&self, coord: Coordinate) -> bool {
        let (i, j) = coord.into();
        (0..self.grid_size.0 as i32).contains(&i) && (0..self.grid_size.1 as i32).contains(&j)
    }
}

impl From<Vec<String>> for AntennaMap {
    fn from(input: Vec<String>) -> Self {
        let mut antenna_map = HashMap::new();
        for (i, row) in input.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                match c {
                    '.' | '#' => {}
                    other => {
                        let coord = Coordinate::new(i as i32, j as i32);
                        antenna_map.entry(other).or_insert_with(Vec::new).push(coord);
                    }
                }
            }
        }

        Self { antenna_map, grid_size: (input.len() as u32, input[0].len() as u32) }
    }
}