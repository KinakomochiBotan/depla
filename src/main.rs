mod io;
mod othello;
mod wthor;

use anyhow::Result;

fn main() -> Result<()> {
    let start = std::time::SystemTime::now();
    let result = crate::wthor::read((2010..=2020).map(|year| format!("wthor/WTH_{}.wtb", year)))?;
    let end = std::time::SystemTime::now();
    println!("{}", end.duration_since(start)?.as_secs_f64());
    return Result::Ok(());
}
