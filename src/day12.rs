use aoc_utils_rust::coordinate_system::direction::{Direction, FullDirection};
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::Grid;
use std::collections::HashSet;
use std::fmt::Debug;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/12).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 12, Some(1387004));
    Utils::run_part(part2, 2, 12, Some(844198));
}

fn part1(garden: Garden) -> u32 {
    garden.fencing_price(false)
}

fn part2(garden: Garden) -> u32 {
    garden.fencing_price(true)
}

#[derive(Debug)]
struct Garden {
    garden: UnsizedGrid<char>,
}
impl Garden {
    fn fencing_price(&self, with_size: bool) -> u32 {
        let mut price = 0;
        let mut plot_visited = HashSet::new();
        let mut global_visited = HashSet::new();
        for i in 0..self.garden.num_rows() {
            for j in 0..self.garden.num_cols() {
                let coord = Coordinate::new(i as i32, j as i32);
                let e = *self.garden.get(&coord).unwrap();
                if !global_visited.contains(&coord) {
                    let perimeter = if with_size {
                        self.calculate_price_with_sides(coord, e, &mut plot_visited)
                    } else {
                        self.calculate_price(coord, e, &mut plot_visited)
                    };
                    price += plot_visited.len() as u32 * perimeter;
                    global_visited.extend(plot_visited.drain());
                }
            }
        }
        price
    }

    fn calculate_price(
        &self,
        curr: Coordinate,
        plot_searching_for: char,
        visited: &mut HashSet<Coordinate>,
    ) -> u32 {
        match self.garden.get(&curr) {
            None => 1,                                                // Out of bounds
            Some(curr_plot) if *curr_plot != plot_searching_for => 1, // Different plot
            Some(_) => {
                if !visited.insert(curr) {
                    return 0; // Already visited
                }

                let mut perimeter = 0;
                for dir in Direction::direction_list() {
                    let next = curr + dir;
                    let next_perimeter = self.calculate_price(next, plot_searching_for, visited);
                    perimeter += next_perimeter;
                }

                perimeter
            }
        }
    }

    fn calculate_price_with_sides(
        &self,
        curr: Coordinate,
        plot_searching_for: char,
        visited: &mut HashSet<Coordinate>,
    ) -> u32 {
        match self.garden.get(&curr) {
            None => 0,                                                // Out of bounds
            Some(curr_plot) if *curr_plot != plot_searching_for => 0, // Different plot
            Some(_) => {
                if !visited.insert(curr) {
                    return 0; // Already visited
                }

                let mut perimeter_queue = [0u32; 4];
                for (idx, dir) in Direction::direction_list().into_iter().enumerate() {
                    let next = curr + dir;
                    perimeter_queue[idx] =
                        self.calculate_price_with_sides(next, plot_searching_for, visited);
                }

                perimeter_queue.iter().sum::<u32>()
                    + self.calculate_curr_perimeter(curr, plot_searching_for)
            }
        }
    }

    fn calculate_curr_perimeter(&self, curr: Coordinate, plot_searching_for: char) -> u32 {
        let mut perimeter = 0;

        const SEARCH_DIR: [[FullDirection; 3]; 4] = [
            [
                FullDirection::North,
                FullDirection::West,
                FullDirection::NorthWest,
            ],
            [
                FullDirection::North,
                FullDirection::East,
                FullDirection::NorthEast,
            ],
            [
                FullDirection::South,
                FullDirection::West,
                FullDirection::SouthWest,
            ],
            [
                FullDirection::South,
                FullDirection::East,
                FullDirection::SouthEast,
            ],
        ];

        for [a, b, c] in SEARCH_DIR {
            perimeter += match (
                self.is_on_curr_plot(curr + a, plot_searching_for),
                self.is_on_curr_plot(curr + b, plot_searching_for),
                self.is_on_curr_plot(curr + c, plot_searching_for),
            ) {
                (true, true, false) => 1,
                (false, false, _) => 1,
                _ => 0,
            }
        }

        perimeter
    }

    fn is_on_curr_plot(&self, coord: Coordinate, curr_plot: char) -> bool {
        match self.garden.get(&coord) {
            Some(&plot) if plot == curr_plot => true,
            _ => false,
        }
    }
}

impl From<Vec<String>> for Garden {
    fn from(value: Vec<String>) -> Self {
        Self {
            garden: UnsizedGrid::from(
                value
                    .into_iter()
                    .map(|s| s.chars().collect())
                    .collect::<Box<_>>(),
            ),
        }
    }
}
