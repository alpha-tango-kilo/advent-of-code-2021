use std::collections::HashSet;
use std::fs;

pub fn input_grid() -> Grid {
    let input =
        fs::read_to_string("day_09/input").expect("Failed to read input file");
    let lines = input.lines().count();
    let line_length = input.lines().next().expect("Bad input").trim_end().len();
    let numbers = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|n| n as u8))
        .collect::<Vec<_>>();
    Grid {
        inner: numbers,
        rows: lines,
        cols: line_length,
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Grid {
    inner: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Point {
        let value = self.inner.get(self.cols * y + x).copied().unwrap();
        Point { x, y, value }
    }

    fn iter(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.rows).into_iter().flat_map(move |y| {
            (0..self.cols).into_iter().map(move |x| self.get(x, y))
        })
    }

    fn get_local_minima(&self) -> impl Iterator<Item = Point> + '_ {
        self.iter().filter(|current| {
            !self
                .get_adjacent_points(*current)
                .into_iter()
                .any(|Point { value, .. }| value <= current.value)
        })
    }

    pub fn get_local_minima_risk_level(&self) -> impl Iterator<Item = u8> + '_ {
        self.get_local_minima().map(|Point { value, .. }| value + 1)
    }

    pub fn get_basins(&self) -> impl Iterator<Item = Basin> {
        self.get_local_minima()
            .map(move |point| Basin::new(point, self))
    }

    fn get_adjacent_points(&self, Point { x, y, .. }: Point) -> Vec<Point> {
        let mut v = Vec::with_capacity(4);
        // Up
        if let Some(up) = y.checked_sub(1) {
            v.push(self.get(x, up));
        }
        // Down
        if let Some(down) = y.checked_add(1).filter(|y| *y < self.rows) {
            v.push(self.get(x, down));
        }
        // Left
        if let Some(left) = x.checked_sub(1) {
            v.push(self.get(left, y));
        }
        // Right
        if let Some(right) = x.checked_add(1).filter(|x| *x < self.cols) {
            v.push(self.get(right, y));
        }
        v
    }

    fn get_adjacent_points_filtered(
        &self,
        point: Point,
    ) -> impl Iterator<Item = Point> {
        self.get_adjacent_points(point).into_iter().filter(
            move |Point { value, .. }| {
                *value < 9
                    && value
                        .checked_sub(point.value)
                        .map(|diff| diff == 1)
                        .unwrap_or(false)
            },
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
    value: u8,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Basin<'a> {
    grid: &'a Grid,
    to_explore: HashSet<Point>,
    in_basin: HashSet<Point>,
}

impl<'a> Basin<'a> {
    fn new(origin: Point, grid: &'a Grid) -> Self {
        let mut singleton_origin = HashSet::new();
        singleton_origin.insert(origin);
        Basin {
            grid,
            // Nodes which haven't yet been expanded
            to_explore: singleton_origin.clone(),
            // All nodes, including those which haven't been expanded and the origin
            in_basin: singleton_origin,
        }
    }

    // Breadth first search
    fn propogate(&mut self) {
        self.to_explore = self
            .to_explore
            .iter()
            .flat_map(|point| self.grid.get_adjacent_points_filtered(*point))
            .filter(|p| !self.in_basin.contains(p))
            .collect();
        self.in_basin.extend(&self.to_explore);
    }

    pub fn propogate_all(&mut self) {
        while !self.to_explore.is_empty() {
            self.propogate();
        }
    }

    pub fn basin_size(&self) -> Option<usize> {
        if self.to_explore.is_empty() {
            Some(self.in_basin.len())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Basin, Grid, Point};
    use std::collections::HashSet;

    const WEBSITE_EXAMPLE: [u8; 50] = [
        2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6,
        7, 8, 9, 8, 9, 2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6,
        7, 8,
    ];

    #[test]
    fn grid_get() {
        let grid = Grid {
            inner: vec![1, 2, 3, 4, 5, 6, 7, 8],
            rows: 4,
            cols: 2,
        };
        assert_eq!(grid.get(1, 0).value, 2);
        assert_eq!(grid.get(1, 2).value, 6);
        assert_eq!(grid.get(0, 1).value, 3);
    }

    #[test]
    fn grid_co_ord_iter() {
        let grid = Grid {
            inner: vec![1, 2, 3, 4, 5, 6, 7, 8],
            rows: 2,
            cols: 4,
        };
        let expected: Vec<(usize, usize, u8)> = vec![
            (0, 0, 1),
            (1, 0, 2),
            (2, 0, 3),
            (3, 0, 4),
            (0, 1, 5),
            (1, 1, 6),
            (2, 1, 7),
            (3, 1, 8),
        ];
        grid.iter().zip(expected).for_each(|(a, b)| {
            assert_eq!(a.x, b.0);
            assert_eq!(a.y, b.1);
            assert_eq!(a.value, b.2);
        });
    }

    #[test]
    fn risk_levels() {
        // From website
        let expected: Vec<u8> = vec![2, 1, 6, 6];
        let grid = Grid {
            inner: WEBSITE_EXAMPLE.to_vec(),
            rows: 5,
            cols: 10,
        };
        grid.get_local_minima_risk_level()
            .zip(expected)
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }

    #[test]
    fn minima() {
        let grid = Grid {
            inner: WEBSITE_EXAMPLE.to_vec(),
            rows: 5,
            cols: 10,
        };
        let expected = vec![
            Point {
                x: 1,
                y: 0,
                value: 1,
            },
            Point {
                x: 9,
                y: 0,
                value: 0,
            },
            Point {
                x: 2,
                y: 2,
                value: 5,
            },
            Point {
                x: 6,
                y: 4,
                value: 5,
            },
        ];
        grid.get_local_minima()
            .zip(expected)
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }

    #[test]
    fn adjacent_filtered() {
        let grid = Grid {
            inner: WEBSITE_EXAMPLE.to_vec(),
            rows: 5,
            cols: 10,
        };
        assert_eq!(
            grid.get_adjacent_points_filtered(Point {
                x: 8,
                y: 0,
                value: 1
            })
            .collect::<Vec<_>>(),
            vec![
                Point {
                    x: 8,
                    y: 1,
                    value: 2
                },
                Point {
                    x: 7,
                    y: 0,
                    value: 2
                },
            ],
        );
        assert_eq!(
            grid.get_adjacent_points_filtered(Point {
                x: 2,
                y: 2,
                value: 5
            })
            .collect::<Vec<_>>(),
            vec![
                Point {
                    x: 2,
                    y: 3,
                    value: 6
                },
                Point {
                    x: 3,
                    y: 2,
                    value: 6
                },
            ],
        );
        assert_eq!(
            grid.get_adjacent_points_filtered(Point {
                x: 9,
                y: 0,
                value: 0
            })
            .collect::<Vec<_>>(),
            vec![
                Point {
                    x: 9,
                    y: 1,
                    value: 1
                },
                Point {
                    x: 8,
                    y: 0,
                    value: 1
                },
            ],
        );
    }

    #[test]
    fn basin_propogation() {
        let grid = Grid {
            inner: WEBSITE_EXAMPLE.to_vec(),
            rows: 5,
            cols: 10,
        };
        let origin = Point {
            x: 9,
            y: 0,
            value: 0,
        };
        let mut basin = Basin::new(origin, &grid);

        basin.propogate();
        let to_explore = HashSet::from_iter(vec![
            Point {
                x: 9,
                y: 1,
                value: 1,
            },
            Point {
                x: 8,
                y: 0,
                value: 1,
            },
        ]);
        let mut in_basin = to_explore.clone();
        in_basin.insert(origin);
        assert_eq!(
            basin,
            Basin {
                grid: &grid,
                to_explore,
                in_basin: in_basin.clone(),
            }
        );

        basin.propogate();
        let to_explore = HashSet::from_iter(vec![
            Point {
                x: 8,
                y: 1,
                value: 2,
            },
            Point {
                x: 7,
                y: 0,
                value: 2,
            },
            Point {
                x: 9,
                y: 2,
                value: 2,
            },
        ]);
        in_basin.extend(to_explore.clone());
        let expected = Basin {
            grid: &grid,
            to_explore,
            in_basin: in_basin.clone(),
        };
        assert_eq!(
            basin.to_explore, expected.to_explore,
            "Different exploration prospects"
        );
        assert_eq!(
            basin.in_basin, expected.in_basin,
            "Different elements in basin"
        );
    }

    #[test]
    fn basin_propogate_all() {
        let grid = Grid {
            inner: WEBSITE_EXAMPLE.to_vec(),
            rows: 5,
            cols: 10,
        };
        let origin = Point {
            x: 9,
            y: 0,
            value: 0,
        };
        let mut actual = Basin::new(origin, &grid);

        actual.propogate_all();
        let expected = Basin {
            grid: &grid,
            to_explore: HashSet::new(),
            in_basin: HashSet::from_iter(vec![
                Point {
                    x: 9,
                    y: 0,
                    value: 0,
                },
                Point {
                    x: 8,
                    y: 1,
                    value: 2,
                },
                Point {
                    x: 7,
                    y: 0,
                    value: 2,
                },
                Point {
                    x: 9,
                    y: 2,
                    value: 2,
                },
                Point {
                    x: 6,
                    y: 0,
                    value: 3,
                },
                Point {
                    x: 5,
                    y: 0,
                    value: 4,
                },
                Point {
                    x: 6,
                    y: 1,
                    value: 4,
                },
                Point {
                    x: 8,
                    y: 0,
                    value: 1,
                },
                Point {
                    x: 9,
                    y: 1,
                    value: 1,
                },
            ]),
        };
        /*println!(
            "Expected: {:#?}\nActual: {:#?}",
            &expected.in_basin, &actual.in_basin
        );*/
        assert_eq!(
            actual.to_explore, expected.to_explore,
            "Different exploration prospects"
        );
        assert_eq!(
            actual.basin_size(),
            Some(9),
        );
        assert_eq!(
            actual.in_basin, expected.in_basin,
            "Different elements in basin"
        );
    }

    #[test]
    fn all_basins() {
        let grid = Grid {
            inner: WEBSITE_EXAMPLE.to_vec(),
            rows: 5,
            cols: 10,
        };
        let mut basin_sizes = grid.get_basins()
            .map(|mut basin| {
                basin.propogate_all();
                basin.basin_size().unwrap()
            })
            .collect::<Vec<_>>();
        basin_sizes.sort_unstable();
        assert_eq!(
            &basin_sizes,
            &[3, 9, 9, 14],
            "Incorrect basins sizes produced"
        );
        let answer = basin_sizes
            .iter()
            .rev()
            .take(3)
            .product::<usize>();
        assert_eq!(answer, 1134, "Incorrect product of top 3");
    }
}
