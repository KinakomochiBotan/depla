use crate::api::{
    Position,
    Game
};

use anyhow::Result;

pub trait Moves {
    fn get_move(&mut self, game: &Game) -> Result<Position>;
}
