use crate::data::{
    Data,
    AugmentedData,
    Dataset
};

use std::array::IntoIter;
use anyhow::Result;

use othello::{
    game::{
        Position,
        DiscColor,
        Game
    },
    players::Players
};

#[derive(Clone, Debug)]
pub struct WTHORPlayers {
    learn_black: bool,
    learn_white: bool,
    moves: IntoIter<u8, 60>,
    dataset: Dataset
}

impl WTHORPlayers {

    #[inline]
    pub fn new(discs: u8, moves: [u8; 60]) -> Self {
        Self {
            learn_black: discs >= 32,
            learn_white: discs <= 32,
            moves: moves.into_iter(),
            dataset: Dataset::new()
        }
    }

    #[inline]
    pub fn dataset(self) -> Dataset {
        self.dataset
    }

}

impl Players for WTHORPlayers {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        let next = self.moves.next().unwrap();
        let position = Position::at((next / 10 - 1) as u32, (next % 10 - 1) as u32).unwrap();

        if match game.color() {
            DiscColor::Black => self.learn_black,
            DiscColor::White => self.learn_white
        } { self.dataset.push(AugmentedData::of(Data::new(game.board().player(), game.board().opponent(), position))); }

        Result::Ok(position)
    }
}
