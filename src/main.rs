mod ai;

use crate::ai::AI;
use std::cmp::Ordering;
use anyhow::Result;

use othello::game::{
    Players,
    Player,
    DefaultPlayers,
    HumanPlayer,
    RandomPlayer
};

fn main() -> Result<()> {
    let mut ai = AI::new(rand::thread_rng())?;
    let mut random = RandomPlayer::new(rand::thread_rng());
    println!("AI vs Random Player");
    run(&mut ai, &mut random, 10000)?;
    let mut human = HumanPlayer::new();
    println!("Human vs AI");
    println!();
    run(&mut human, &mut ai, 10)?;
    return Result::Ok(());
}

#[inline]
fn run<B: Player, W: Player>(black: &mut B, white: &mut W, n: usize) -> Result<()> {
    let mut players = DefaultPlayers::new(black, white);
    let mut black = 0;
    let mut white = 0;
    let mut draw = 0;

    for _ in 0..n {
        let count = players.run()?;

        match count.0.cmp(&count.1) {
            Ordering::Greater => black += 1,
            Ordering::Equal => draw += 1,
            Ordering::Less => white += 1
        }

    }

    println!("black: {}, white: {}, draw: {}", black, white, draw);

    println!("{}", match black.cmp(&white) {
        Ordering::Greater => "Black Win",
        Ordering::Equal => "Draw",
        Ordering::Less => "White Win"
    });

    println!();
    return Result::Ok(());
}

// ai vs random, black: 5815, white: 3795, draw: 390, depth: 8, epoch: 16
// ai vs random, black: 3767, white: 5841, draw: 392, depth: 8, epoch: 16