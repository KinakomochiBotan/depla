use crate::{
    io::FileReader,
    othello::{
        Board,
        BoardIndex,
        BoardData,
        PlayerType,
        Players
    }
};

use std::{
    array::IntoIter,
    path::Path
};

use anyhow::Result;
use itertools::Itertools;
use tokio::runtime::Runtime;

#[inline]
pub fn parse<P: 'static + AsRef<Path> + Send, I: IntoIterator<Item = P>>(paths: I) -> Result<(Vec<Vec<Vec<([f32; 64], [f32; 64], [f32; 64])>>>, Vec<(usize, usize, usize)>)> {
    Runtime::new()?.block_on(async {
        let handles = paths.into_iter().enumerate().map(|(index, path)| tokio::spawn(parse_file(path, index))).collect_vec();
        let mut boards = Vec::with_capacity(handles.len());
        let mut indexes = Vec::new();

        for handle in handles {
            let mut result = handle.await??;
            boards.push(result.0);
            indexes.append(&mut result.1);
        }

        Result::Ok((boards, indexes))
    })
}

#[inline]
async fn parse_file<P: AsRef<Path>>(path: P, year: usize) -> Result<(Vec<Vec<([f32; 64], [f32; 64], [f32; 64])>>, Vec<(usize, usize, usize)>)> {
    let mut reader = FileReader::new(path).await?;
    reader.seek(4).await?;
    let games = u32::from_le_bytes(reader.read().await?) as usize;
    reader.seek(4).await?;
    let size = u8::from_le_bytes(reader.read().await?);

    if size != 8 {
        return Result::Err(anyhow::anyhow!("the board size must be 8, but it was {}", size));
    }

    reader.seek(3).await?;
    let mut boards = Vec::with_capacity(games);
    let mut indexes = Vec::new();

    for game in 0..games {
        reader.seek(6).await?;
        let discs = u8::from_le_bytes(reader.read().await?);
        reader.seek(1).await?;
        let moves: [u8; 60] = reader.read().await?;
        let mut players = WTHORPlayers::new(discs, moves);
        crate::othello::run(&mut players)?;
        indexes.extend((0..players.boards.len()).map(|board| (year, game, board)));
        boards.push(players.boards);
    }

    return Result::Ok((boards, indexes));
}

struct WTHORPlayers {
    moves: IntoIter<u8, 60>,
    learn_black: bool,
    learn_white: bool,
    boards: Vec<([f32; 64], [f32; 64], [f32; 64])>
}

impl WTHORPlayers {
    #[inline]
    fn new(discs: u8, moves: [u8; 60]) -> Self {
        Self {
            moves: moves.into_iter(),
            learn_black: discs >= 32,
            learn_white: discs <= 32,
            boards: Vec::new()
        }
    }
}

impl WTHORPlayers {

    #[inline]
    fn push8(&mut self, player: BoardData, opponent: BoardData, position: BoardData) {
        self.push4(player, opponent, position);
        self.push4(flip_diagonal(player), flip_diagonal(opponent), flip_diagonal(position));
    }

    #[inline]
    fn push4(&mut self, player: BoardData, opponent: BoardData, position: BoardData) {
        self.push2(player, opponent, position);
        self.push2(rotate180(player), rotate180(opponent), rotate180(position));
    }

    #[inline]
    fn push2(&mut self, player: BoardData, opponent: BoardData, position: BoardData) {
        self.push(player, opponent, position);
        self.push(flip_vertical(player), flip_vertical(opponent), flip_vertical(position));
    }

    #[inline]
    fn push(&mut self, player: BoardData, opponent: BoardData, position: BoardData) {
        self.boards.push((to_array(player), to_array(opponent), to_array(position)))
    }

}

impl Players for WTHORPlayers {
    #[inline]
    fn get_move(&mut self, player_type: PlayerType, board: Board) -> BoardIndex {
        let next = self.moves.next().unwrap();
        let index = BoardIndex::at((next / 10 - 1) as usize, (next % 10 - 1) as usize).unwrap();

        if match player_type {
            PlayerType::Black => self.learn_black,
            PlayerType::White => self.learn_white
        } {
            self.push8(board.player(), board.opponent(), BoardData::new().set(index));
        }

        return index;
    }
}

#[inline]
fn to_array(data: BoardData) -> [f32; 64] {
    let mut result = [0.0; 64];
    (0..8).cartesian_product(0..8).map(|(row, column)| BoardIndex::at(row, column).unwrap()).filter(|index| data.is_set(*index)).map(|index| index.value()).for_each(|index| result[index] = 1.0);
    return result;
}

#[inline]
fn flip_vertical(data: BoardData) -> BoardData {
    BoardData::of(data.value().swap_bytes())
}

#[inline]
fn rotate180(data: BoardData) -> BoardData {
    BoardData::of(data.value().reverse_bits())
}

#[inline]
fn flip_diagonal(data: BoardData) -> BoardData {
    BoardData::of([
        (28, 0x0f0f0f0f00000000),
        (14, 0x3333000033330000),
        (07, 0x5500550055005500)
    ].into_iter().fold(data.value(), |value, (shifts, mask)| {
        let mask = mask & (value ^ (value << shifts));
        value ^ (mask ^ (mask >> shifts))
    }))
}

