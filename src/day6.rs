use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::{Grid, GridMut};
use std::fmt::{Debug, Formatter};

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/202024/day/6).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part_single(part1, 1, 6, None);
    Utils::run_part_single(part2, 2, 0, None);
}

fn part1(mut office_plan: OfficePlan) -> usize {
    office_plan.simulate();
    office_plan.count_visited()
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

struct OfficePlan {
    map: UnsizedGrid<char>,
    guard_position: Coordinate,
    guard_moving_direction: Direction,
}

impl OfficePlan {
    fn simulate(&mut self) {
        let mut guard = self.guard_position;
        let mut direction = self.guard_moving_direction;
        loop {
            let peek_next = guard + direction;

            if let Some(cell) = self.map.get_mut(&peek_next) {
                match cell {
                    '#' => {
                        // The guard has hit a wall
                        direction = match direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                            Direction::Current => {
                                unreachable!("Guard should never stand still")
                            }
                        };
                    }
                    _ => {
                        // NoObstacles or walking on a path traversed before
                        // Mark the cell the guard is standing on as visited
                        // to prepare to move to the next cell
                        *self.map.get_mut(&guard).unwrap() = 'X';
                        guard = peek_next;
                    }
                }
            } else {
                // Mark the final cell the guard visited before leaving the map
                *self.map.get_mut(&guard).unwrap() = 'X';
                // The guard has left the perimeter
                break;
            }
        }
    }
    fn count_visited(&self) -> usize {
        self.map
            .iter()
            .map(|row_iter| row_iter.filter(|(_, &e)| e == 'X').count())
            .sum()
    }
}

impl Debug for OfficePlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Guard: {:?}", self.guard_position)?;
        writeln!(f, "Guard Direction: {:?}", self.guard_moving_direction)?;
        for row in self.map.iter() {
            for (_, cell) in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Vec<String>> for OfficePlan {
    fn from(value: Vec<String>) -> Self {
        let mut guard = None;
        Self {
            map: UnsizedGrid::new(
                value
                    .into_iter()
                    .enumerate()
                    .map(|(i, s)| {
                        let row: Vec<char> = s.chars().collect();
                        row.iter().position(|&c| c == '^').map(|j| {
                            guard = Some(Coordinate::new(i as i32, j as i32));
                        });
                        row
                    })
                    .collect::<Vec<_>>(),
            ),
            guard_position: guard.expect("No guard found"),
            guard_moving_direction: Direction::North,
        }
    }
}
