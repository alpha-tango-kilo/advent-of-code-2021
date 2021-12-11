use std::fs;

pub fn input_octopi() -> OctopusGrid {
    let data = fs::read_to_string("day_11/input")
        .expect("Failed to read input file")
        .lines()
        .flat_map(str::chars)
        .map(|c| c.to_digit(10).expect("Bad input") as u8)
        .collect();
    OctopusGrid::new(data, 10, 10)
}

#[derive(Debug)]
pub struct OctopusGrid {
    inner: Vec<u8>,
    rows: usize,
    cols: usize,
    flashes: usize,
    flashed: Vec<bool>,
}

impl OctopusGrid {
    const FLASH_THRESHOLD: u8 = 10;

    fn new(data: Vec<u8>, rows: usize, cols: usize) -> Self {
        let flashed = vec![false; data.len()];
        OctopusGrid {
            inner: data,
            rows,
            cols,
            flashes: 0,
            flashed,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.inner[self.cols * y + x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        self.inner.get_mut(self.cols * y + x).unwrap()
    }

    fn inc(&mut self, x: usize, y: usize) {
        *self.get_mut(x, y) += 1;
    }

    fn has_flashed(&self, x: usize, y: usize) -> bool {
        self.flashed[self.cols * y + x]
    }

    fn co_ord_iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows)
            .into_iter()
            .flat_map(move |y| (0..self.cols).into_iter().map(move |x| (x, y)))
    }

    fn increment_all(&mut self) {
        self.inner.iter_mut().for_each(|octopus| *octopus += 1);
    }

    fn flash(&mut self, x: usize, y: usize) {
        self.flashed[self.cols * y + x] = true;

        let up = y.checked_sub(1);
        let down = y.checked_add(1).filter(|y| *y < self.rows);
        let left = x.checked_sub(1);
        let right = x.checked_add(1).filter(|x| *x < self.cols);

        if let Some(up) = up {
            self.inc(x, up);
            if let Some(left) = left {
                self.inc(left, up)
            }
            if let Some(right) = right {
                self.inc(right, up);
            }
        }
        if let Some(left) = left {
            self.inc(left, y);
        }
        if let Some(right) = right {
            self.inc(right, y);
        }
        if let Some(down) = down {
            self.inc(x, down);
            if let Some(left) = left {
                self.inc(left, down)
            }
            if let Some(right) = right {
                self.inc(right, down);
            }
        }
    }

    fn flash_all(&mut self) {
        loop {
            let flash_these_lads = self
                .co_ord_iter()
                .filter(|(x, y)| self.get(*x, *y) >= Self::FLASH_THRESHOLD)
                .filter(|(x, y)| !self.has_flashed(*x, *y))
                .collect::<Vec<_>>();
            if !flash_these_lads.is_empty() {
                flash_these_lads
                    .into_iter()
                    .for_each(|(x, y)| self.flash(x, y));
            } else {
                break;
            }
        }
    }

    fn reset(&mut self) {
        self.inner.iter_mut().for_each(|octopus| {
            if *octopus >= Self::FLASH_THRESHOLD {
                *octopus = 0
            }
        });
        self.flashed.iter_mut().for_each(|b| *b = false);
    }

    pub fn simulate_n(&mut self, n: usize) {
        for _ in 0..n {
            debug_assert!(self
                .inner
                .iter()
                .all(|o| *o < Self::FLASH_THRESHOLD));
            self.increment_all();
            self.flash_all();
            self.flashes += self.flashed.iter().filter(|b| **b).count();
            self.reset();
        }
    }

    pub fn flashes(&self) -> usize {
        self.flashes
    }
}

#[cfg(test)]
mod test {
    use crate::OctopusGrid;

    #[test]
    fn small_example() {
        let mut octopi = OctopusGrid::new(
            vec![
                1, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1,
                1, 1, 1, 1,
            ],
            5,
            5,
        );

        octopi.simulate_n(1);
        assert_eq!(
            &octopi.inner,
            &[
                3, 4, 5, 4, 3, 4, 0, 0, 0, 4, 5, 0, 0, 0, 5, 4, 0, 0, 0, 4, 3,
                4, 5, 4, 3
            ]
        );
        assert_eq!(octopi.flashes(), 9);

        octopi.simulate_n(1);
        assert_eq!(
            &octopi.inner,
            &[
                4, 5, 6, 5, 4, 5, 1, 1, 1, 5, 6, 1, 1, 1, 6, 5, 1, 1, 1, 5, 4,
                5, 6, 5, 4
            ]
        );
        assert_eq!(octopi.flashes(), 9);
    }

    /* Will to live lost while adding all the commas
    #[test]
    fn big_example() {
        let mut octopi = OctopusGrid::new(
            vec![
                5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5,
                2, 6, 4, 5, 5, 6, 1, 7, 3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3,
                5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6, 4, 5, 2, 1, 7,
                6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6,
                8, 4, 8, 5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
            ],
            10,
            10,
        );
    }
     */
}
