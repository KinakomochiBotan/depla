mod io;
mod othello;
mod wthor;

use ::othello::game::{
    Players,
    player::{
        DefaultPlayers,
        HumanPlayer
    }
};

fn main() {
    let black = HumanPlayer::new();
    let white = HumanPlayer::new();
    let (row, column) = DefaultPlayers::new(black, white).run().unwrap();
    println!("{} vs {}", row, column);
}
