mod dataset;

pub use self::dataset::Dataset;
use crate::data::Data;
use std::array::IntoIter;
use anyhow::Result;

use othello::{
    Index,
    Data as OthelloData,
    game::{
        PlayerType,
        Game,
        Players
    }
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
    fn get_move(&mut self, game: &Game) -> Result<Index> {
        let next = self.moves.next().unwrap();
        let index = Index::at((next / 10 - 1) as usize, (next % 10 - 1) as usize).unwrap();

        if match game.player_type() {
            PlayerType::Black => self.learn_black,
            PlayerType::White => self.learn_white
        } {
            self.dataset.push(Data::new(game.board().player(), game.board().opponent(), OthelloData::of(0).set(index)));
        }

        return Result::Ok(index);
    }
}
