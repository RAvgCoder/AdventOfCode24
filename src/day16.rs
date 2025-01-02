use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::graph::static_graph::{EdgeRelationship, StaticGraph, StaticNodePtr};
use aoc_utils_rust::grid::unsized_grid::UnsizedGrid;
use aoc_utils_rust::grid::{Grid, GridMut};
use aoc_utils_rust::miscellaneous::dump_grid_to_file;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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
    Utils::run_part(part1, 1, 16, Some(65436));
    Utils::run_part(part2, 2, 16, Some(489));
}

fn part1(reindeer_maze: ReindeerMaze) -> u32 {
    reindeer_maze.find_lowest_cost()
}

fn part2(reindeer_maze: ReindeerMaze) -> u32 {
    reindeer_maze.count_tiles_in_best_path()
}

#[derive(Debug)]
struct ReindeerMaze {
    start: Coordinate,
    end: Coordinate,
    maze: UnsizedGrid<Objects>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    cost: u32,
    direction: Direction,
    curr_coord: Coordinate,
    graph_ptr: StaticNodePtr,
}

impl Location {
    fn key(&self) -> (Coordinate, Direction) {
        (self.curr_coord, self.direction)
    }

    fn next(
        &self,
        graph: &mut StaticGraph<Coordinate, ()>,
        grid: &UnsizedGrid<Objects>,
        min_cost: u32,
    ) -> Vec<Location> {
        let mut res = vec![];
        for stats in [
            (self.cost + ReindeerMaze::BASE_MULTIPLIER, self.direction), // Move in same direction
            (
                // Move Left
                self.cost + ReindeerMaze::NINETY_DEGREE_TURN_MULTIPLIER,
                self.direction.rotate_90(),
            ),
            (
                // Move Right
                self.cost + ReindeerMaze::NINETY_DEGREE_TURN_MULTIPLIER,
                self.direction.rotate_270(),
            ),
        ] {
            if *grid.get(&(self.curr_coord + stats.1)).unwrap() == Objects::Wall
                || stats.0 > min_cost
            {
                continue;
            }

            res.push(Location {
                cost: stats.0,
                direction: stats.1,
                curr_coord: self.curr_coord + stats.1,
                graph_ptr: {
                    let node = graph.add_node(self.curr_coord + stats.1);
                    graph
                        .add_edge(node, self.graph_ptr, EdgeRelationship::AToB(()))
                        .unwrap();
                    node
                },
            });
        }
        res
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl ReindeerMaze {
    const BASE_MULTIPLIER: u32 = 1;
    const NINETY_DEGREE_TURN_MULTIPLIER: u32 = 1001;

    fn count_tiles_in_best_path(&self) -> u32 {
        let mut cost_at_goal: Option<u32> = None;
        let mut graph = StaticGraph::new();
        let start_ptr = graph.add_node(self.start);
        let end_ptr = graph.add_node(self.end);
        let mut queue = BinaryHeap::from_iter(
            Location {
                cost: 0,
                direction: Direction::East,
                curr_coord: self.start,
                graph_ptr: start_ptr,
            }
            .next(&mut graph, &self.maze, cost_at_goal.unwrap_or(u32::MAX))
            .into_iter()
            .map(|x| Reverse(x)),
        );

        fn count_visited(
            end_ptr: StaticNodePtr,
            graph: &StaticGraph<Coordinate, ()>,
            unsized_grid: &UnsizedGrid<Objects>,
        ) -> u32 {
            #[cfg(debug_assertions)]
            {
                // Print the path to a file
                let x = graph
                    .get_nodes_reachable_from(end_ptr)
                    .iter()
                    .map(|x| *graph.get(*x).unwrap())
                    .collect::<HashSet<_>>();
                dump_grid_to_file(
                    &UnsizedGrid::transform_from(unsized_grid, |(coord, obj)| {
                        if x.contains(&coord) {
                            return 'O';
                        }
                        match obj {
                            Objects::Wall => '#',
                            Objects::Path => '.',
                            Objects::Start => 'S',
                            Objects::End => 'E',
                        }
                    }),
                    None,
                    false,
                    Some(|e: &char| *e),
                )
                .unwrap();
            }
            println!("{}", graph.len());
            graph
                .get_nodes_reachable_from(end_ptr)
                .iter()
                .map(|x| *graph.get(*x).unwrap())
                .collect::<HashSet<_>>()
                .len() as u32
        }

        let mut cache = HashMap::new();
        while let Some(Reverse(location)) = queue.pop() {
            if let Some(goal_cost) = cost_at_goal {
                if location.cost > goal_cost {
                    return count_visited(end_ptr, &graph, &self.maze);
                }
            }

            if location.curr_coord == self.end {
                cost_at_goal = Some(location.cost);
                graph
                    .add_edge(end_ptr, location.graph_ptr, EdgeRelationship::AToB(()))
                    .unwrap()
            } else {
                let res = *cache.get(&location.key()).unwrap_or(&u32::MAX);
                if res >= location.cost {
                    cache.insert(location.key(), location.cost);
                    queue.extend(
                        location
                            .next(&mut graph, &self.maze, cost_at_goal.unwrap_or(u32::MAX))
                            .into_iter()
                            .map(|x| Reverse(x)),
                    );
                }
            }
        }

        count_visited(end_ptr, &graph, &self.maze)
    }

    fn find_lowest_cost(&self) -> u32 {
        const INFINITY: u32 = u32::MAX;

        let mut min_score_grid = UnsizedGrid::transform_from(&self.maze, |_| INFINITY);

        let mut queue = VecDeque::with_capacity(self.maze.num_rows() * self.maze.num_cols());

        Direction::direction_list()
            .map(|dir| (self.start + dir, dir))
            .into_iter()
            .filter(|(coord, _)| *self.maze.get(coord).unwrap() != Objects::Wall)
            .for_each(|(coord, dir)| {
                if dir == Direction::East {
                    queue.push_back((coord, dir, Self::BASE_MULTIPLIER));
                } else {
                    queue.push_back((coord, dir, Self::NINETY_DEGREE_TURN_MULTIPLIER))
                }
            });

        while let Some((curr_coord, curr_dir, curr_score)) = queue.pop_front() {
            {
                let obj = *self.maze.get(&curr_coord).unwrap();
                match obj {
                    Objects::Wall | Objects::Start => continue,
                    Objects::End => {
                        let curr_min = min_score_grid.get_mut(&curr_coord).unwrap();
                        *curr_min = curr_score.min(*curr_min);
                        continue;
                    }
                    Objects::Path => {
                        let curr_min = min_score_grid.get_mut(&curr_coord).unwrap();
                        if *curr_min <= curr_score {
                            continue;
                        }
                        *curr_min = curr_score.min(*curr_min);
                    }
                }
            }

            for next_dir in Direction::direction_list() {
                // No point going backwards
                if next_dir == curr_dir.rotate_180() {
                    continue;
                }
                let next_coord = curr_coord + next_dir;
                let new_score = curr_score
                    + if next_dir == curr_dir.rotate_180() || next_dir == curr_dir {
                        Self::BASE_MULTIPLIER
                    } else {
                        Self::NINETY_DEGREE_TURN_MULTIPLIER
                    };

                queue.push_back((next_coord, next_dir, new_score));
            }
        }

        // Retrieve answer from end coordinate
        *min_score_grid.get(&self.end).unwrap()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Objects {
    Wall,
    Path,
    Start,
    End,
}

impl Debug for Objects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Objects::Wall => write!(f, "#"),
            Objects::Path => write!(f, "."),
            Objects::Start => write!(f, "S"),
            Objects::End => write!(f, "E"),
        }
    }
}

impl From<Vec<String>> for ReindeerMaze {
    fn from(value: Vec<String>) -> Self {
        let mut maze = UnsizedGrid::new(value.len(), value[0].len(), Objects::Path);
        let mut start = None;
        let mut end = None;
        for (i, line) in value.iter().enumerate() {
            if line.is_empty() {
                break;
            }
            for (j, e) in line.chars().enumerate() {
                let obj = match e {
                    '#' => Objects::Wall,
                    '.' | 'O' | 'v' | '>' | '^' | '<' => Objects::Path,
                    'S' => {
                        start = Some(Coordinate::new(i as i32, j as i32));
                        Objects::Start
                    }
                    'E' => {
                        end = Some(Coordinate::new(i as i32, j as i32));
                        Objects::End
                    }
                    _ => panic!("Invalid character in maze `{e}`"),
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
