mod human;
mod random;

pub use self::{
    human::HumanPlayer,
    random::RandomPlayer
};

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

pub struct DefaultPlayers<B, W> {
    black: B,
    white: W
}

impl<B, W> DefaultPlayers<B, W> {

    #[inline]
    pub fn new(black: B, white: W) -> Self {
        Self {
            black,
            white
        }
    }

    #[inline]
    pub fn to(self) -> (B, W) {
        (self.black, self.white)
    }

}

impl<B: Player, W: Player> Players for DefaultPlayers<B, W> {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Index> {
        match game.player_type {
            PlayerType::Black => self.black.get_move(game),
            PlayerType::White => self.white.get_move(game)
        }
    }
}
