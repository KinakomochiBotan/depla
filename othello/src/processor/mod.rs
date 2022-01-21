mod moves;
mod player;

pub use self::{
    moves::*,
    player::*
};

use crate::api::Game;
use anyhow::Result;

pub fn play<M: Moves>(moves: &mut M) -> Result<(u32, u32)> {
    let mut game = Game::new();
    let mut will_pass = false;

    loop {

        if game.board().legal().value() == 0 {
            if will_pass {
                break;
            } else {
                game.pass();
                will_pass = true;
                continue;
            }
        }

        game.make_move(moves.get_move(&game)?)?;
        will_pass = false;
    }

    Result::Ok((game.black().count(), game.white().count()))
}
