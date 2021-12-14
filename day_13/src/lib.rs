use std::str::FromStr;
use std::{fmt, fs};

type CoOrdPair = ((usize, usize), (usize, usize));

pub fn input_dot_grid_instructions() -> (DotGrid, Instructions) {
    let input =
        fs::read_to_string("day_13/input").expect("Failed to read input file");
    string_dot_grid_instructions(&input)
}

fn string_dot_grid_instructions(input: &str) -> (DotGrid, Instructions) {
    let mut halves = input.split("\n\n");

    let grid_str = halves.next().expect("Bad input");
    let trues = grid_str
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            let x = split
                .next()
                .expect("Bad input")
                .parse::<usize>()
                .expect("Bad input");
            let y = split
                .next()
                .expect("Bad input")
                .parse::<usize>()
                .expect("Bad input");
            (x, y)
        })
        .collect::<Vec<_>>();
    println!("Biggest x: {}", trues.iter().map(|(x, _)| *x).max().unwrap());
    println!("Biggest y: {}", trues.iter().map(|(_, y)| *y).max().unwrap());
    let cols = trues.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let rows = trues.iter().map(|(_, y)| *y).max().unwrap() + 1;
    println!("Going to make the grid {}x{}", cols, rows);

    let mut grid = DotGrid::new(rows, cols);
    trues
        .into_iter()
        .for_each(|(x, y)| *grid.get_mut(x, y) = true);

    let instructions = halves
        .next()
        .expect("Bad input")
        .lines()
        .map(|line| Fold::from_str(line).expect("Bad instruction"))
        .collect();

    (grid, instructions)
}

pub type Instructions = Vec<Fold>;

pub struct DotGrid {
    inner: Vec<bool>,
    rows: usize,
    cols: usize,
}

impl DotGrid {
    fn new(rows: usize, cols: usize) -> Self {
        DotGrid {
            inner: vec![false; cols * rows],
            rows,
            cols,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.inner[self.cols * y + x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self.inner[self.cols * y + x]
    }

    fn fold_iter(
        rows: usize,
        cols: usize,
        Fold(axis, loc): Fold,
    ) -> Box<dyn Iterator<Item = CoOrdPair> + 'static> {
        match axis {
            Axis::X => {
                // Half we're retaining
                let retaining = (0..loc).into_iter().flat_map(move |x| {
                    (0..rows).into_iter().map(move |y| (x, y))
                });
                // Half we're getting rid of / folding
                let folding =
                    (loc + 1..cols).into_iter().rev().flat_map(move |x| {
                        (0..rows).into_iter().map(move |y| (x, y))
                    });
                Box::new(retaining.zip(folding))
            }
            Axis::Y => {
                // Half we're retaining
                let retaining = (0..loc).into_iter().flat_map(move |y| {
                    (0..cols).into_iter().map(move |x| (x, y))
                });
                let r = retaining.collect::<Vec<_>>();
                //println!("r ({}): {:?}", r.len(), &r);
                // Half we're getting rid of / folding
                let folding =
                    (loc + 1..rows).into_iter().rev().flat_map(move |y| {
                        (0..cols).into_iter().map(move |x| (x, y))
                    });
                let f = folding.collect::<Vec<_>>();
                //println!("f ({}): {:?}", f.len(), &f);
                Box::new(r.into_iter().zip(f.into_iter()))
            }
        }
    }

    pub fn fold(&mut self, fold: Fold) {
        match fold.0 {
            Axis::X => assert_eq!(
                fold.1 * 2 + 1,
                self.cols,
                "Not folding down the centre (x)"
            ),
            Axis::Y => assert_eq!(
                fold.1 * 2 + 1,
                self.rows,
                "Not folding down the centre (y)"
            ),
        }
        println!("{:?} on {}x{}", &fold, self.cols, self.rows);
        Self::fold_iter(self.rows, self.cols, fold).for_each(
            |((x_new, y_new), (x_old, y_old))| {
                let _before = self.get(x_new, y_new);
                *self.get_mut(x_new, y_new) |= self.get(x_old, y_old);
                /*println!(
                    "({}, {}) {}\t-> {} \t({}, {}): {}",
                    x_old,
                    y_old,
                    self.get(x_old, y_old),
                    _before,
                    x_new,
                    y_new,
                    self.get(x_new, y_new),
                );*/
            },
        );
        match fold.0 {
            Axis::X => self.cols = fold.1,
            Axis::Y => self.rows = fold.1,
        }
        self.inner.truncate(self.rows * self.cols);
    }

    pub fn dots(&self) -> usize {
        self.inner.iter().filter(|b| **b).count()
    }
}

impl fmt::Display for DotGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .inner
            .chunks(self.cols)
            .map(|chunk| {
                let mut s = chunk
                    .iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>();
                s.push('\n');
                s
            })
            .collect::<String>();
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Copy, Clone)]
pub struct Fold(Axis, usize);

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix("fold along ") {
            let mut split = stripped.split('=');
            let axis = split.next().ok_or(())?;
            let operand =
                split.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;
            match axis {
                "x" => Ok(Fold(Axis::X, operand)),
                "y" => Ok(Fold(Axis::Y, operand)),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    const WEBSITE_EXAMPLE: &str = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5\n";

    #[test]
    fn website_example() {
        let (mut grid, mut instructions) =
            string_dot_grid_instructions(WEBSITE_EXAMPLE);
        println!("Before:\n{}", &grid);
        assert_eq!(grid.rows, 15);
        assert_eq!(grid.cols, 11);
        // Fold(Y, 7)
        grid.fold(instructions.remove(0));
        println!("Actual:\n{}", &grid);
        let expected = "#.##..#..#.\n#...#......\n......#...#\n#...#......\n.#.#..#.###\n...........\n...........\n";
        println!("Expected:\n{}", expected);
        assert_eq!(&grid.to_string(), expected);
    }

    #[test]
    fn fold_iter_x() {
        let mut actual = DotGrid::fold_iter(7, 5, Fold(Axis::X, 2))
            .collect::<Vec<CoOrdPair>>();
        actual.sort_unstable();
        let mut expected: Vec<CoOrdPair> = vec![
            ((0, 0), (4, 0)),
            ((1, 0), (3, 0)),
            ((0, 1), (4, 1)),
            ((1, 1), (3, 1)),
            ((0, 2), (4, 2)),
            ((1, 2), (3, 2)),
            ((0, 3), (4, 3)),
            ((1, 3), (3, 3)),
            ((0, 4), (4, 4)),
            ((1, 4), (3, 4)),
            ((0, 5), (4, 5)),
            ((1, 5), (3, 5)),
            ((0, 6), (4, 6)),
            ((1, 6), (3, 6)),
        ];
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn fold_iter_y() {
        let mut actual = DotGrid::fold_iter(7, 5, Fold(Axis::Y, 3))
            .collect::<Vec<CoOrdPair>>();
        actual.sort_unstable();
        let mut expected: Vec<CoOrdPair> = vec![
            ((0, 0), (0, 6)),
            ((1, 0), (1, 6)),
            ((2, 0), (2, 6)),
            ((3, 0), (3, 6)),
            ((4, 0), (4, 6)),
            ((0, 1), (0, 5)),
            ((1, 1), (1, 5)),
            ((2, 1), (2, 5)),
            ((3, 1), (3, 5)),
            ((4, 1), (4, 5)),
            ((0, 2), (0, 4)),
            ((1, 2), (1, 4)),
            ((2, 2), (2, 4)),
            ((3, 2), (3, 4)),
            ((4, 2), (4, 4)),
        ];
        expected.sort_unstable();
        assert_eq!(actual, expected,);
    }
}
