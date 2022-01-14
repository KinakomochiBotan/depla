use super::Player;

use crate::game::{
    Position,
    Game
};

use anyhow::Result;

use rand::{
    Rng,
    seq::SliceRandom
};

#[derive(Clone, Hash, Debug)]
pub struct RandomPlayer<R> {
    rng: R,
    buffer: Vec<Position>
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
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        let legal = game.board().legal();
        self.buffer.clear();
        self.buffer.extend(Position::iter().filter(|position| legal.is_set(*position)));
        Result::Ok(*self.buffer.choose(&mut self.rng).unwrap())
    }
}

