pub mod player;

use crate::{
    Index,
    Data,
    Board
};

use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerType {
    Black,
    White
}

impl PlayerType {
    #[inline]
    fn next(&mut self) {
        *self = match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}

impl Display for PlayerType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            Self::Black => "Black",
            Self::White => "White"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Game {
    board: Board,
    player_type: PlayerType
}

impl Game {

    #[inline]
    pub fn new() -> Self {
        Self {
            board: Board::of(
                Data::of(0).set(Index::at(3, 4).unwrap()).set(Index::at(4, 3).unwrap()),
                Data::of(0).set(Index::at(3, 3).unwrap()).set(Index::at(4, 4).unwrap())
            ),
            player_type: PlayerType::Black
        }
    }

    #[inline]
    fn put(&mut self, index: Index) -> Result<()> {
        self.board.put(index)?;
        self.player_type.next();
        return Result::Ok(());
    }

    #[inline]
    fn pass(&mut self) {
        self.board.pass();
        self.player_type.next();
    }

    #[inline]
    pub fn board(&self) -> Board {
        self.board
    }

    #[inline]
    pub fn player_type(&self) -> PlayerType {
        self.player_type
    }

    #[inline]
    pub fn black(&self) -> Data {
        match self.player_type {
            PlayerType::Black => self.board.player(),
            PlayerType::White => self.board.opponent()
        }
    }

    #[inline]
    pub fn white(&self) -> Data {
        match self.player_type {
            PlayerType::Black => self.board.opponent(),
            PlayerType::White => self.board.player()
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
                let index = Index::at(row, column).unwrap();

                write!(f, " {}", match (black.is_set(index), white.is_set(index), legal.is_set(index))  {
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

        return Result::Ok(());
    }
}

pub trait Players {
    fn get_move(&mut self, game: &Game) -> Result<Index>;

    #[inline]
    fn run(&mut self) -> Result<(u32, u32)> where Self: Sized {
        let mut game = Game::new();
        let mut will_pass = false;

        loop {

            if game.board.legal().value() == 0 {
                if will_pass {
                    break;
                } else {
                    game.pass();
                    will_pass = true;
                    continue;
                }
            }

            game.put(self.get_move(&game)?)?;
            will_pass = false;
        }

        return Result::Ok((game.black().value().count_ones(), game.white().value().count_ones()));
    }

}
