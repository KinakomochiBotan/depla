mod index;
mod data;

use self::{
    index::BoardIndex,
    data::BoardData
};

use std::ops::{
    BitOr,
    Shl,
    Shr
};

use anyhow::Result;

use itertools::{
    Itertools,
    FoldWhile
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Board {
    player: BoardData,
    opponent: BoardData,
    legal: BoardData
}

impl Board {

    #[inline]
    pub fn new() -> Self {
        Self::of(
            BoardData::new().set(BoardIndex::at(3, 4).unwrap()).set(BoardIndex::at(4, 3).unwrap()),
            BoardData::new().set(BoardIndex::at(3, 3).unwrap()).set(BoardIndex::at(4, 4).unwrap())
        )
    }

    #[inline]
    fn of(player: BoardData, opponent: BoardData) -> Self {
        let player_value = player.value();
        let opponent_value = opponent.value();

        return Self {
            player,
            opponent,
            legal: BoardData::of([
                (1, 0x7e7e7e7e7e7e7e7e),
                (7, 0x007e7e7e7e7e7e00),
                (8, 0x00ffffffffffff00),
                (9, 0x007e7e7e7e7e7e00)
            ].into_iter().cartesian_product([
                Shl::shl,
                Shr::shr
            ]).map(|((n, mask), shift)| (n, opponent_value & mask, shift)).map(|(n, mask, shift)| std::iter::repeat(()).fold_while(mask & shift(player_value, n), |value, _| {
                let next = value | (mask & shift(value, n));

                match value == next {
                    true => FoldWhile::Done(shift(value, n)),
                    false => FoldWhile::Continue(next)
                }

            }).into_inner()).reduce(u64::bitor).unwrap() & !(player_value | opponent_value))
        };

    }

    #[inline]
    pub fn put(&mut self, index: BoardIndex) -> Result<()> {

        if !self.legal.is_set(index) {
            return Result::Err(anyhow::anyhow!("cannot put a disc at {}", index));
        }

        let player = self.player.value();
        let opponent = self.opponent.value();
        let position = BoardData::new().set(index).value();

        let reverse = [
            (1, 0xfefefefefefefefe),
            (7, 0x7f7f7f7f7f7f7f00),
            (8, 0xffffffffffffff00),
            (9, 0xfefefefefefefe00)
        ].into_iter().flat_map(|(n, mask)| [
            (n, mask, Shl::shl as fn(u64, u8) -> u64),
            (n, mask >> n, Shr::shr as fn(u64, u8) -> u64)
        ]).map(|(n, mask, shift)| std::iter::repeat(()).fold_while((position, 0), |(position, reverse), _| {
            let next = mask & shift(position, n);

            match opponent & next == 0 {
                true => FoldWhile::Done((next, reverse)),
                false => FoldWhile::Continue((next, reverse | next))
            }

        }).into_inner()).map(|result| match player & result.0 != 0 {
            true => result.1,
            false => 0
        }).reduce(u64::bitor).unwrap();

        *self = Self::of(BoardData::of(player ^ position | reverse), BoardData::of(opponent ^ reverse));
        return Result::Ok(());
    }

    #[inline]
    pub fn pass(&mut self) {
        *self = Self::of(self.opponent, self.player);
    }

    #[inline]
    pub fn player(&self) -> BoardData {
        self.player
    }

    #[inline]
    pub fn opponent(&self) -> BoardData {
        self.opponent
    }

    #[inline]
    pub fn legal(&self) -> BoardData {
        self.legal
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal() {
        let index1 = BoardIndex::at(3, 6).unwrap();
        let data1 = BoardData::new().set(index1);
        let index2 = BoardIndex::at(3, 3).unwrap();
        let index3 = BoardIndex::at(3, 4).unwrap();
        let index4 = BoardIndex::at(3, 5).unwrap();
        let index5 = BoardIndex::at(4, 5).unwrap();
        let data2 = BoardData::new().set(index2).set(index3).set(index4).set(index5);
        let board = Board::of(data1, data2);
        let index6 = BoardIndex::at(3, 2).unwrap();
        let index7 = BoardIndex::at(5, 4).unwrap();
        let data3 = BoardData::new().set(index6).set(index7);
        assert_eq!(board.legal, data3);
    }

    #[test]
    fn test_put() {
        let index1 = BoardIndex::at(4, 4).unwrap();
        let index2 = BoardIndex::at(4, 6).unwrap();
        let data1 = BoardData::new().set(index1).set(index2);
        let index3 = BoardIndex::at(5, 5).unwrap();
        let index4 = BoardIndex::at(6, 5).unwrap();
        let data2 = BoardData::new().set(index3).set(index4);
        let mut board = Board::of(data1, data2);
        let index5 = BoardIndex::at(6, 6).unwrap();
        board.put(index5);
        let index6 = BoardIndex::at(4, 4).unwrap();
        let index7 = BoardIndex::at(4, 6).unwrap();
        let index8 = BoardIndex::at(5, 5).unwrap();
        let index9 = BoardIndex::at(6, 6).unwrap();
        let data3 = BoardData::new().set(index6).set(index7).set(index8).set(index9);
        let index10 = BoardIndex::at(6, 5).unwrap();
        let data4 = BoardData::new().set(index10);
        assert_eq!(board.player, data3);
        assert_eq!(board.opponent, data4);
    }

}
