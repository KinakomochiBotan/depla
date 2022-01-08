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
    moves: IntoIter<u8, 60>,
    learn_black: bool,
    learn_white: bool,
    boards: Vec<TrainData>
}

impl WTHORPlayers {

    #[inline]
    pub fn new(discs: u8, moves: [u8; 60]) -> Self {
        Self {
            moves: moves.into_iter(),
            learn_black: discs >= 32,
            learn_white: discs <= 32,
            boards: Vec::new()
        }
    }

    #[inline]
    fn push(&mut self, player: Data, opponent: Data, position: Data) {
        self.boards.push(TrainData::new(player, opponent, position));
    }

    #[inline]
    fn push2(&mut self, player: Data, opponent: Data, position: Data) {
        self.push(player, opponent, position);
        self.push(flip_vertical(player), flip_vertical(opponent), flip_vertical(position));
    }

    #[inline]
    fn push4(&mut self, player: Data, opponent: Data, position: Data) {
        self.push2(player, opponent, position);
        self.push2(rotate180(player), rotate180(opponent), rotate180(position));
    }

    #[inline]
    fn push8(&mut self, player: Data, opponent: Data, position: Data) {
        self.push4(player, opponent, position);
        self.push4(flip_diagonal(player), flip_diagonal(opponent), flip_diagonal(position));
    }

    #[inline]
    pub fn boards(self) -> Vec<TrainData> {
        self.boards
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
            self.push8(game.board().player(), game.board().opponent(), Data::of(0).set(index));
        }

        return Result::Ok(index);
    }
}

#[inline]
fn flip_vertical(data: Data) -> Data {
    Data::of(data.value().swap_bytes())
}

#[inline]
fn rotate180(data: Data) -> Data {
    Data::of(data.value().reverse_bits())
}

#[inline]
fn flip_diagonal(data: Data) -> Data {
    let mut result = data.value();

    for (n, mask) in [
        (28, 0x0f0f0f0f00000000),
        (14, 0x3333000033330000),
        (07, 0x5500550055005500)
    ] {
        let mask = mask & (result ^ (result << n));
        result ^= mask ^ (mask >> n);
    }

    return Data::of(result);
}
