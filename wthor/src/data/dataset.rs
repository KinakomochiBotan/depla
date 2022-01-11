use crate::parser::Dataset as ParserDataset;
use anyhow::Result;

use pyo3::{
    Python,
    IntoPyPointer,
    FromPyPointer,
    types::PyTuple,
    ffi::Py_ssize_t
};

use numpy::IntoPyArray;

pub struct Dataset {
    data: Vec<ParserDataset>,
    length: usize
}

impl Dataset {

    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            length: 0
        }
    }

    #[inline]
    pub fn push(&mut self, data: ParserDataset) {
        self.length += data.len();
        self.data.push(data);
    }

    #[inline]
    pub fn to(self, python: Python) -> Result<&PyTuple> {

        let result = unsafe {
            pyo3::ffi::PyTuple_New(self.length as Py_ssize_t)
        };

        let to_tensor = python.import("torch")?.getattr("from_numpy")?;

        for (index, data) in self.data.into_iter().flat_map(|vec| vec.data().into_iter().flat_map(|vec| vec.data().into_iter())).enumerate() {

            let tuple = unsafe {
                pyo3::ffi::PyTuple_New(2)
            };

            let (data, label) = data.to();

            unsafe {
                pyo3::ffi::PyTuple_SetItem(tuple, 0, to_tensor.call1((data.into_pyarray(python),))?.into_ptr());
                pyo3::ffi::PyTuple_SetItem(tuple, 1, to_tensor.call1((label.into_pyarray(python),))?.into_ptr());
                pyo3::ffi::PyTuple_SetItem(result, index as Py_ssize_t, tuple);
            }

        }

        return Result::Ok(unsafe {
            PyTuple::from_owned_ptr(python, result)
        });

    }

}
