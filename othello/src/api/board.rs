use super::{
    Position,
    Data
};

use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

use anyhow::{
    Result,
    ensure
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Board {
    player: Data,
    opponent: Data,
    legal: Data
}

impl Board {

    #[inline]
    pub fn of(player: Data, opponent: Data) -> Result<Self> {
        ensure!(player.value() & opponent.value() == 0, "a player and an opponent can't put a disc in the same place, but they were {player} and {opponent}");
        Result::Ok(unsafe { Self::of_unchecked(player, opponent) })
    }

    #[inline]
    pub const unsafe fn of_unchecked(player: Data, opponent: Data) -> Self {
        Self {
            player,
            opponent,
            legal: Data::of(crate::bit::legal((player.value(), opponent.value())))
        }
    }

    #[inline]
    pub fn make_move(self, position: Position) -> Result<Self> {
        ensure!(self.legal.is_set(position), "cannot put a disc at {position}");
        let board = (self.player.value(), self.opponent.value());
        let position = position.into_data().value();
        let (player, opponent) = crate::bit::make_move(board, position, crate::bit::reverse(board, position));
        Result::Ok(unsafe { Self::of_unchecked(Data::of(opponent), Data::of(player)) })
    }

    #[inline]
    pub fn pass(self) -> Self {
        unsafe { Self::of_unchecked(self.opponent, self.player) }
    }

    #[inline]
    pub fn player(self) -> Data {
        self.player
    }

    #[inline]
    pub fn opponent(self) -> Data {
        self.opponent
    }

    #[inline]
    pub fn legal(self) -> Data {
        self.legal
    }

}

impl Display for Board {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.player, self.opponent)
    }
}

