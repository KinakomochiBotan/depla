use crate::Index;

use super::{
    PlayerType,
    Game,
    Players
};

use anyhow::Result;

pub trait Player {
    fn get_move(&mut self, game: &Game) -> Result<Index>;
}

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
    fn get_move(&mut self, game: &Game) -> Result<Index> {
        match game.player_type {
            PlayerType::Black => self.black.get_move(game),
            PlayerType::White => self.white.get_move(game)
        }
    }
}
