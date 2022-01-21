use std::io::Write;
use anyhow::Result;

use othello::{
    api::{
        Position,
        Game
    },
    processor::Moves
};

#[derive(Clone, Debug)]
pub struct HumanMoves {
    buffer: String
}

impl HumanMoves {
    #[inline]
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }
}

impl Moves for HumanMoves {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        println!("{game}");
        println!();

        loop {
            print!("Please input a position: ");
            std::io::stdout().flush()?;
            self.buffer.clear();
            std::io::stdin().read_line(&mut self.buffer)?;
            let input = self.buffer.trim();
            if input.len() != 2 { continue; }
            let mut input = input.chars();

            let column = match input.next() {
                Option::Some(column) => (column as u32).overflowing_sub('A' as u32).0,
                Option::None => continue
            };

            let row = match input.next() {
                Option::Some(row) => (row as u32).overflowing_sub('1' as u32).0,
                Option::None => continue
            };

            let position = match Position::at(row, column) {
                Result::Ok(position) => position,
                Result::Err(_) => continue
            };

            if !game.board().legal().is_set(position) { continue; }
            println!();
            return Result::Ok(position);
        }

    }
}
