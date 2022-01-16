use crate::{
    data::Dataset,
    option::LoadOption,
    players::WTHORPlayers,
    io::FileReader
};

use std::path::Path;

use anyhow::{
    Result,
    ensure
};

use tokio::{
    runtime::Runtime,
    sync::mpsc::{
        Sender,
        Receiver
    }
};

use othello::players::Players;

use pyo3::{
    Python,
    PyResult,
    types::{
        PyModule,
        PyTuple
    }
};

#[pyo3::pyfunction]
pub fn parse(python: Python, paths: Vec<String>, option: LoadOption) -> PyResult<&PyTuple> {

    #[inline]
    fn parse(option: LoadOption, paths: Vec<String>) -> Result<Dataset> {
        ensure!(option.check_valid(), "with the setting {}, nothing will be loaded", option);
        let runtime = Runtime::new()?;
        let (sender, mut receiver) = tokio::sync::mpsc::channel(paths.len());
        for path in paths { runtime.spawn(load(sender.clone(), path, option)); }
        std::mem::drop(sender);
        runtime.block_on(collect(&mut receiver, option.load_unique()))
    }

    Result::Ok(parse(option, paths)?.to(python)?)
}

#[inline]
async fn load<P: AsRef<Path>>(sender: Sender<Result<Dataset>>, path: P, option: LoadOption) {

    #[inline]
    async fn load<P: AsRef<Path>>(path: P, option: LoadOption) -> Result<Dataset> {
        let mut reader = FileReader::new(path).await?;
        reader.seek(4).await?;
        let games = u32::from_le_bytes(reader.read().await?);
        reader.seek(4).await?;
        let size = u8::from_le_bytes(reader.read().await?);
        ensure!(size == 8, "the board size must be 8, but it was {}", size);
        reader.seek(3).await?;
        let mut result = Dataset::new(option.load_unique());

        for _ in 0..games {
            reader.seek(6).await?;
            let discs = u8::from_le_bytes(reader.read().await?);
            reader.seek(1).await?;
            let moves = reader.read().await?;
            let mut players = WTHORPlayers::new(option, discs, moves);
            players.play()?;
            result.append(players.dataset());
        }

        Result::Ok(result)
    }

    sender.send(load(path, option).await).await.unwrap();
}

#[inline]
async fn collect(receiver: &mut Receiver<Result<Dataset>>, unique: bool) -> Result<Dataset> {
    let mut result = Dataset::new(unique);
    while let Option::Some(data) = receiver.recv().await { result.append(data?); }
    Result::Ok(result)
}

#[inline]
pub fn add(module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(parse, module)?)
}
