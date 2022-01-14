mod io;
mod data;
mod players;
mod parser;

use crate::data::Dataset;
use anyhow::Result;

use tokio::{
    runtime::Runtime,
    sync::mpsc::Receiver
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
    module.add_function(pyo3::wrap_pyfunction!(parse, module)?)
}

#[pyo3::pyfunction]
fn parse(python: Python, paths: Vec<String>) -> PyResult<&PyTuple> {
    let runtime = Runtime::new()?;
    let (sender, receiver) = tokio::sync::mpsc::channel(paths.len());
    for path in paths { runtime.spawn(crate::parser::parse(path, sender.clone())); }
    std::mem::drop(sender);

    #[inline]
    async fn collect(mut receiver: Receiver<Result<Dataset>>) -> Result<Dataset> {
        let mut result = Dataset::new();
        while let Option::Some(data) = receiver.recv().await { result.append(data?); }
        Result::Ok(result)
    }

    Result::Ok(runtime.block_on(collect(receiver))?.to(python)?)
}
