use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::{Grid, GridMut};
use std::collections::VecDeque;
use std::fmt::Debug;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/16).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 16, None);
    Utils::run_part(part2, 2, 0, None);
}

fn part1(mut reindeer_maze: ReindeerMaze) -> u32 {
    reindeer_maze.find_lowest_cost()
}

fn part2(_input: Vec<String>) -> u64 {
    // println!("Part 2 {:#?}", input);
    0
}

impl ReindeerMaze {
    fn find_lowest_cost(&mut self) -> u32 {
        const INFINITY: u32 = u32::MAX;
        const BASE_MULTIPLIER: u32 = 1;
        const NINETY_DEGREE_TURN_MULTIPLIER: u32 = 1000 + BASE_MULTIPLIER;

        let mut min_score_grid = UnsizedGrid::with_size_from(&self.maze, INFINITY);

        let mut queue = VecDeque::with_capacity(self.maze.num_rows() * self.maze.num_cols());

        Direction::direction_list()
            .map(|dir| (self.start + dir, dir))
            .into_iter()
            .filter(|(coord, _)| *self.maze.get(coord).unwrap() != Objects::Wall)
            .for_each(|(coord, dir)| {
                if dir == Direction::East {
                    queue.push_back((coord, dir, BASE_MULTIPLIER));
                } else {
                    queue.push_back((coord, dir, NINETY_DEGREE_TURN_MULTIPLIER))
                }
            });

        while let Some((curr_coord, curr_dir, curr_score)) = queue.pop_front() {
            {
                let obj = *self.maze.get(&curr_coord).unwrap();
                match obj {
                    Objects::Wall => continue,
                    Objects::Start => continue,
                    Objects::End => {
                        let curr_min = min_score_grid.get_mut(&curr_coord).unwrap();
                        *curr_min = curr_score.min(*curr_min);
                        continue;
                    }
                    Objects::Path | Objects::Sim(_) => {
                        let curr_min = min_score_grid.get_mut(&curr_coord).unwrap();
                        if *curr_min <= curr_score {
                            continue;
                        }
                        *curr_min = curr_score.min(*curr_min);

                        *self.maze.get_mut(&curr_coord).unwrap() = Self::get_sim(curr_dir);
                    }
                }
            }

            for next_dir in Direction::direction_list() {
                let next_coord = curr_coord + next_dir;

                let new_score = curr_score
                    + if next_dir == curr_dir.opposite() || next_dir == curr_dir {
                        BASE_MULTIPLIER
                    } else {
                        NINETY_DEGREE_TURN_MULTIPLIER
                    };

                queue.push_back((next_coord, next_dir, new_score));
            }
        }

        *min_score_grid.get(&self.end).unwrap()
    }

    fn write_to_file<T>(grid: &UnsizedGrid<T>, score: u32)
    where
        T: Debug,
    {
        println!("Score: {}", score);
        aoc_utils_rust::miscellaneous::dump_grid_to_file(
            grid,
            "grid_output.txt",
            false,
            None::<fn(&T) -> char>,
        )
        .expect("Failed to dump grid to file");
    }

    fn get_sim(dir: Direction) -> Objects {
        Objects::Sim(dir)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Objects {
    Wall,
    Path,
    Start,
    End,
    Sim(Direction),
}

impl Debug for Objects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Objects::Wall => write!(f, "#"),
            Objects::Path => write!(f, "."),
            Objects::Start => write!(f, "S"),
            Objects::End => write!(f, "E"),
            Objects::Sim(dir) => {
                let c = match *dir {
                    Direction::North => '^',
                    Direction::South => 'v',
                    Direction::East => '>',
                    Direction::West => '<',
                    Direction::Current => panic!("Invalid direction"),
                };
                write!(f, "{}", c)
            }
        }
    }
}

#[derive(Debug)]
struct ReindeerMaze {
    start: Coordinate,
    end: Coordinate,
    maze: UnsizedGrid<Objects>,
}

impl From<Vec<String>> for ReindeerMaze {
    fn from(value: Vec<String>) -> Self {
        let mut maze = UnsizedGrid::new(value.len(), value[0].len(), Objects::Path);
        let mut start = None;
        let mut end = None;
        for (i, line) in value.iter().enumerate() {
            for (j, e) in line.chars().enumerate() {
                let obj = match e {
                    '#' => Objects::Wall,
                    '.' => Objects::Path,
                    'S' => {
                        start = Some(Coordinate::new(i as i32, j as i32));
                        Objects::Start
                    }
                    'E' => {
                        end = Some(Coordinate::new(i as i32, j as i32));
                        Objects::End
                    }
                    _ => panic!("Invalid character in maze"),
                };
                *maze.get_mut(&Coordinate::new(i as i32, j as i32)).unwrap() = obj;
            }
        }
        Self {
            maze,
            end: end.unwrap(),
            start: start.unwrap(),
        }
    }
}
