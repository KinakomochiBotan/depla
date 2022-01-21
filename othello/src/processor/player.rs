use crate::api::{
    Position,
    Color,
    Game
};

use super::Moves;
use anyhow::Result;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct PlayersMoves<'a, B, W> {
    black: &'a mut B,
    white: &'a mut W
}

impl<'a, B, W> PlayersMoves<'a, B, W> {
    #[inline]
    pub fn new(black: &'a mut B, white: &'a mut W) -> Self {
        Self { black, white }
    }
}

impl<'a, B: Moves, W: Moves> Moves for PlayersMoves<'a, B, W> {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        match game.color() {
            Color::Black => self.black.get_move(game),
            Color::White => self.white.get_move(game)
        }
    }
}
