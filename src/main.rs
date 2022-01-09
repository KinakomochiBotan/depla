mod io;
mod othello;
mod wthor;

use anyhow::Result;

fn main() -> Result<()> {
    let data = crate::wthor::parse((2010..=2020).map(|year| format!("wthor_data/WTH_{}.wtb", year)))?;
    println!("{}", data.1.len());
    return Result::Ok(());
}

// 5416528
// 5416528
// 5416528
// 5416528