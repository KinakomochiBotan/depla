use crate::{
    io::FileReader,
    data::TrainData,
    players::WTHORPlayers
};

use std::path::Path;
use anyhow::Result;
use tokio::sync::mpsc::Sender;
use othello::game::Players;

#[inline]
pub async fn parse<P: AsRef<Path>>(path: P, sender: Sender<Vec<Vec<TrainData>>>) -> Result<()> {
    let mut reader = FileReader::new(path).await?;
    reader.seek(4).await?;
    let games = u32::from_le_bytes(reader.read().await?) as usize;
    reader.seek(4).await?;
    let size = u8::from_le_bytes(reader.read().await?);

    if size != 8 {
        return Result::Err(anyhow::anyhow!("the board size must be 8, but it was {}", size));
    }

    reader.seek(3).await?;
    let mut result = Vec::with_capacity(games);

    for _ in 0..games {
        reader.seek(6).await?;
        let discs = u8::from_le_bytes(reader.read().await?);
        reader.seek(1).await?;
        let moves = reader.read().await?;
        let mut players = WTHORPlayers::new(discs, moves);
        players.run()?;
        result.push(players.result());
    }

    sender.send(result).await?;
    return Result::Ok(());
}
