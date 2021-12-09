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
    fn get(&self, x: usize, y: usize) -> Point {
        let value = self.inner.get(self.cols * y + x).copied().unwrap();
        Point { x, y, value }
    }

    fn iter(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.rows)
            .into_iter()
            .flat_map(move |y| (0..self.cols).into_iter().map(move |x| self.get(x, y)))
    }

    pub fn get_local_minima_risk_level(&self) -> impl Iterator<Item = u8> + '_ {
        self.iter().filter_map(|current| {
            if !self
                .get_adjacent_points(current)
                .into_iter()
                .any(|Point { value, .. }| value <= current.value)
            {
                Some(current.value + 1)
            } else {
                None
            }
        })
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    value: u8,
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
        grid.iter()
            .zip(expected)
            .for_each(|(a, b)| {
                assert_eq!(a.x, b.0);
                assert_eq!(a.y, b.1);
                assert_eq!(a.value, b.2);
            });
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
