use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::GridMut;
use aoc_utils_rust::math::Math;
use std::str::FromStr;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/14).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 14, None);
    Utils::run_part(part2, 2, 0, None);
}

fn part1(mut robot_simulation: RobotSimulation) -> u32 {
    let map_size = (101, 103);
    robot_simulation.determine_safety_factor(100, map_size)
}

fn part2(input: Vec<String>) -> u64 {
    println!("Part 2 {:#?}", input);
    0
}

#[derive(Debug)]
struct Robot {
    pos: Coordinate,
    velocity: Coordinate,
}

impl Robot {
    fn move_robot(&mut self, iterations: u32) {
        let new_velocity = self.velocity * iterations as i32;
        self.pos += new_velocity;
    }

    fn recalibrate(&mut self, i: u32, j: u32) {
        self.pos.i = Math::mod_(self.pos.i as i64, i as u64) as i32;
        self.pos.j = Math::mod_(self.pos.j as i64, j as u64) as i32;
    }
}

#[derive(Debug)]
struct RobotSimulation {
    robot: Vec<Robot>,
}

impl RobotSimulation {
    fn print_grid(&self, (wide, tall): (u32, u32)) {
        let mut unsized_grid = UnsizedGrid::new(tall as _, wide as _, '.');
        for robot in self.robot.iter() {
            *unsized_grid.get_mut(&robot.pos.transpose()).unwrap() = '1';
        }
        for row in unsized_grid.iter_mut() {
            for (c, e) in row {
                if c.j == (wide as i32 / 2) {
                    print!(" ");
                } else {
                    if c.i == ((tall as i32) / 2) {
                        print!(" ");
                        // print!("{}", e);
                    } else {
                        print!("{}", e);
                    }
                }
            }
            println!()
        }
    }

    fn determine_safety_factor(&mut self, seconds: u32, (wide, tall): (u32, u32)) -> u32 {
        // Move each robot by the number of seconds disregarding out of bounds
        self.robot.iter_mut().for_each(|robot| {
            robot.move_robot(seconds);
            // Move the calibrate the robots to the correct position
            robot.recalibrate(wide, tall);
        });

        // (wide, tall) => (11, 7)
        // Quadrants produced
        // ..... .....
        // ..1.. ..2..
        // ..... .....
        //
        // ..... .....
        // ..3.. ..4..
        // ..... .....

        // Split into quadrants
        let q1 = (0..wide as i32 / 2, 0..tall as i32 / 2);
        let q2 = ((wide as i32 / 2) + 1..wide as i32, 0..tall as i32 / 2);
        let q3 = (0..wide as i32 / 2, (tall as i32 / 2) + 1..tall as i32);
        let q4 = (
            (wide as i32 / 2) + 1..wide as i32,
            (tall as i32 / 2) + 1..tall as i32,
        );

        [q1, q2, q3, q4]
            .into_iter()
            .map(|(quad_wide, quad_tall)| {
                self
                    .robot
                    .iter()
                    .filter(|robot| {
                        quad_wide.contains(&robot.pos.i) && quad_tall.contains(&robot.pos.j)
                    })
                    .count() as u32
            })
            .fold(1, |acc, x| x * acc)
    }
}

impl From<Vec<String>> for RobotSimulation {
    fn from(value: Vec<String>) -> Self {
        let mut robots = Vec::with_capacity(value.len());

        for line in value {
            let mut line = line.split_whitespace();
            let (pos, velocity) = (line.next().unwrap(), line.next().unwrap());
            // "p=x,y" => "x,y" => Coordinate::from_str("x,y")
            let pos = Coordinate::from_str(&pos[2..]).unwrap();
            let velocity = Coordinate::from_str(&velocity[2..]).unwrap();
            robots.push(Robot { pos, velocity });
        }

        RobotSimulation { robot: robots }
    }
}
