use aoc_utils_rust::day_setup::Utils;
use std::collections::HashMap;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/13).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 13, Some(29522));
    Utils::run_part(part2, 2, 0, None);
}

fn part1(claw_machines: ClawMachines) -> u64 {
    claw_machines.find_min_ticket_costs()
}

fn part2(claw_machines: ClawMachines) -> u64 {
    // println!("{:#?}", claw_machines);
    claw_machines.find_min_ticket_costs()
}

#[derive(Debug)]
struct Machine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

impl Machine {
    const BUTTON_A_MULTIPLIER: u64 = 3;
    const BUTTON_B_MULTIPLIER: u64 = 1;
    fn minimum_tickets(
        &self,
        curr_prize: (u64, u64),
        depth: u32,
        cache: &mut HashMap<(u64, u64), u64>, // (curr_price, depth) -> min_cost
        (a, b): (u64, u64),                   // (a_steps, b_steps)
    ) -> u64 {
        if let Some(&result) = cache.get(&curr_prize) {
            // println!("Hit cache: {:?} | {:?}", curr_prize, result);
            return result;
        } else if self.is_more_than_prize(curr_prize) {
            return u64::MAX;
        } else if self.prize == curr_prize {
            // println!("{depth} | {:?} | {:?}", curr_prize, (a, b));
            return (a * Self::BUTTON_A_MULTIPLIER) + (b * Self::BUTTON_B_MULTIPLIER);
        }

        
        let result = self
            .minimum_tickets(   // Push A
                Self::add(curr_prize, self.button_a),
                depth + 1,
                cache,
                (a + 1, b),
            ) // Push A
            .min(self.minimum_tickets( // Push B
                Self::add(curr_prize, self.button_b),
                depth + 1,
                cache,
                (a, b + 1),
            )); // Push B

        // Save to cache
        cache.insert(curr_prize, result);

        result
    }

    fn add((ax, ay): (u64, u64), (bx, by): (u64, u64)) -> (u64, u64) {
        (ax + bx, ay + by)
    }

    fn is_more_than_prize(&self, curr_prize: (u64, u64)) -> bool {
        let (i, j) = self.prize.into();
        curr_prize.0 > i || curr_prize.1 > j
    }
}

#[derive(Debug)]
struct ClawMachines {
    machines: Vec<Machine>,
}

impl ClawMachines {
    fn find_min_ticket_costs(&self) -> u64 {
        let mut cache = HashMap::new();

        self.machines
            .iter()
            .map(|machine| {
                cache.clear();
                machine.minimum_tickets((0, 0), 0, &mut cache, (0, 0))
            })
            .filter(|&e| e != u64::MAX)
            .sum()
    }
}

impl From<Vec<String>> for ClawMachines {
    fn from(value: Vec<String>) -> Self {
        let mut value = value.into_iter().peekable();
        let mut machines = vec![];
        // Button A: X+94, Y+34
        loop {
            let line = value.next().unwrap();
            let (_, first_x) = line.split_at("Button A: X+".len());
            let x_num = first_x[..2].parse::<u64>().unwrap();
            let (_, f_y) = first_x[2..].split_once('+').unwrap();
            let y_num = f_y.parse::<u64>().unwrap();

            let a_coord = (x_num, y_num);

            let line = value.next().unwrap();
            let (_, first_x) = line.split_at("Button A: X+".len());
            let x_num = first_x[..2].parse::<u64>().unwrap();
            let (_, f_y) = first_x[2..].split_once('+').unwrap();
            let y_num = f_y.parse::<u64>().unwrap();

            let b_coord = (x_num, y_num);

            let line = value.next().unwrap();
            let line = &line["Prize: X=".len()..];
            let (x, rest) = line.split_once(',').unwrap();

            let x = x.parse::<u64>().unwrap();

            let (_, y) = rest.split_once('=').unwrap();
            let y = y.parse().unwrap();

            let prize = (x, y);

            machines.push(Machine {
                button_a: a_coord,
                button_b: b_coord,
                prize,
            });

            value.next(); // Skip space
            match value.peek() {
                None => break,
                Some(_) => {}
            }
        }

        ClawMachines { machines }
    }
}
