mod option;

pub use self::option::*;

use crate::data::{
    Data,
    Dataset
};

use std::{
    cmp::Ordering,
    array::IntoIter
};

use anyhow::Result;

use othello::{
    api::{
        Position,
        Color,
        Game
    },
    processor::Moves
};

#[derive(Clone, Debug)]
pub struct WTHORMoves {
    dataset: Dataset,
    learn_black: bool,
    learn_white: bool,
    moves: IntoIter<u8, 60>,
}

impl WTHORMoves {

    #[inline]
    pub fn new(dataset: Dataset, option: LoadOption, discs: u8, moves: [u8; 60]) -> Self {

        let (learn_black, learn_white) = match discs.cmp(&32) {
            Ordering::Greater => (option.win(), option.lose()),
            Ordering::Equal => (option.draw(), option.draw()),
            Ordering::Less => (option.lose(), option.win())
        };

        Self {
            dataset,
            learn_black,
            learn_white,
            moves: moves.into_iter(),
        }

    }

    #[inline]
    pub fn dataset(self) -> Dataset {
        self.dataset
    }

}

impl Moves for WTHORMoves {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        let next = self.moves.next()?;
        let position = Position::at((next / 10 - 1) as u32, (next % 10 - 1) as u32)?;

        if match game.color() {
            Color::Black => self.learn_black,
            Color::White => self.learn_white
        } {
            self.dataset.push(Data::new(game.board().player(), game.board().opponent(), position));
        }

        Result::Ok(position)
    }
}
