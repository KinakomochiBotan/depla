mod io;
mod othello;
mod wthor;

use ::othello::game::{
    Players,
    player::{
        DefaultPlayers,
        HumanPlayer,
        RandomPlayer
    }
};

fn main() {
    let black = HumanPlayer::new();
    let white = RandomPlayer::new(rand::thread_rng());
    let (row, column) = DefaultPlayers::new(black, white).run().unwrap();
    println!("{} vs {}", row, column);
}
