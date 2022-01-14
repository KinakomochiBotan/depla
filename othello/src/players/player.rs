use super::Players;

use crate::game::{
    Position,
    DiscColor,
    Game
};

use anyhow::Result;

pub trait Player {
    fn get_move(&mut self, game: &Game) -> Result<Position>;
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct DefaultPlayers<'a, B, W> {
    black: &'a mut B,
    white: &'a mut W
}

impl<'a, B, W> DefaultPlayers<'a, B, W> {
    #[inline]
    pub fn new(black: &'a mut B, white: &'a mut W) -> Self {
        Self {
            black,
            white
        }
    }
}

impl<'a, B: Player, W: Player> Players for DefaultPlayers<'a, B, W> {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        match game.color() {
            DiscColor::Black => self.black.get_move(game),
            DiscColor::White => self.white.get_move(game)
        }
    }
}
