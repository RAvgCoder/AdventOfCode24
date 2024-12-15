use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::{Grid, GridMut};
use std::fmt::Debug;
use std::mem;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/15).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 15, Some(1538871));
    Utils::run_part(part2, 2, 0, None);
}

fn part1(mut warehouse_robot: WarehouseRobot) -> u32 {
    warehouse_robot.start_simulation();
    warehouse_robot.sum_of_gps_coordinates()
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

type Dir = (u8, Direction);
#[derive(Debug)]
struct WarehouseRobot {
    robot_pos: Coordinate,
    map: UnsizedGrid<Object>,
    moves: Vec<Dir>,
}

impl WarehouseRobot {
    fn start_simulation(&mut self) {
        // Appease the borrow checker gods by moving the moves 🕺 out of the struct
        let moves = mem::take(&mut self.moves);
        for (times, direction) in moves {
            match direction {
                Direction::North | Direction::South => self.move_box((times, direction)),
                Direction::East | Direction::West => self.move_box((times, direction)),
                Direction::Current => unreachable!("Invalid direction"),
            }
        }
    }

    fn sum_of_gps_coordinates(&self) -> u32 {
        const MULTIPLIER: u32 = 100;
        self.map
            .iter()
            .map(|row| {
                row.map(|(coord, obj)| {
                    if Object::Box == *obj {
                        MULTIPLIER * coord.i as u32 + coord.j as u32
                    } else {
                        0
                    }
                })
                .sum::<u32>()
            })
            .sum()
    }

    fn move_box(&mut self, (times, dir): Dir) {
        let mut _times = times;
        let mut space_searcher = self.robot_pos + dir;
        while _times != 0 {
            match self.map.get(&space_searcher).unwrap() {
                Object::Wall => {
                    // Can move no further
                    break;
                }
                Object::Empty => {
                    // Move the robot and the box specially for now
                    *self.map.get_mut(&space_searcher).unwrap() = Object::Box;
                    _times -= 1;
                }
                Object::Box => { /* Pass over */ }
                Object::Robot => unreachable!("Robot cannot be in the path iterating over"),
            }
            space_searcher += dir;
        }

        let boxes_to_move = times - _times;
        for _ in 0..boxes_to_move {
            // Make the prev robot pos empty
            *self.map.get_mut(&self.robot_pos).unwrap() = Object::Empty;
            self.robot_pos += dir;
        }
        // Place the robot at the new position
        *self.map.get_mut(&self.robot_pos).unwrap() = Object::Robot;
    }

    fn print_grid(&self) {
        for row in self.map.iter() {
            for (_, e) in row {
                print!("{:?}", e);
            }
            println!();
        }
        println!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Object {
    Wall,
    Empty,
    Robot,
    Box,
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Wall => write!(f, "#"),
            Object::Empty => write!(f, "."),
            Object::Robot => write!(f, "@"),
            Object::Box => write!(f, "O"),
        }
    }
}

impl From<Vec<String>> for WarehouseRobot {
    fn from(input: Vec<String>) -> Self {
        fn get_dir(e: char) -> (u8, Direction) {
            (
                1,
                match e {
                    '^' => Direction::North,
                    'v' => Direction::South,
                    '>' => Direction::East,
                    '<' => Direction::West,
                    _ => unreachable!(),
                },
            )
        }

        let mut map: Vec<&str> = vec![];
        let mut moves = vec![];
        let mut input = input.iter();
        loop {
            let line = input.next().unwrap();
            if line.is_empty() {
                break;
            }
            map.push(line);
        }
        for line in input {
            line.chars().for_each(|e| {
                let (c, d) = get_dir(e);
                match moves.last_mut() {
                    Some((count, dir)) => {
                        // Group similar directions together
                        if *dir == d {
                            *count += 1;
                        } else {
                            moves.push((c, d))
                        }
                    }
                    None => moves.push((c, d)),
                }
            });
        }
        let mut grid = UnsizedGrid::new(map.len(), map[0].len(), Object::Empty);
        let mut robot_pos = None;
        for (i, e) in map.into_iter().enumerate() {
            for (j, c) in e.chars().enumerate() {
                let coord = Coordinate::new(i as i32, j as i32);
                match c {
                    '@' => {
                        if robot_pos.is_some() {
                            panic!("Multiple robots found in the map");
                        }
                        robot_pos = Some(coord);
                        *grid.get_mut(&coord).unwrap() = Object::Robot;
                    }
                    c => {
                        *grid.get_mut(&coord).unwrap() = match c {
                            '#' => Object::Wall,
                            '.' => Object::Empty,
                            'O' => Object::Box,
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        Self {
            map: grid,
            robot_pos: robot_pos.expect("No robot found in the map"),
            moves,
        }
    }
}
