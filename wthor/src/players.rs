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
    augmentation: bool,
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
            augmentation: option.load_augmentation(),
            moves: moves.into_iter(),
            dataset: Dataset::new(option.load_unique())
        }

    }

    #[inline]
    fn add(&mut self, data: Data) {
        match self.augmentation {
            true => {
                let mut data = [data; 8];
                data[4] = data[4].flip_diagonal();
                data[5] = data[4];
                data[6] = data[4];
                data[7] = data[4];
                data[2] = data[2].rotate180();
                data[3] = data[2];
                data[6] = data[6].rotate180();
                data[7] = data[6];
                data[1] = data[1].flip_vertical();
                data[3] = data[3].flip_vertical();
                data[5] = data[5].flip_vertical();
                data[7] = data[7].flip_vertical();
                self.dataset.extend(data);
            },
            false => self.dataset.push(data)
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
        } { self.add(Data::new(game.board().player(), game.board().opponent(), position)); }

        Result::Ok(position)
    }
}
