use crate::{
    Index,
    Data
};

use std::{
    ops::{
        Shl,
        Shr,
    },
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    }
};

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Board {
    player: Data,
    opponent: Data,
    legal: Data
}

impl Board {

    #[inline]
    pub fn of(player: Data, opponent: Data) -> Self {
        let mut legal = 0;

        for (n, mask) in [
            (1, 0x7e7e7e7e7e7e7e7e),
            (7, 0x007e7e7e7e7e7e00),
            (8, 0x00ffffffffffff00),
            (9, 0x007e7e7e7e7e7e00)
        ] {
            for shift in [Shl::shl, Shr::shr] {
                let mask = mask & opponent.value();
                let mut result = mask & shift(player.value(), n);

                loop {
                    let before = result;
                    result |= mask & shift(result, n);

                    if result == before {
                        break;
                    }

                }

                legal |= shift(result, n);
            }
        }

        return Self {
            player,
            opponent,
            legal: Data::of(legal & !(player.value() | opponent.value()))
        };

    }

    #[inline]
    pub fn put(&mut self, index: Index) -> Result<()> {

        if !self.legal.is_set(index) {
            return Result::Err(anyhow::anyhow!("cannot put a disc at {}", index));
        }

        let player = self.player.value();
        let opponent = self.opponent.value();
        let position = Data::of(0).set(index).value();
        let mut reverse = 0;

        for (n, mask) in [
            (1, 0xfefefefefefefefe),
            (7, 0x7f7f7f7f7f7f7f00),
            (8, 0xffffffffffffff00),
            (9, 0xfefefefefefefe00)
        ] {
            for (mask, shift) in [
                (mask, Shl::shl as fn(u64, u8) -> u64),
                (mask >> n, Shr::shr as fn(u64, u8) -> u64)
            ] {
                let mut position = position;
                let mut result = 0;

                loop {
                    position = mask & shift(position, n);

                    match opponent & position == 0 {
                        true => break,
                        false => result |= position
                    }

                }

                if player & position != 0 {
                    reverse |= result;
                }

            }
        }

        *self = Self::of(Data::of(opponent ^ reverse), Data::of(player ^ position | reverse));
        return Result::Ok(());
    }

    #[inline]
    pub fn pass(&mut self) {
        *self = Self::of(self.opponent, self.player);
    }

    #[inline]
    pub fn player(&self) -> Data {
        self.player
    }

    #[inline]
    pub fn opponent(&self) -> Data {
        self.opponent
    }

    #[inline]
    pub fn legal(&self) -> Data {
        self.legal
    }

}

impl Display for Board {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.player, self.opponent)
    }
}
