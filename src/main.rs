mod ai;

use crate::ai::AIFactory;
use std::cmp::Ordering;
use anyhow::Result;

use othello::players::{
    Players,
    Player,
    DefaultPlayers,
    HumanPlayer,
    RandomPlayer
};

fn main() -> Result<()> {
    let factory = AIFactory::new("cuda")?;
    let mut ai = factory.create(3, Option::None, rand::thread_rng())?;
    let mut random = RandomPlayer::new(rand::thread_rng());
    println!("AI vs Random Player");
    run(&mut ai, &mut random, 10000)?;
    let mut human = HumanPlayer::new();
    println!("Human vs AI");
    println!();
    run(&mut ai, &mut human, 1)?;
    Result::Ok(())
}

#[inline]
fn run<B: Player, W: Player>(black: &mut B, white: &mut W, n: usize) -> Result<()> {
    let mut players = DefaultPlayers::new(black, white);
    let mut black = 0;
    let mut white = 0;
    let mut draw = 0;

    for _ in 0..n {
        let count = players.play()?;

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
    Result::Ok(())
}
