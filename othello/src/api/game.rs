use super::{
    Position,
    Data,
    Board,
    Color
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
    color: Color
}

impl Game {

    const BOARD: Board = unsafe {
        Board::of_unchecked(
            Data::of(Position::at_unchecked(3, 4).into_data().value() | Position::at_unchecked(4, 3).into_data().value()),
            Data::of(Position::at_unchecked(3, 3).into_data().value() | Position::at_unchecked(4, 4).into_data().value())
        )
    };

    #[inline]
    pub const fn new() -> Self {
        Self {
            board: Self::BOARD,
            color: Color::Black
        }
    }

    #[inline]
    pub fn make_move(&mut self, position: Position) -> Result<()> {
        self.board = self.board.make_move(position)?;
        self.color = self.color.flip();
        Result::Ok(())
    }

    #[inline]
    pub fn pass(&mut self) {
        self.board = self.board.pass();
        self.color = self.color.flip();
    }

    #[inline]
    pub fn board(&self) -> Board {
        self.board
    }

    #[inline]
    pub fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn black(&self) -> Data {
        match self.color {
            Color::Black => self.board.player(),
            Color::White => self.board.opponent()
        }
    }

    #[inline]
    pub fn white(&self) -> Data {
        match self.color {
            Color::Black => self.board.opponent(),
            Color::White => self.board.player()
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

            if row != 7 { writeln!(f)?; }
        }

        Result::Ok(())
    }
}
