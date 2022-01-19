mod bit;
mod data;
mod moves;
mod io;

use crate::{
    data::Dataset,
    moves::{
        WTHORMoves,
        LoadOption
    },
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

use pyo3::{
    Python,
    PyResult,
    types::{
        PyModule,
        PyTuple
    }
};

#[pyo3::pymodule]
fn wthor(_: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(load, module)?)
}

#[pyo3::pyfunction]
fn load(python: Python, paths: Vec<String>, unique: bool, augmentation: bool, win: bool, draw: bool, lose: bool) -> PyResult<&PyTuple> {
    let option = LoadOption::new(win, draw, lose);
    ensure!(option.check(), "with the setting {option}, nothing will be loaded");
    let runtime = Runtime::new()?;
    let dataset = Dataset::new(unique, augmentation);
    let (sender, receiver) = tokio::sync::mpsc::channel(paths.len());

    #[inline]
    async fn load<P: AsRef<Path>>(path: P, mut dataset: Dataset, option: LoadOption, sender: Sender<Result<Dataset>>) {
        sender.send({
            let mut reader = FileReader::new(path).await?;
            reader.seek(4).await?;
            let games = u32::from_le_bytes(reader.read().await?);
            reader.seek(4).await?;
            let size = u8::from_le_bytes(reader.read().await?);
            ensure!(size == 8, "the board size must be 8, but it was {size}");
            reader.seek(3).await?;

            for _ in 0..games {
                reader.seek(6).await?;
                let discs = u8::from_le_bytes(reader.read().await?);
                reader.seek(1).await?;
                let moves = reader.read().await?;
                let mut moves = WTHORMoves::new(dataset.child(), option, discs, moves);
                othello::processor::play(&mut moves)?;
                dataset.append(moves.dataset());
            }

            Result::Ok(dataset)
        }).await.unwrap()
    }

    for path in paths { runtime.spawn(load(path, dataset.child(), option, sender.clone())); }
    std::mem::drop(sender);

    #[inline]
    async fn collect(mut dataset: Dataset, mut receiver: Receiver<Result<Dataset>>) -> Result<Dataset> {
        while let Option::Some(other) = receiver.recv().await { dataset.append(other?); }
        Result::Ok(dataset)
    }

    Result::Ok(runtime.block_on(collect(dataset, receiver))?.into(python)?)
}
