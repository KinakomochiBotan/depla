mod ai;

use crate::ai::AI;
use std::cmp::Ordering;
use anyhow::Result;

use othello::game::{
    Players,
    player::{
        DefaultPlayers,
        HumanPlayer,
        RandomPlayer
    }
};

fn main() -> Result<()> {
    let ai = AI::new(rand::thread_rng())?;
    let random = RandomPlayer::new(rand::thread_rng());
    let mut players = DefaultPlayers::new(ai, random);
    let mut black = 0;
    let mut draw = 0;
    let mut white = 0;

    for _ in 0..10000 {
        let count = players.run()?;

        match count.0.cmp(&count.1) {
            Ordering::Greater => black += 1,
            Ordering::Equal => draw += 1,
            Ordering::Less => white += 1
        }

    }

    println!("black: {}, draw: {}, white: {}", black, draw, white);
    let (ai, _) = players.to();
    let human = HumanPlayer::new();
    let mut players = DefaultPlayers::new(human, ai);
    players.run()?;
    return Result::Ok(());
}
