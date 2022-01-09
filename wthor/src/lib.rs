mod io;
mod data;
mod players;
mod parser;

use tokio::runtime::Runtime;

use pyo3::{
    Python,
    PyResult,
    IntoPy,
    PyObject,
    types::{
        PyModule,
        PyList
    }
};

#[pyo3::pymodule]
fn wthor(_: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(parse, module)?)
}

#[pyo3::pyfunction]
fn parse(python: Python, paths: &PyList) -> PyResult<PyObject> {
    let runtime = Runtime::new()?;
    let (sender, mut receiver) = tokio::sync::mpsc::channel(paths.len());

    for path in paths {
        runtime.spawn(crate::parser::parse(path.extract::<String>()?, sender.clone()));
    }

    std::mem::drop(sender);

    return Result::Ok(runtime.block_on(async {
        let mut result = Vec::new();

        while let Option::Some(data) = receiver.recv().await {
            result.reserve(data.iter().map(|vec| vec.len()).sum());
            data.into_iter().for_each(|mut vec| result.append(&mut vec));
        }

        return result;
    }).into_py(python));

}
