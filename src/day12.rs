use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::Grid;
use std::collections::HashSet;

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
    Utils::run_part(part2, 2, 0, None);
}

fn part1(mut garden: Garden) -> u32 {
    garden.fencing_price()
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

#[derive(Debug)]
struct Garden {
    garden: UnsizedGrid<char>,
}

type PlotResult = (u32, u32); // (Area, Perimeter)
impl Garden {
    fn fencing_price(&mut self) -> u32 {
        let mut price = 0;
        let mut plot_visited = HashSet::new();
        let mut global_visited = HashSet::new();
        for i in 0..self.garden.num_rows() {
            for j in 0..self.garden.num_cols() {
                let coord = Coordinate::new(i as i32, j as i32);
                let e = *self.garden.get(&coord).unwrap();
                if !global_visited.contains(&coord) {
                    let (area, perimeter) = self.calculate_price(coord, e, &mut plot_visited);
                    global_visited.extend(plot_visited.drain());
                    price += area * perimeter;
                }
            }
        }
        price
    }

    fn calculate_price(
        &mut self,
        curr: Coordinate,
        plot_searching_for: char,
        visited: &mut HashSet<Coordinate>,
    ) -> PlotResult {
        match self.garden.get(&curr) {
            None => (0, 1),                                                // Out of bounds
            Some(curr_plot) if *curr_plot != plot_searching_for => (0, 1), // Different plot
            Some(_) => {
                if !visited.insert(curr) {
                    return (0, 0); // Already visited
                }

                let mut area = 1; // Count my self
                let mut perimeter = 0; // I'm not a perimeter
                for dir in Direction::direction_list() {
                    let next = curr + dir;
                    let (next_area, next_perimeter) =
                        self.calculate_price(next, plot_searching_for, visited);
                    area += next_area;
                    perimeter += next_perimeter;
                }

                (area, perimeter)
            }
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
