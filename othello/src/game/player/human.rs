use super::{
    super::Game,
    Player
};

use crate::Index;
use std::io::Write;
use anyhow::Result;

#[derive(Clone, Debug)]
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
    fn get_move(&mut self, game: &Game) -> Result<Index> {
        println!("{}", game);
        println!();

        loop {
            print!("please input an index: ");
            std::io::stdout().flush()?;
            self.buffer.clear();
            std::io::stdin().read_line(&mut self.buffer)?;
            let input = self.buffer.trim();

            if input.len() != 2 {
                continue;
            }

            let mut input = input.chars();

            let column = match input.next() {
                Option::Some(column) => (column as usize).overflowing_sub('a' as usize).0,
                Option::None => continue
            };

            let row = match input.next() {
                Option::Some(row) => (row as usize).overflowing_sub('1' as usize).0,
                Option::None => continue
            };

            let index = match Index::at(row, column) {
                Result::Ok(index) => index,
                Result::Err(_) => continue
            };

            if !game.board.legal().is_set(index) {
                continue;
            }

            println!();
            return Result::Ok(index);
        }

    }
}
