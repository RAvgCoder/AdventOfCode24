use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::sized_grid::SizedGrid;
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
    Utils::run_part(part1, 1, 14, Some(232253028));
    Utils::run_part(part2, 2, 14, Some(8179));
}

fn part1(mut robot_simulation: RobotSimulation<101, 103>) -> u32 {
    const SIM_SECONDS: u32 = 100;
    robot_simulation.bulk_simulate_robots(SIM_SECONDS);
    robot_simulation.determine_safety_factor()
}

type Buffer<'a> = (&'a mut SizedGrid<char, 103, 101>, &'a mut String);
fn part2(mut robot_simulation: RobotSimulation<101, 103>) -> u16 {
    let mut grid_buff = SizedGrid::new('_');
    let mut buff = String::with_capacity(103 * 101);
    for time in 1..u16::MAX {
        robot_simulation.bulk_simulate_robots(1);
        // Put the thread to sleep for a second
        if robot_simulation.has_made_tree((&mut grid_buff, &mut buff)) {
            #[cfg(debug_assertions)]
            aoc_utils_rust::miscellaneous::dump_grid_to_file(
                &grid_buff,
                "grid_output.txt",
                Some(|e: &char| *e),
            )
            .expect("Failed to dump grid to file");
            return time;
        }
    }
    panic!("No christmas trees found");
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
struct RobotSimulation<const WIDE: u32, const TALL: u32> {
    robot: Vec<Robot>,
}

impl<const WIDE: u32, const TALL: u32> RobotSimulation<WIDE, TALL> {
    fn has_made_tree(&self, (grid, buff): Buffer) -> bool {
        buff.clear();
        // Clear the grid
        for row in grid.iter_mut() {
            row.for_each(|(_, e)| *e = '_');
        }

        for robot in self.robot.iter() {
            *grid.get_mut(&robot.pos.transpose()).unwrap() = '#';
        }

        for row in grid.iter() {
            for (_, e) in row {
                buff.push(*e);
            }
            buff.push('\n');
        }

        buff.contains(&"#".repeat(10))
    }

    fn bulk_simulate_robots(&mut self, seconds: u32) {
        // Move each robot by the number of seconds disregarding out of bounds
        self.robot.iter_mut().for_each(|robot| {
            robot.move_robot(seconds);
            // Move the calibrate the robots to the correct position
            robot.recalibrate(WIDE, TALL);
        });
    }

    fn determine_safety_factor(&self) -> u32 {
        let (wide, tall) = (WIDE, TALL);
        // Example (wide, tall) => (11, 7)
        // Split into quadrants
        // Quadrants produced
        // ..... .....
        // ..1.. ..2..
        // ..... .....
        //
        // ..... .....
        // ..3.. ..4..
        // ..... .....
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
                self.robot
                    .iter()
                    .filter(|robot| {
                        quad_wide.contains(&robot.pos.i) && quad_tall.contains(&robot.pos.j)
                    })
                    .count() as u32
            })
            .fold(1, |acc, x| x * acc)
    }
}

impl<const WIDE: u32, const TALL: u32> From<Vec<String>> for RobotSimulation<WIDE, TALL> {
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
