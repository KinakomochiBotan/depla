mod data;
mod option;
mod players;
mod io;
mod parser;

use pyo3::{
    Python,
    PyResult,
    types::PyModule
};

#[pyo3::pymodule]
fn wthor(_: Python, module: &PyModule) -> PyResult<()> {
    crate::option::add(module)?;
    crate::parser::add(module)?;
    Result::Ok(())
}