use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::{Grid, GridMut};
use std::collections::HashSet;
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
    Utils::run_part(part2, 2, 15, Some(1543338));
}

fn part1(mut warehouse_robot: WarehouseRobot<ObjectNormal>) -> u32 {
    warehouse_robot.start_simulation();
    warehouse_robot.sum_of_gps_coordinates()
}

fn part2(mut warehouse_robot: WarehouseRobot<ObjectMalfunctioning>) -> u32 {
    warehouse_robot.start_simulation();
    warehouse_robot.sum_of_gps_coordinates()
}

type Dir = (u8, Direction);
#[derive(Debug)]
struct WarehouseRobot<T> {
    robot_pos: Coordinate,
    map: UnsizedGrid<T>,
    moves: Vec<Dir>,
}

type BoxPair = (Coordinate, Coordinate);
impl WarehouseRobot<ObjectMalfunctioning> {
    fn start_simulation(&mut self) {
        // Appease the borrow checker gods by moving the moves 🕺 out of the struct
        let moves = mem::take(&mut self.moves);
        for (times, direction) in moves {
            match direction {
                Direction::North | Direction::South => self.move_vertically((times, direction)),
                Direction::East | Direction::West => self.move_horizontally((times, direction)),
                Direction::Current => unreachable!("Invalid direction"),
            }
        }
    }

    fn sum_of_gps_coordinates(&self) -> u32 {
        const MULTIPLIER: u32 = 100;
        self.map
            .iter()
            .map(|row| {
                row.map(|(coord, obj)| match *obj {
                    ObjectMalfunctioning::BoxLeft => MULTIPLIER * coord.i as u32 + coord.j as u32,
                    ObjectMalfunctioning::Wall
                    | ObjectMalfunctioning::Empty
                    | ObjectMalfunctioning::Robot
                    | ObjectMalfunctioning::BoxRight => 0,
                })
                .sum::<u32>()
            })
            .sum()
    }

    fn move_vertically(&mut self, (times, dir): Dir) {
        for _ in 0..times {
            let next = self.robot_pos + dir;
            match self.map.get(&next).unwrap() {
                ObjectMalfunctioning::BoxLeft => {
                    if !self.move_box_vertically(dir, (next, next + Direction::East)) {
                        break;
                    }
                }
                ObjectMalfunctioning::BoxRight => {
                    if !self.move_box_vertically(dir, (next + Direction::West, next)) {
                        break;
                    }
                }
                ObjectMalfunctioning::Wall => break,
                ObjectMalfunctioning::Empty => (),
                ObjectMalfunctioning::Robot => {
                    unreachable!("Robot cannot be in the path iterating over")
                }
            }
            *self.map.get_mut(&self.robot_pos).unwrap() = ObjectMalfunctioning::Empty;
            self.robot_pos = next;
            *self.map.get_mut(&self.robot_pos).unwrap() = ObjectMalfunctioning::Robot;
        }
    }

    fn move_box_vertically(&mut self, dir: Direction, (left_box, right_box): BoxPair) -> bool {
        let left = *self.map.get(&left_box).unwrap();
        let right = *self.map.get(&right_box).unwrap();
        match (left, right) {
            (ObjectMalfunctioning::BoxLeft, ObjectMalfunctioning::BoxRight) => (),
            _ => panic!("Invalid box configuration"),
        }

        let mut visited = HashSet::new();
        if self.can_move_vertically(dir, left_box, &mut visited) {
            visited.clear();
            self.recursively_move_vertically(
                dir,
                right_box,
                ObjectMalfunctioning::BoxRight,
                &mut visited,
            );
            true
        } else {
            false
        }
    }

    fn can_move_vertically(
        &self,
        dir: Direction,
        box_part: Coordinate,
        visited: &mut HashSet<Coordinate>,
    ) -> bool {
        if !visited.insert(box_part) {
            return true;
        }
        match *self.map.get(&box_part).unwrap() {
            // Can move no further
            ObjectMalfunctioning::Wall => false,
            ObjectMalfunctioning::Empty => true,
            ObjectMalfunctioning::Robot => panic!("Robot cannot be in the path iterating over"),
            ObjectMalfunctioning::BoxLeft => {
                // Check Rights bottom
                self.can_move_vertically(dir, box_part + dir, visited)
                    // Check Left side
                    && self.can_move_vertically(dir, box_part + Direction::East, visited)
            }
            ObjectMalfunctioning::BoxRight => {
                // Check Right side
                self.can_move_vertically(dir, box_part + Direction::West, visited)
                    // Check Lefts bottom
                    && self.can_move_vertically(dir, box_part + dir, visited)
            }
        }
    }

    fn recursively_move_vertically(
        &mut self,
        dir: Direction,
        box_part: Coordinate,
        part: ObjectMalfunctioning,
        visited: &mut HashSet<Coordinate>,
    ) {
        if !visited.insert(box_part) {
            return;
        }
        match *self.map.get(&box_part).unwrap() {
            ObjectMalfunctioning::Wall => {
                panic!("I was given the go to move a box but there's a wall")
            }
            ObjectMalfunctioning::Empty => {
                *self.map.get_mut(&box_part).unwrap() = part;
            }
            ObjectMalfunctioning::Robot => panic!("Robot cannot be in the path iterating over"),
            ObjectMalfunctioning::BoxLeft => {
                // Check Lefts bottom
                self.recursively_move_vertically(
                    dir,
                    box_part + dir,
                    ObjectMalfunctioning::BoxLeft,
                    visited,
                );
                *self.map.get_mut(&box_part).unwrap() = ObjectMalfunctioning::Empty;
                *self.map.get_mut(&(box_part + dir)).unwrap() = ObjectMalfunctioning::BoxLeft;

                self.recursively_move_vertically(
                    dir,
                    box_part + Direction::East,
                    ObjectMalfunctioning::BoxRight,
                    visited,
                ); // Check Rights bottom
                *self.map.get_mut(&(box_part + Direction::East)).unwrap() =
                    ObjectMalfunctioning::Empty;
            }
            ObjectMalfunctioning::BoxRight => {
                self.recursively_move_vertically(
                    dir,
                    box_part + Direction::West,
                    ObjectMalfunctioning::BoxLeft,
                    visited,
                ); // Check Rights bottom
                *self.map.get_mut(&(box_part + Direction::West)).unwrap() =
                    ObjectMalfunctioning::Empty;

                self.recursively_move_vertically(
                    dir,
                    box_part + dir,
                    ObjectMalfunctioning::BoxRight,
                    visited,
                ); // Check Lefts bottom
                *self.map.get_mut(&box_part).unwrap() = ObjectMalfunctioning::Empty;
                *self.map.get_mut(&(box_part + dir)).unwrap() = ObjectMalfunctioning::BoxRight;
            }
        }
    }

    fn move_horizontally(&mut self, (times, dir): Dir) {
        let mut _times = times;
        let mut space_searcher = self.robot_pos + dir;
        while _times != 0 {
            match self.map.get(&space_searcher).unwrap() {
                ObjectMalfunctioning::Wall => {
                    // Can move no further
                    break;
                }
                ObjectMalfunctioning::Empty => {
                    // Move the robot and the box specially for now
                    let bounds = match dir {
                        Direction::East => self.robot_pos.j as usize..=space_searcher.j as usize,
                        Direction::West => space_searcher.j as usize..=self.robot_pos.j as usize,
                        _ => unreachable!(),
                    };
                    // Move the box to the empty space
                    let row = self.map.get_row_mut(space_searcher.i as usize).unwrap();
                    match dir {
                        Direction::East => {
                            row[bounds].rotate_right(1);
                        }
                        Direction::West => {
                            row[bounds].rotate_left(1);
                        }
                        _ => unreachable!(),
                    }
                    self.robot_pos += dir;
                    _times -= 1;
                }
                ObjectMalfunctioning::BoxLeft | ObjectMalfunctioning::BoxRight => {
                    /* Pass over */
                }
                ObjectMalfunctioning::Robot => {
                    unreachable!("Robot cannot be in the path iterating over")
                }
            }
            space_searcher += dir;
        }
    }
}

impl WarehouseRobot<ObjectNormal> {
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
                row.map(|(coord, obj)| match *obj {
                    ObjectNormal::Box => MULTIPLIER * coord.i as u32 + coord.j as u32,
                    ObjectNormal::Wall | ObjectNormal::Empty | ObjectNormal::Robot => 0,
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
                ObjectNormal::Wall => {
                    // Can move no further
                    break;
                }
                ObjectNormal::Empty => {
                    // Move the robot and the box specially for now
                    *self.map.get_mut(&space_searcher).unwrap() = ObjectNormal::Box;
                    _times -= 1;
                }
                ObjectNormal::Box => { /* Pass over */ }
                ObjectNormal::Robot => unreachable!("Robot cannot be in the path iterating over"),
            }
            space_searcher += dir;
        }

        let boxes_to_move = times - _times;
        for _ in 0..boxes_to_move {
            // Make the prev robot pos empty
            *self.map.get_mut(&self.robot_pos).unwrap() = ObjectNormal::Empty;
            self.robot_pos += dir;
        }
        // Place the robot at the new position
        *self.map.get_mut(&self.robot_pos).unwrap() = ObjectNormal::Robot;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ObjectNormal {
    Wall,
    Empty,
    Robot,
    Box,
}

impl Debug for ObjectNormal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectNormal::Wall => write!(f, "#"),
            ObjectNormal::Empty => write!(f, "."),
            ObjectNormal::Robot => write!(f, "@"),
            ObjectNormal::Box => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ObjectMalfunctioning {
    Wall,
    Empty,
    Robot,
    BoxLeft,
    BoxRight,
}

impl Debug for ObjectMalfunctioning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectMalfunctioning::Wall => write!(f, "#"),
            ObjectMalfunctioning::Empty => write!(f, "."),
            ObjectMalfunctioning::Robot => write!(f, "@"),
            ObjectMalfunctioning::BoxLeft => write!(f, "["),
            ObjectMalfunctioning::BoxRight => write!(f, "]"),
        }
    }
}

impl<T> WarehouseRobot<T> {
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

    fn process_input(input: &[String]) -> (Vec<&str>, Vec<Dir>) {
        let mut iter = input.into_iter();
        let mut map: Vec<&str> = vec![];
        loop {
            let line = iter.next().unwrap();
            if line.is_empty() {
                break;
            }
            map.push(line);
        }

        let mut moves = vec![];
        for line in iter {
            line.chars().for_each(|e| {
                let (c, d) = Self::get_dir(e);
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

        (map, moves)
    }
}

impl From<Vec<String>> for WarehouseRobot<ObjectMalfunctioning> {
    fn from(line: Vec<String>) -> Self {
        let (map, moves) = Self::process_input(&line);

        let mut grid = UnsizedGrid::new(
            map.len(),
            map[0].len() * 2, /* Cuz its double the size */
            ObjectMalfunctioning::Empty,
        );
        let mut robot_pos = None;
        let next = Direction::East;
        for (i, row) in map.into_iter().enumerate() {
            let mut j = 0;
            for c in row.chars() {
                let coord = Coordinate::new(i as i32, j);
                match c {
                    '@' => {
                        if robot_pos.is_some() {
                            panic!("Multiple robots found in the map");
                        }
                        robot_pos = Some(coord);
                        // @.
                        *grid.get_mut(&coord).unwrap() = ObjectMalfunctioning::Robot;
                        *grid.get_mut(&(coord + next)).unwrap() = ObjectMalfunctioning::Empty;
                    }
                    c => {
                        let new = match c {
                            '#' => [ObjectMalfunctioning::Wall, ObjectMalfunctioning::Wall],
                            '.' => [ObjectMalfunctioning::Empty, ObjectMalfunctioning::Empty],
                            'O' => [
                                ObjectMalfunctioning::BoxLeft,
                                ObjectMalfunctioning::BoxRight,
                            ],
                            _ => unreachable!(),
                        };
                        *grid.get_mut(&coord).unwrap() = new[0];
                        *grid.get_mut(&(coord + next)).unwrap() = new[1];
                    }
                }
                j += 2;
            }
        }

        Self {
            map: grid,
            robot_pos: robot_pos.unwrap(),
            moves,
        }
    }
}

impl From<Vec<String>> for WarehouseRobot<ObjectNormal> {
    fn from(lines: Vec<String>) -> Self {
        let (map, moves) = Self::process_input(&lines);

        let mut grid = UnsizedGrid::new(map.len(), map[0].len(), ObjectNormal::Empty);
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
                        *grid.get_mut(&coord).unwrap() = ObjectNormal::Robot;
                    }
                    c => {
                        *grid.get_mut(&coord).unwrap() = match c {
                            '#' => ObjectNormal::Wall,
                            '.' => ObjectNormal::Empty,
                            'O' => ObjectNormal::Box,
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
