use super::Player;

use crate::game::{
    Position,
    Game
};

use std::io::Write;
use anyhow::Result;

#[derive(Clone, Hash, Debug)]
pub struct HumanPlayer {
    buffer: String
}

impl HumanPlayer {
    #[inline]
    pub fn new() -> Self {
        Self {
            buffer: String::new()
        }
    }
}

impl Player for HumanPlayer {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        println!("{}", game);
        println!();

        loop {
            print!("please input a position: ");
            std::io::stdout().flush()?;
            self.buffer.clear();
            std::io::stdin().read_line(&mut self.buffer)?;
            let input = self.buffer.trim();
            if input.len() != 2 { continue; }
            let mut input = input.chars();

            let column = match input.next() {
                Option::Some(column) => (column as u32).overflowing_sub('a' as u32).0,
                Option::None => continue
            };

            let row = match input.next() {
                Option::Some(row) => (row as u32).overflowing_sub('1' as u32).0,
                Option::None => continue
            };

            let position = match Position::at(row, column) {
                Result::Ok(index) => index,
                Result::Err(_) => continue
            };

            if !game.board().legal().is_set(position) { continue; }
            println!();
            return Result::Ok(position);
        }

    }
}

