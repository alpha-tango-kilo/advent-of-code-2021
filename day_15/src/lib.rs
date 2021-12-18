use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{fs, usize};

const DESTINATION: (usize, usize) = (99, 99);

pub fn input_chiton_grid() -> ChitonGrid {
    let input =
        fs::read_to_string("day_15/input").expect("Failed to read input file");
    string_chiton_grid(&input)
}

fn string_chiton_grid(s: &str) -> ChitonGrid {
    let cols = s.find('\n').expect("Bad input");
    let rows = s.lines().count();
    let inner = s
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u8)
        .collect::<Vec<_>>();
    let distances = vec![u32::MAX; inner.len()];
    ChitonGrid { inner, distances, rows, cols }
}

pub struct ChitonGrid {
    inner: Vec<u8>,
    distances: Vec<u32>,
    pub rows: usize,
    pub cols: usize,
}

impl ChitonGrid {
    fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.cols, "x too big");
        assert!(y < self.rows, "y too big");
        self.inner[self.cols * y + x]
    }

    fn get_dist(&self, x: usize, y: usize) -> u32 {
        assert!(x < self.cols, "x too big");
        assert!(y < self.rows, "y too big");
        self.distances[self.cols * y + x]
    }

    fn get_dist_mut(&mut self, x: usize, y: usize) -> &mut u32 {
        assert!(x < self.cols, "x too big");
        assert!(y < self.rows, "y too big");
        &mut self.distances[self.cols * y + x]
    }

    // A*
    pub fn most_efficient_route(&mut self, from: (usize, usize)) -> u32 {
        // Ord for Exploration is implemented such that smaller manhatten
        // distances will come first
        let mut heap: BinaryHeap<Exploration> = BinaryHeap::new();
        heap.push(Exploration {
            x: from.0,
            y: from.1,
            weight: 0,
            depth: 0,
        });

        // https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/15.rs

        while let Some(exploration) = heap.pop() {
            if exploration.weight < self.get_dist(exploration.x, exploration.y) {
                *self.get_dist_mut(exploration.x, exploration.y) = exploration.weight;
                //println!("Distances {:?}", &self.distances);
                if let Some(solution) = self.explore(&mut heap, exploration) {
                    return solution;
                }
            }
        }
        *self.distances.last().unwrap()
    }

    fn explore(
        &self,
        heap: &mut BinaryHeap<Exploration>,
        exploration: Exploration,
    ) -> Option<u32> {
        let mut new_prospects =
            self.get_adjacent_points(exploration.x, exploration.y);
        new_prospects.sort_unstable_by(|a, b| {
            manhatten(*a, DESTINATION).cmp(&manhatten(*b, DESTINATION))
        });
        // Lowest manhatten distance last as we are using a stack
        let mut stack_entries_iter = new_prospects
            .into_iter()
            .rev()
            .map(|(x, y)| Exploration {
                x,
                y,
                weight: exploration.weight + self.get(x, y) as u32,
                depth: exploration.depth + 1,
            })
            .peekable();
        match stack_entries_iter.peek() {
            Some(ex) if (ex.x, ex.y) == DESTINATION => {
                // Solution found!
                Some(ex.weight)
            }
            _ => {
                heap.extend(stack_entries_iter);
                None
            }
        }
    }

    fn get_adjacent_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::with_capacity(4);
        // Up
        if let Some(up) = y.checked_sub(1) {
            v.push((x, up));
        }
        // Down
        if let Some(down) = y.checked_add(1).filter(|y| *y < self.rows) {
            v.push((x, down));
        }
        // Left
        if let Some(left) = x.checked_sub(1) {
            v.push((left, y));
        }
        // Right
        if let Some(right) = x.checked_add(1).filter(|x| *x < self.cols) {
            v.push((right, y));
        }
        v
    }
}

#[derive(Debug, Copy, Clone)]
struct Exploration {
    x: usize,
    y: usize,
    weight: u32,
    depth: usize,
}

impl PartialEq for Exploration {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}

impl Eq for Exploration {}

impl PartialOrd for Exploration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        manhatten(DESTINATION, (self.x, self.y))
            .partial_cmp(&manhatten(DESTINATION, (other.x, other.y)))
            .map(Ordering::reverse)
    }
}

impl Ord for Exploration {
    fn cmp(&self, other: &Self) -> Ordering {
        manhatten(DESTINATION, (self.x, self.y))
            .cmp(&manhatten(DESTINATION, (other.x, other.y)))
            .reverse()
    }
}

fn manhatten(destination: (usize, usize), current: (usize, usize)) -> usize {
    let x_diff = if destination.0 > current.0 {
        destination.0 - current.0
    } else {
        current.0 - destination.0
    };
    let y_diff = if destination.1 > current.1 {
        destination.1 - current.1
    } else {
        current.1 - destination.1
    };
    x_diff + y_diff
}
