use aoc_utils_rust::coordinate_system::direction::Direction;
use aoc_utils_rust::coordinate_system::Coordinate;
use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::grid::sized_grid::SizedGrid;
use aoc_utils_rust::grid::{Grid, GridMut};
use aoc_utils_rust::miscellaneous::the_visitor::{TheVisitor, Timer};
use std::collections::VecDeque;
use std::fmt::Debug;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/18).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    // Utils::run_part(part1, 1, 18, Some(262));
    Utils::run_part(part2, 2, 18, Some((22, 20)));
}
const GRID_SIZE: usize = 71;
type TimerMap = SizedGrid<Timer, GRID_SIZE, GRID_SIZE>;
type Map = SizedGrid<bool, GRID_SIZE, GRID_SIZE>;
fn part1(corruption_byte_stream: CorruptedByteStream) -> u32 {
    let mut map = SizedGrid::<_, GRID_SIZE, GRID_SIZE>::new(true);
    let mut the_visitor = {
        let backing_grid = SizedGrid::with_size_from(&map, Timer::BLANK);
        TheVisitor::new(backing_grid)
    };
    CorruptedByteStream::toggle_corrupted_bytes(
        &mut map,
        &corruption_byte_stream.corrupted_stream[..1024],
    );
    find_shortest_path(&mut map, &mut the_visitor).expect("I'm guaranteed to find a path")
}

fn part2(mut corruption_byte_stream: CorruptedByteStream) -> (i32, i32) {
    // Flip the coordinates for some reason 🤷‍♂️ (I'm not sure why he flipped the coordinates for part 2)
    corruption_byte_stream
        .corrupted_stream
        .iter_mut()
        .for_each(|c| *c = c.transpose());
    // Find the max corrupted bytes to escape and transpose the result back (I guess u can avoid transposition if u change the
    // start and coordinates but im too lazy for that
    corruption_byte_stream
        .find_max_corrupted_bytes_to_escape()
        .transpose()
        .into()
}

fn find_shortest_path(map: &Map, the_visitor: &mut TheVisitor<TimerMap>) -> Option<u32> {
    let end_coord = map.bottom_right_coordinate();
    let mut queue = VecDeque::with_capacity(map.num_cols());
    queue.push_back((Coordinate::ORIGIN, 0));
    while let Some((next_coord, steps)) = queue.pop_front() {
        if next_coord == end_coord {
            return Some(steps);
        }
        if the_visitor.mark_visited(next_coord) {
            Direction::direction_list()
                .map(|dir| next_coord + dir)
                .iter()
                .filter(|coord| map.get(&coord).is_some()) // Those in bounds
                .filter(|coord| *map.get(&coord).unwrap()) // Only paths not corrupted
                .for_each(|&next_coord| {
                    queue.push_back((next_coord, steps + 1));
                });
        }
    }
    None
}

#[derive(Debug)]
struct CorruptedByteStream {
    corrupted_stream: Box<[Coordinate]>,
}
type ByteSpot = bool;
impl CorruptedByteStream {
    fn toggle_corrupted_bytes(map: &mut Map, corrupted_stream: &[Coordinate]) {
        for coord in corrupted_stream.iter() {
            *map.get_mut(coord).unwrap() ^= true;
        }
    }

    fn find_max_corrupted_bytes_to_escape(&self) -> Coordinate {
        let mut map = SizedGrid::<bool, GRID_SIZE, GRID_SIZE>::new(true);
        let mut the_visitor = {
            let backing_grid = SizedGrid::with_size_from(&map, Timer::BLANK);
            TheVisitor::new(backing_grid)
        };

        let mut l_ptr = 0;
        let mut r_ptr = self.corrupted_stream.len() - 1;
        let mut result = None;
        let list = self.corrupted_stream.as_ref();

        let mut prev_mid = 0;
        while l_ptr <= r_ptr {
            let mid = l_ptr + (r_ptr - l_ptr) / 2;

            Self::toggle_corrupted_bytes(
                &mut map,
                if prev_mid < mid {
                    &list[prev_mid..mid]
                } else {
                    &list[mid..prev_mid]
                },
            );

            prev_mid = mid;

            if find_shortest_path(&map, &mut the_visitor).is_some() {
                result = Some(list[mid]);
                l_ptr = mid + 1;
            } else {
                r_ptr = mid - 1;
            }

            the_visitor.clear();
        }

        result.unwrap()
    }
}

impl From<Vec<String>> for CorruptedByteStream {
    #[inline]
    fn from(input: Vec<String>) -> Self {
        use std::str::FromStr;
        Self {
            corrupted_stream: input
                .iter()
                .map(|str| Coordinate::from_str(str).unwrap())
                .collect::<Box<_>>(),
        }
    }
}
