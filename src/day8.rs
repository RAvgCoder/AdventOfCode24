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
    Utils::run_part(part2, 2, 8, Some(1077));
}

fn part1(frequency_map: AntennaMap) -> usize {
    frequency_map
        .create_anti_node(AntiNodeDistance::Twice)
        .len()
}

fn part2(frequency_map: AntennaMap) -> usize {
    frequency_map
        .create_anti_node(AntiNodeDistance::Unbounded)
        .len()
}

#[derive(Debug)]
struct AntennaMap {
    antenna_map: HashMap<char, Vec<Coordinate>>,
    grid_size: (u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AntiNodeDistance {
    Twice,
    Unbounded,
}

impl AntennaMap {
    fn create_anti_node(&self, anti_node_distance: AntiNodeDistance) -> HashSet<Coordinate> {
        let mut anti_nodes = HashSet::new();

        for (_, coords) in self.antenna_map.iter() {
            for (i, c1) in coords.iter().enumerate() {
                for c2 in coords[i + 1..].iter() {
                    self.find_anti_node(*c1, *c2, &mut anti_nodes, anti_node_distance);
                }
            }
        }

        anti_nodes
    }

    fn find_anti_node(
        &self,
        c1: Coordinate,
        c2: Coordinate,
        anti_node: &mut HashSet<Coordinate>,
        anti_node_distance: AntiNodeDistance,
    ) {
        let (dx, dy) = c1.slope_relative(c2);
        if dx == 0 {
            panic!("dx is 0 for c1: {:#?}, c2: {:#?}", c1, c2);
        }

        let dx_dy_coord = Coordinate::new(dx, dy);
        let c1_temp = c1 - dx_dy_coord;
        let c2_temp = c2 + dx_dy_coord;

        match anti_node_distance {
            AntiNodeDistance::Twice => {
                if self.in_bounds(c1_temp) {
                    anti_node.insert(c1_temp);
                }
                if self.in_bounds(c2_temp) {
                    anti_node.insert(c2_temp);
                }
            }
            AntiNodeDistance::Unbounded => {
                // Add the current coordinates to the anti_node set as they form part of the anti-node
                anti_node.insert(c1);
                anti_node.insert(c2);

                // Add all the coordinates in the direction of the slope to the anti_node set
                let mut c1_temp = c1_temp;
                let mut c2_temp = c2_temp;
                while self.in_bounds(c1_temp) {
                    anti_node.insert(c1_temp);
                    c1_temp -= dx_dy_coord;
                }
                while self.in_bounds(c2_temp) {
                    anti_node.insert(c2_temp);
                    c2_temp += dx_dy_coord;
                }
            }
        }
    }

    fn in_bounds(&self, coord: Coordinate) -> bool {
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
                        antenna_map
                            .entry(other)
                            .or_insert_with(Vec::new)
                            .push(coord);
                    }
                }
            }
        }

        Self {
            antenna_map,
            grid_size: (input.len() as u32, input[0].len() as u32),
        }
    }
}
