use crate::parser::Dataset as ParserDataset;

use pyo3::{
    Python,
    IntoPy,
    IntoPyPointer,
    Py,
    types::PyTuple,
    ffi::Py_ssize_t
};

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

}

impl IntoPy<Py<PyTuple>> for Dataset {
    #[inline]
    fn into_py(self, py: Python) -> Py<PyTuple> {

        let tuple = unsafe {
            pyo3::ffi::PyTuple_New(self.length as Py_ssize_t)
        };

        self.data.into_iter().flat_map(|vec| vec.data().into_iter().flat_map(|vec| vec.data().into_iter())).enumerate().for_each(|(index, data)| unsafe {
            pyo3::ffi::PyTuple_SetItem(tuple, index as Py_ssize_t, data.into_py(py).into_ptr());
        });

        return unsafe {
            Py::from_owned_ptr(py, tuple)
        };

    }
}
