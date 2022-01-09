use crate::data::TrainData;
use std::array::IntoIter;
use anyhow::Result;

use othello::{
    Index,
    Data,
    game::{
        PlayerType,
        Game,
        Players
    }
};

pub struct WTHORPlayers {
    learn_black: bool,
    learn_white: bool,
    moves: IntoIter<u8, 60>,
    result: Vec<TrainData>
}

impl WTHORPlayers {

    #[inline]
    pub fn new(discs: u8, moves: [u8; 60]) -> Self {
        Self {
            learn_black: discs >= 32,
            learn_white: discs <= 32,
            moves: moves.into_iter(),
            result: Vec::new()
        }
    }

    #[inline]
    fn push(&mut self, player: Data, opponent: Data, index: Index) {
        let mut push = |data: TrainData| self.result.push(data);

        let mut push2 = |data: TrainData| {
            push(data);
            push(data.flip_vertical());
        };

        let mut push4 = |data: TrainData| {
            push2(data);
            push2(data.rotate180());
        };

        let mut push8 = |data: TrainData| {
            push4(data);
            push4(data.flip_diagonal());
        };

        push8(TrainData::new(player, opponent, Data::of(0).set(index)));
    }

    #[inline]
    pub fn result(self) -> Vec<TrainData> {
        self.result
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
            self.push(game.board().player(), game.board().opponent(), index);
        }

        return Result::Ok(index);
    }
}
