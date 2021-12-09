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

pub struct Grid {
    inner: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.inner.get(self.cols * y + x).copied().unwrap()
    }

    fn co_ord_iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows)
            .into_iter()
            .flat_map(|y| (0..self.cols).into_iter().map(move |x| (x, y)))
    }

    pub fn get_local_minima_risk_level(&self) -> impl Iterator<Item = u8> + '_ {
        self.co_ord_iter().filter_map(|(x, y)| {
            let current = self.get(x, y);
            // Up
            if let Some(up) = y.checked_sub(1) {
                if self.get(x, up) <= current {
                    return None;
                }
            }
            // Down
            if let Some(down) = y.checked_add(1).filter(|y| *y < self.rows) {
                if self.get(x, down) <= current {
                    return None;
                }
            }
            // Left
            if let Some(left) = x.checked_sub(1) {
                if self.get(left, y) <= current {
                    return None;
                }
            }
            // Right
            if let Some(right) = x.checked_add(1).filter(|x| *x < self.cols) {
                if self.get(right, y) <= current {
                    return None;
                }
            }
            Some(current + 1)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::Grid;

    #[test]
    fn grid_get() {
        let grid = Grid {
            inner: vec![1, 2, 3, 4, 5, 6, 7, 8],
            rows: 4,
            cols: 2,
        };
        assert_eq!(grid.get(1, 0), 2);
        assert_eq!(grid.get(1, 2), 6);
        assert_eq!(grid.get(0, 1), 3);
    }

    #[test]
    fn grid_co_ord_iter() {
        let grid = Grid {
            inner: vec![1, 2, 3, 4, 5, 6, 7, 8],
            rows: 2,
            cols: 4,
        };
        let expected: Vec<(usize, usize)> = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
        ];
        grid.co_ord_iter()
            .zip(expected)
            .for_each(|(a, b)| assert_eq!(a, b));
    }

    #[test]
    fn risk_levels() {
        // From website
        let input = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8,
            5, 6, 7, 8, 9, 8, 9, 2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9,
            9, 6, 5, 6, 7, 8,
        ];
        let expected: Vec<u8> = vec![2, 1, 6, 6];
        let grid = Grid {
            inner: input,
            rows: 5,
            cols: 10,
        };
        grid.get_local_minima_risk_level()
            .zip(expected)
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }
}
