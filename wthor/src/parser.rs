use crate::{
    data::Dataset,
    players::WTHORPlayers,
    io::FileReader
};

use std::path::Path;
use anyhow::Result;
use tokio::sync::mpsc::Sender;
use othello::players::Players;

#[inline]
pub async fn parse<P: AsRef<Path>>(path: P, sender: Sender<Result<Dataset>>) {

    #[inline]
    async fn parse<P: AsRef<Path>>(path: P) -> Result<Dataset> {
        let mut reader = FileReader::new(path).await?;
        reader.seek(4).await?;
        let games = u32::from_le_bytes(reader.read().await?);
        reader.seek(4).await?;
        let size = u8::from_le_bytes(reader.read().await?);
        anyhow::ensure!(size == 8, "the board size must be 8, but it was {}", size);
        reader.seek(3).await?;
        let mut result = Dataset::new();

        for _ in 0..games {
            reader.seek(6).await?;
            let discs = u8::from_le_bytes(reader.read().await?);
            reader.seek(1).await?;
            let moves = reader.read().await?;
            let mut players = WTHORPlayers::new(discs, moves);
            players.play()?;
            result.append(players.dataset());
        }

        Result::Ok(result)
    }

    sender.send(parse(path).await).await.unwrap();
}
