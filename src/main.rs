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
    let factory = AIFactory::new("cuda")?;
    let mut ai = factory.create(&experiment)?;
    let mut players = PlayersMoves::new(&mut human, &mut ai);
    let (human, ai) = othello::processor::play(&mut players)?;
    println!("{human} and {ai}");

    println!("{}", match human.cmp(&ai) {
        Ordering::Greater => "Player Win",
        Ordering::Equal => "Draw",
        Ordering::Less => "AI Win"
    });

    Result::Ok(())
}
