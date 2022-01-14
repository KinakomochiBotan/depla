use super::AugmentedData;
use std::collections::HashSet;
use anyhow::Result;

use pyo3::{
    Python,
    IntoPy,
    IntoPyPointer,
    FromPyPointer,
    types::PyTuple,
    ffi::Py_ssize_t
};

use numpy::IntoPyArray;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dataset {
    data: HashSet<AugmentedData>
}

impl Dataset {

    #[inline]
    pub fn new() -> Self {
        Self { data: HashSet::new() }
    }

    #[inline]
    pub fn push(&mut self, data: AugmentedData) {
        self.data.insert(data);
    }

    #[inline]
    pub fn append(&mut self, other: Self) {
        self.data.extend(other.data);
    }

    #[inline]
    pub fn to(self, python: Python) -> Result<&PyTuple> {
        let result = unsafe { pyo3::ffi::PyTuple_New(8 * self.data.len() as Py_ssize_t) };
        let to_tensor = python.import("torch")?.getattr("from_numpy")?;

        for (index, (data, label)) in self.data.into_iter().flat_map(|data| data.to()).enumerate() {
            unsafe {
                let tuple = pyo3::ffi::PyTuple_New(2);
                pyo3::ffi::PyTuple_SetItem(tuple, 0, to_tensor.call1((data.into_pyarray(python),))?.into_ptr());
                pyo3::ffi::PyTuple_SetItem(tuple, 1, label.into_py(python).into_ptr());
                pyo3::ffi::PyTuple_SetItem(result, index as Py_ssize_t, tuple);
            }
        }

        return Result::Ok(unsafe { PyTuple::from_owned_ptr(python, result) });
    }

}
