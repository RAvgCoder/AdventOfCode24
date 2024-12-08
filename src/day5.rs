use aoc_utils_rust::day_setup::Utils;
use aoc_utils_rust::graph::Graph;
use std::collections::{HashMap, HashSet};

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2024/day/5).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 5, Some(5091));
    Utils::run_part(part2, 2, 5, Some(4681));
}

fn part1(book: Books) -> u32 {
    book.partition().0.sum_middle_pages()
}

fn part2(book: Books) -> u32 {
    book.partition().1.fix_unordered_books().sum_middle_pages()
}

struct UnorderedBooks {
    rules: HashMap<u8, HashSet<u8>>,
    unordered_books: Vec<Vec<u8>>,
}

impl UnorderedBooks {
    fn fix_unordered_books(self) -> OrderedBooks {
        let mut ordered_pages = Vec::with_capacity(self.unordered_books.len());
        let mut page_set = HashSet::with_capacity(self.unordered_books.len());
        for book in self.unordered_books {
            page_set.clear();
            page_set.extend(&book);

            let adjacency_list = book
                .into_iter()
                .map(|page| {
                    (
                        page,
                        self.rules
                            .get(&page)
                            .unwrap_or(&HashSet::new())
                            .intersection(&page_set)
                            .map(|&e| e)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<HashMap<_, Vec<_>>>();

            let graph = Graph::<_, ()>::from(adjacency_list);

            ordered_pages.push(
                graph
                    .topological_sort()
                    .expect("Cycle detected in graph")
                    .into_iter()
                    .map(|ptr| *graph.get(ptr).unwrap())
                    .collect(),
            );
        }
        OrderedBooks { ordered_pages }
    }
}

struct OrderedBooks {
    ordered_pages: Vec<Vec<u8>>,
}

impl OrderedBooks {
    fn sum_middle_pages(self) -> u32 {
        self.ordered_pages
            .into_iter()
            .map(|page| page[page.len() / 2] as u32)
            .sum()
    }
}

#[derive(Debug)]
struct Books {
    rules: HashMap<u8, HashSet<u8>>,
    books: Vec<Vec<u8>>,
}

impl Books {
    fn partition(self) -> (OrderedBooks, UnorderedBooks) {
        let mut ordered_pages = Vec::with_capacity(self.books.len());
        let mut unordered_pages = Vec::with_capacity(self.books.len());

        let mut page_set = HashSet::new();
        'search_books: for book in self.books {
            for page in &book {
                if !self
                    .rules
                    .get(page)
                    .map(|p| p.is_disjoint(&page_set))
                    .unwrap_or(true)
                {
                    unordered_pages.push(book);
                    page_set.clear();
                    continue 'search_books;
                }
                page_set.insert(*page);
            }
            ordered_pages.push(book);
            page_set.clear();
        }

        (
            OrderedBooks { ordered_pages },
            UnorderedBooks {
                rules: self.rules,
                unordered_books: unordered_pages,
            },
        )
    }
}

impl From<Vec<String>> for Books {
    fn from(value: Vec<String>) -> Self {
        let mut rules_forward = HashMap::with_capacity(value.len());
        let mut pages = Vec::with_capacity(value.len());

        let mut value = value.into_iter();

        while let Some(line) = value.next() {
            if line.is_empty() {
                break;
            }

            // 53|13
            let (before, after) = line.split_at(2);
            rules_forward
                .entry(before.parse().unwrap())
                .or_insert(HashSet::new())
                .insert(after[1..].parse().unwrap());
        }

        for line in value {
            pages.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }

        Self {
            rules: rules_forward,
            books: pages,
        }
    }
}
