#[cfg(feature = "player")]
mod player;

#[cfg(feature = "human")]
mod human;

#[cfg(feature = "random")]
mod random;

#[cfg(feature = "player")]
pub use self::player::*;

#[cfg(feature = "human")]
pub use self::human::*;

#[cfg(feature = "random")]
pub use self::random::*;

use crate::game::{
    Position,
    Game
};

use anyhow::Result;

pub trait Players {
    fn get_move(&mut self, game: &Game) -> Result<Position>;

    #[inline]
    fn play(&mut self) -> Result<(u32, u32)> {
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

            game.put(self.get_move(&game)?)?;
            will_pass = false;
        }

        Result::Ok((game.black().count(), game.white().count()))
    }

}
