mod data;
mod players;
mod io;
mod parser;

use pyo3::{
    Python,
    PyResult,
    PyAny,
    types::{
        PyModule
    }
};

#[pyo3::pyfunction]
fn parse(python: Python, path: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(python, async {
        let result = crate::parser::parse(path).await?;
        Result::Ok(result)
    })
}

#[pyo3::pymodule]
fn wthor(_python: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(parse, module)?)
}
