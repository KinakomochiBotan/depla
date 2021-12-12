use std::ops::{Shl, Shr};

#[derive(Clone, Copy, Debug)]
pub struct BoardData {
    data: u64
}

impl BoardData {

    #[inline]
    pub const fn new() -> Self {
        Self::new_from(0)
    }

    #[inline]
    pub const fn new_from(data: u64) -> Self {
        Self {
            data
        }
    }

    #[inline]
    pub const fn set(self, index: usize) -> Self {
        Self::new_from(self.data | (1 << index))
    }

    #[inline]
    pub const fn set_at(self, row: usize, column: usize) -> Self {
        self.set(8 * row + column)
    }

    #[inline]
    pub const fn is_set(self, row: usize, column: usize) -> bool {
        self.data & Self::new().set_at(row, column).data != 0
    }

    #[inline]
    pub fn count(self) -> u64 {
        let mut result = self.data;

        for (shifts, mask) in [
            (01, 0x5555555555555555),
            (02, 0x3333333333333333),
            (04, 0x0f0f0f0f0f0f0f0f),
            (08, 0x00ff00ff00ff00ff),
            (16, 0x0000ffff0000ffff),
            (32, 0x00000000ffffffff)
        ] {
            result = (result & mask) + (result >> shifts & mask);
        }

        return result;
    }

    #[inline]
    pub fn legal(self, opponent: Self) -> Self {
        let player = self.data;
        let opponent = opponent.data;
        let mut result = 0;

        #[inline]
        fn calc(player: u64, opponent: u64, shifts: u8, mask: u64, operator: fn(u64, u8) -> u64) -> u64 {
            let mask = opponent & mask;
            let mut result = mask & operator(player, shifts);
            (0..5).for_each(|_| result |= mask & operator(result, shifts));
            return operator(result, shifts);
        }

        for (shifts, mask) in [
            (1, 0x7e7e7e7e7e7e7e7e),
            (7, 0x007e7e7e7e7e7e00),
            (8, 0x00ffffffffffff00),
            (9, 0x007e7e7e7e7e7e00)
        ] {
            [Shl::shl, Shr::shr].into_iter().for_each(|operator| result |= calc(player, opponent, shifts, mask, operator))
        }

        return Self::new_from(result & !(player | opponent));
    }

    #[inline]
    pub fn put(self, opponent: Self, row: usize, column: usize) -> (Self, Self) {
        let player = self.data;
        let opponent = opponent.data;
        let position = Self::new().set_at(row, column).data;
        let mut reverse = 0;

        #[inline]
        fn calc(player: u64, opponent: u64, mut position: u64, shifts: u8, mask: u64, operator: fn(u64, u8) -> u64) -> u64 {
            let mut result = 0;

            loop {
                position = mask & operator(position, shifts);

                if opponent & position == 0 {
                    break;
                }

                result |= position;
            }

            return match player & position != 0 {
                true => result,
                false => 0
            }

        }

        for (shifts, mask) in [
            (1, 0xfefefefefefefefe),
            (7, 0x7f7f7f7f7f7f7f00),
            (8, 0xffffffffffffff00),
            (9, 0xfefefefefefefe00)
        ] {
            for (mask, operator) in [
                (mask, Shl::shl as fn(u64, u8) -> u64),
                (mask >> shifts, Shr::shr as fn(u64, u8) -> u64)
            ] {
                reverse |= calc(player, opponent, position, shifts, mask, operator);
            }
        }

        return (Self::new_from(player ^ position | reverse), Self::new_from(opponent ^ reverse));
    }

    #[inline]
    pub fn data(self) -> u64 {
        self.data
    }

}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    black: bool,
    player: BoardData,
    opponent: BoardData,
    legal: BoardData
}

impl Board {

    #[inline]
    pub fn new() -> Self {
        let black = BoardData::new().set_at(3, 4).set_at(4, 3);
        let white = BoardData::new().set_at(3, 3).set_at(4, 4);

        return Self {
            black: true,
            player: black,
            opponent: white,
            legal: black.legal(white)
        }

    }

    #[inline]
    fn update(&mut self, player: BoardData, opponent: BoardData) {
        self.player = player;
        self.opponent = opponent;
        self.legal = player.legal(opponent);
    }

    #[inline]
    pub fn put(&mut self, row: usize, column: usize) {
        let (player, opponent) = self.player.put(self.opponent, row, column);
        self.update(player, opponent);
    }

    #[inline]
    pub fn pass(&mut self) {
        self.black = !self.black;
        self.update(self.opponent, self.player);
    }

    #[inline]
    pub fn is_black_turn(&self) -> bool {
        self.black
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

    #[inline]
    pub fn black(&self) -> BoardData {
        match self.black {
            true => self.player,
            false => self.opponent
        }
    }

    #[inline]
    pub fn white(&self) -> BoardData {
        match self.black {
            true => self.opponent,
            false => self.player
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let a = BoardData::new().set(45).set(23).set(18);
        assert_eq!(a.data, 0x0000200000840000);
    }

    #[test]
    fn test_set_at() {
        let a = BoardData::new().set_at(4, 5).set_at(1, 3).set_at(0, 7);
        let b = BoardData::new().set(37).set(11).set(7);
        assert_eq!(a.data, b.data);
    }

    #[test]
    fn test_is_set() {
        let a = BoardData::new().set_at(6, 7).set_at(4, 3);
        assert!(a.is_set(4, 3));
        assert!(!a.is_set(1, 7));
    }

    #[test]
    fn test_count() {
        let a = BoardData::new().set_at(0, 0).set_at(3, 2).set_at(7, 3);
        assert_eq!(a.count(), 3);
    }

    #[test]
    fn test_legal() {
        let a = BoardData::new().set_at(3, 6);
        let b = BoardData::new().set_at(3, 3).set_at(3, 4).set_at(3, 5).set_at(4, 5);
        let c = BoardData::new().set_at(3, 2).set_at(5, 4);
        assert_eq!(a.legal(b).data, c.data);
    }

    #[test]
    fn test_put() {
        let a = BoardData::new().set_at(4, 4).set_at(4, 6);
        let b = BoardData::new().set_at(5, 5).set_at(6, 5);
        let (c, d) = a.put(b, 6, 6);
        let e = BoardData::new().set_at(4, 4).set_at(4, 6).set_at(5, 5).set_at(6, 6);
        let f = BoardData::new().set_at(6, 5);
        assert_eq!(c.data, e.data);
        assert_eq!(d.data, f.data);
    }

}
