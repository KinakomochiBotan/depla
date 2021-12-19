use std::path::Path;
use crate::io::FileReader;
use anyhow::Result;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

/*
pub fn read<P: AsRef<Path>, I: IntoIterator<Item = P>>(paths: I) {
    Runtime::new()?.block_on(async {
        let mut handles = Vec::new();

        for path in paths {
            handles.push(tokio::spawn(async {
                let mut reader = FileReader::new(path).await?;
                reader.seek(4).await?;
                let games = u32::from_le_bytes(reader.read().await?);
                reader.seek(4).await?;
                let size = u8::from_le_bytes(reader.read().await?);

                if size != 8 {
                    return Result::Err(anyhow::anyhow!("the board size must be 8, but it was {}", size));
                }

                reader.seek(3).await?;
                let mut buffer = Vec::new();

                for _ in 0..games {
                    reader.seek(6).await?;
                    let discs = u8::from_le_bytes(reader.read().await?);
                    reader.seek(1).await?;
                    let moves: [u8; 60] = reader.read().await?;
                    let boards = Vec::new();



                    buffer.push((discs, moves));
                }

                return Result::Ok(buffer);
            }));
        }

        for handle in handles {
            handle.await;
        }

    })
}
*/
