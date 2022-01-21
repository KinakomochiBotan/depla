use othello::{
    api::{
        Position,
        Game
    },
    processor::Moves
};

use anyhow::{
    Result,
    Context as _
};

use rand::{
    Rng,
    seq::IteratorRandom
};

#[derive(Clone, Hash, Debug)]
pub struct RandomMoves<R> {
    rng: R
}

impl<R> RandomMoves<R> {
    #[inline]
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> Moves for RandomMoves<R> {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        let legal = game.board().legal();
        Position::iter().filter(|position| legal.is_set(*position)).choose(&mut self.rng).context("failed to guess")
    }
}
