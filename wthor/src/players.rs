use crate::{
    data::{
        Data,
        Dataset
    },
    option::LoadOption
};

use std::{
    cmp::Ordering,
    array::IntoIter
};

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
    pub fn new(option: LoadOption, discs: u8, moves: [u8; 60]) -> Self {

        let (learn_black, learn_white) = match discs.cmp(&32) {
            Ordering::Greater => (option.load_win(), option.load_lose()),
            Ordering::Equal => (option.load_draw(), option.load_draw()),
            Ordering::Less => (option.load_lose(), option.load_win())
        };

        Self {
            learn_black,
            learn_white,
            moves: moves.into_iter(),
            dataset: Dataset::new(option.load_unique())
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
        } { self.dataset.push(Data::new(game.board().player(), game.board().opponent(), position)); }

        Result::Ok(position)
    }
}
