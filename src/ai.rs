use ai::AI;

use pyo3::{
    Python,
    PyResult,
    Py,
    PyAny
};

#[derive(Clone, Debug)]
pub struct AIFactory {
    device: Py<PyAny>,
    load: Py<PyAny>
}

impl AIFactory {

    #[inline]
    pub fn new(device: &str) -> PyResult<Self> {
        std::env::set_current_dir("run")?;

        #[inline]
        fn initialize(python: Python, device: &str) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
            python.import("sys")?.getattr("path")?.call_method1("append", (".",))?;
            let device = python.import("torch")?.call_method1("device", (device,))?;
            let load = python.import("depla")?.getattr("CNN")?.getattr("load")?;
            PyResult::Ok((Py::from(device), Py::from(load)))
        }

        let (device, load) = Python::with_gil(|python| initialize(python, device))?;
        PyResult::Ok(Self { device, load })
    }

    #[inline]
    pub fn create(&self, experiment: &str) -> PyResult<AI> {

        #[inline]
        fn load(python: Python, load: &Py<PyAny>, path: &str, device: &Py<PyAny>) -> PyResult<AI> {
            PyResult::Ok(AI::new(python, load.call1(python, (path, device))?)?)
        }

        Python::with_gil(|python| load(python, &self.load, &["result", &experiment.replace(".", "/"), "cnn.pt"].join("/"), &self.device))
    }

}
