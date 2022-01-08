use crate::{
    data::TrainData,
    io::FileReader,
    players::WTHORPlayers
};

use std::path::Path;
use anyhow::Result;
use othello::game::Players;

#[inline]
pub async fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<TrainData>>> {
    let mut reader = FileReader::new(path).await?;
    reader.seek(4).await?;
    let games = u32::from_le_bytes(reader.read().await?) as usize;
    reader.seek(4).await?;
    let size = u8::from_le_bytes(reader.read().await?);

    if size != 8 {
        return Result::Err(anyhow::anyhow!("the board size must be 8, but it was {}", size));
    }

    reader.seek(3).await?;
    let mut boards = Vec::with_capacity(games);

    for _ in 0..games {
        reader.seek(6).await?;
        let discs = u8::from_le_bytes(reader.read().await?);
        reader.seek(1).await?;
        let moves = reader.read().await?;
        let mut players = WTHORPlayers::new(discs, moves);
        players.run()?;
        boards.push(players.boards());
    }

    return Result::Ok(boards);
}
