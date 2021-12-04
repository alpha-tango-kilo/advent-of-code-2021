use ndarray::Array2;
use std::fs;

pub fn input_game() -> (Vec<u8>, GameState) {
    let input =
        fs::read_to_string("day_04/input").expect("Failed to read input file");

    let draw = input
        .lines()
        .next()
        .expect("Bad input")
        .split(',')
        .map(|num_str| num_str.parse::<u8>().expect("Bad input"))
        .collect::<Vec<_>>();

    let boards = input
        .split("\n\n")
        .skip(1)
        .map(|board_str| {
            let mut array = Array2::<u8>::default((5, 5));
            board_str
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<u8>().expect("Bad input"))
                .zip(array.iter_mut())
                .for_each(|(num, cell)| *cell = num);
            array
        })
        .collect::<Vec<_>>();

    (draw, GameState::new(boards))
}

#[derive(Debug)]
pub struct GameState {
    inner: Vec<Array2<u8>>,
    played: Vec<u8>,
    already_won: Vec<usize>,
}

impl GameState {
    pub fn new(boards: Vec<Array2<u8>>) -> Self {
        GameState {
            inner: boards,
            played: Vec::new(),
            already_won: Vec::new(),
        }
    }

    pub fn draw(&mut self, ball: u8) {
        self.played.push(ball);
    }

    // For first exercise only, returns the index of the winner
    pub fn winner(&self) -> Option<usize> {
        self.inner.iter().enumerate().find_map(|(index, board)| {
            if self.has_won(board) {
                Some(index)
            } else {
                None
            }
        })
    }

    /// Returns the indexes of the winner(s)
    pub fn winners(&mut self) -> Option<Vec<usize>> {
        let new_winner_indexes = self
            .inner
            .iter()
            .enumerate()
            .filter_map(|(index, board)| {
                if !self.already_won.contains(&index) && self.has_won(board) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !new_winner_indexes.is_empty() {
            self.already_won.extend_from_slice(&new_winner_indexes);
            Some(new_winner_indexes)
        } else {
            None
        }
    }

    pub fn score_of(&self, player_index: usize) -> u32 {
        let sum_of_unmarked = self
            .inner
            .get(player_index)
            .expect("Out of range player_index")
            .into_iter()
            .filter(|num| !self.played.contains(num))
            .map(|u8_ref| *u8_ref as u32)
            .sum::<u32>();
        let last_played =
            *self.played.last().expect("No balls have been drawn yet") as u32;
        sum_of_unmarked * last_played
    }

    pub fn players(&self) -> usize {
        self.inner.len()
    }

    fn has_won(&self, board: &Array2<u8>) -> bool {
        if self.played.len() < 5 {
            false
        } else {
            let win_by_rows = board
                .rows()
                .into_iter()
                .any(|row| row.iter().all(|n| self.played.contains(n)));
            let win_by_cols = board
                .columns()
                .into_iter()
                .any(|col| col.iter().all(|n| self.played.contains(n)));
            // Yes this is slightly inefficient because it always checks both,
            // but it looks way better
            win_by_rows || win_by_cols
        }
    }
}
