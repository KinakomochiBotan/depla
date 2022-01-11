use super::{
    Game,
    Player
};

use crate::Index;
use anyhow::Result;
use rand::Rng;

pub struct RandomPlayer<R> {
    rng: R,
    buffer: Vec<Index>
}

impl<R> RandomPlayer<R> {
    #[inline]
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            buffer: Vec::new()
        }
    }
}

impl<R: Rng> Player for RandomPlayer<R> {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Index> {
        self.buffer.clear();

        for row in 0..8 {
            for column in 0..8 {
                let index = Index::at(row, column).unwrap();

                if game.board.legal().is_set(index) {
                    self.buffer.push(index);
                }

            }
        }

        return Result::Ok(self.buffer[self.rng.gen_range(0..self.buffer.len())]);
    }
}
