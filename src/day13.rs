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
    Utils::run_part(part2, 2, 13, Some(101214869433312));
}

fn part1(claw_machines: ClawMachines) -> i64 {
    claw_machines.find_min_ticket_costs_like_a_comp_sci_student()
}

fn part2(claw_machines: ClawMachines) -> i64 {
    claw_machines
        .increase_price(10000000000000)
        .find_min_ticket_costs_like_a_math_student()
}

#[derive(Debug, Clone)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    const BUTTON_A_MULTIPLIER: i64 = 3;
    const BUTTON_B_MULTIPLIER: i64 = 1;
    /// Finds the minimum number of tickets required to reach the prize using dynamic programming.
    ///
    /// # Arguments
    ///
    /// * `curr_prize` - The current coordinates of the prize.
    /// * `cache` - A mutable reference to a cache for storing previously computed results.
    /// * `(a, b)` - The number of steps taken for button A and button B respectively.
    ///
    /// # Returns
    ///
    /// The minimum number of tickets required to reach the prize.
    fn minimum_tickets_dp(
        &self,
        curr_prize: (i64, i64),
        cache: &mut HashMap<(i64, i64), i64>, // (curr_price, depth) -> min_cost
        (a, b): (i64, i64),                   // (a_steps, b_steps)
    ) -> i64 {
        if let Some(&result) = cache.get(&curr_prize) {
            return result;
        } else if self.is_more_than_prize(curr_prize) {
            return i64::MAX;
        } else if self.prize == curr_prize {
            return (a * Self::BUTTON_A_MULTIPLIER) + (b * Self::BUTTON_B_MULTIPLIER);
        }

        let mut result = self.minimum_tickets_dp(
            // Push A
            Self::add(curr_prize, self.button_a),
            cache,
            (a + 1, b),
        );
        if result == i64::MAX {
            // We do this as we know that only one solution is ever possible at least for this question
            result = self.minimum_tickets_dp(
                // Push B
                Self::add(curr_prize, self.button_b),
                cache,
                (a, b + 1),
            )
        }

        // Save to cache
        cache.insert(curr_prize, result);

        result
    }

    /// Finds the minimum number of tickets required to reach the prize using linear algebra.
    ///
    /// # Returns
    ///
    /// An `Option` containing the minimum number of tickets required to reach the prize, or `None` if it is not possible.
    fn minimum_tickets_linear_algebra(&self) -> Option<i64> {
        let det = self.determinant(self.button_a, self.button_b);
        let x = self.determinant(self.prize, self.button_b);
        let dx = x / det;

        let y = self.determinant(self.button_a, self.prize);
        let dy = y / det;

        let a = self.button_a.0 * dx + self.button_b.0 * dy;
        let b = self.button_a.1 * dx + self.button_b.1 * dy;

        if (a, b) == self.prize {
            Some(dx * Machine::BUTTON_A_MULTIPLIER + dy * Machine::BUTTON_B_MULTIPLIER)
        } else {
            None
        }
    }

    fn determinant(&self, a: (i64, i64), b: (i64, i64)) -> i64 {
        let (ax, ay) = a;
        let (bx, by) = b;

        (ax * by) - (ay * bx)
    }

    fn increase_price(&mut self, extra: i64) {
        self.prize.0 += extra;
        self.prize.1 += extra;
    }

    fn add((ax, ay): (i64, i64), (bx, by): (i64, i64)) -> (i64, i64) {
        (ax + bx, ay + by)
    }

    fn is_more_than_prize(&self, curr_prize: (i64, i64)) -> bool {
        let (i, j) = self.prize.into();
        curr_prize.0 > i || curr_prize.1 > j
    }
}

#[derive(Debug)]
struct ClawMachines {
    machines: Vec<Machine>,
}

impl ClawMachines {
    fn find_min_ticket_costs_like_a_comp_sci_student(&self) -> i64 {
        let mut cache = HashMap::new();

        self.machines
            .iter()
            .map(|machine| {
                cache.clear();
                machine.minimum_tickets_dp((0, 0), &mut cache, (0, 0))
            })
            .filter(|&e| e != i64::MAX)
            .sum()
    }

    fn find_min_ticket_costs_like_a_math_student(&self) -> i64 {
        self.machines
            .iter()
            .filter_map(Machine::minimum_tickets_linear_algebra)
            .sum()
    }

    fn increase_price(mut self, extra: i64) -> Self {
        self.machines.iter_mut().for_each(|machine| {
            machine.increase_price(extra);
        });
        self
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
            let x_num = first_x[..2].parse::<i64>().unwrap();
            let (_, f_y) = first_x[2..].split_once('+').unwrap();
            let y_num = f_y.parse::<i64>().unwrap();

            let a_coord = (x_num, y_num);

            let line = value.next().unwrap();
            let (_, first_x) = line.split_at("Button A: X+".len());
            let x_num = first_x[..2].parse::<i64>().unwrap();
            let (_, f_y) = first_x[2..].split_once('+').unwrap();
            let y_num = f_y.parse::<i64>().unwrap();

            let b_coord = (x_num, y_num);

            let line = value.next().unwrap();
            let line = &line["Prize: X=".len()..];
            let (x, rest) = line.split_once(',').unwrap();

            let x = x.parse::<i64>().unwrap();

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
