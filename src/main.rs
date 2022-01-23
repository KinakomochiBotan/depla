mod human;
mod ai;

use crate::{
    human::HumanMoves,
    ai::AIFactory
};

use std::cmp::Ordering;

use anyhow::{
    Result,
    bail
};

use othello::processor::PlayersMoves;

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next();

    let experiment = match args.next() {
        Option::Some(experiment) => experiment,
        Option::None => bail!("please enter an experiment number")
    };

    let mut human = HumanMoves::new();
    let mut ai = AIFactory::new("cuda")?.create(&experiment)?;
    let result1 = othello::processor::play(&mut PlayersMoves::new(&mut human, &mut ai))?;
    let result2 = othello::processor::play(&mut PlayersMoves::new(&mut ai, &mut human))?;
    print(result1.0, result1.1);
    print(result2.1, result2.0);
    Result::Ok(())
}

#[inline]
fn print(human: u32, ai: u32) {
    println!("{}", match human.cmp(&ai) {
        Ordering::Greater => "Player Win",
        Ordering::Equal => "Draw",
        Ordering::Less => "AI Win"
    });
}
