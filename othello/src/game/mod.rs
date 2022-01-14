mod position;
mod data;
mod board;
mod color;

pub use self::{
    position::*,
    data::*,
    board::*,
    color::*
};

use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Game {
    board: Board,
    color: DiscColor
}

impl Game {
    pub const BLACK: Data = unsafe { Data::of(0).set(Position::at_unchecked(3, 4)).set(Position::at_unchecked(4, 3)) };
    pub const WHITE: Data = unsafe { Data::of(0).set(Position::at_unchecked(3, 3)).set(Position::at_unchecked(4, 4)) };
    pub const BOARD: Board = unsafe { Board::of_unchecked(Self::BLACK, Self::WHITE) };

    #[inline]
    pub const fn new() -> Self {
        Self {
            board: Self::BOARD,
            color: DiscColor::Black
        }
    }

    #[inline]
    pub fn put(&mut self, position: Position) -> Result<()> {
        self.board.put(position)?;
        self.color.flip();
        Result::Ok(())
    }

    #[inline]
    pub fn pass(&mut self) {
        self.board.pass();
        self.color.flip();
    }

    #[inline]
    pub fn board(&self) -> Board {
        self.board
    }

    #[inline]
    pub fn color(&self) -> DiscColor {
        self.color
    }

    #[inline]
    pub fn black(&self) -> Data {
        match self.color {
            DiscColor::Black => self.board.player(),
            DiscColor::White => self.board.opponent()
        }
    }

    #[inline]
    pub fn white(&self) -> Data {
        match self.color {
            DiscColor::Black => self.board.opponent(),
            DiscColor::White => self.board.player()
        }
    }

}

impl Display for Game {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let black = self.black();
        let white = self.white();
        let legal = self.board.legal();
        writeln!(f, "   | a b c d e f g h")?;
        writeln!(f, "---|-----------------")?;

        for row in 0..8 {
            write!(f, " {} |", row + 1)?;

            for column in 0..8 {
                let position = unsafe { Position::at_unchecked(row, column) };

                write!(f, " {}", match (black.is_set(position), white.is_set(position), legal.is_set(position))  {
                    (true, _, _) => 'O',
                    (_, true, _) => 'X',
                    (_, _, true) => '*',
                    _ => ' '
                })?;

            }

            if row != 7 {
                writeln!(f)?;
            }

        }

        Result::Ok(())
    }
}
