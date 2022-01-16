use super::Data;

use std::{
    vec::IntoIter as VecIterator,
    collections::hash_set::{
        HashSet,
        IntoIter as SetIterator
    }
};

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

#[derive(Clone, Debug)]
enum Container {
    Origin(Vec<Data>),
    Unique(HashSet<Data>)
}

impl Container {

    #[inline]
    fn new(unique: bool) -> Self {
        match unique {
            true => Self::Unique(HashSet::new()),
            false => Self::Origin(Vec::new())
        }
    }

    #[inline]
    fn push(&mut self, data: Data) {
        match self {
            Self::Origin(vec) => vec.push(data),
            Self::Unique(set) => { set.insert(data); }
        }
    }

    #[inline]
    fn extend<const N: usize>(&mut self, data: [Data; N]) {
        match self {
            Self::Origin(vec) => vec.extend(data),
            Self::Unique(set) => set.extend(data)
        }
    }

    #[inline]
    fn append(&mut self, other: Self) {
        match self {
            Self::Origin(vec) => match other {
                Self::Origin(mut other) => vec.append(&mut other),
                Self::Unique(other) => vec.extend(other)
            },
            Self::Unique(set) => match other {
                Self::Origin(other) => set.extend(other),
                Self::Unique(other) => set.extend(other)
            }
        }
    }

    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::Origin(vec) => vec.len(),
            Self::Unique(set) => set.len()
        }
    }

    #[inline]
    fn to(self, python: Python) -> Result<&PyTuple> {
        let result = unsafe { pyo3::ffi::PyTuple_New(self.len() as Py_ssize_t) };
        let to_tensor = python.import("torch")?.getattr("from_numpy")?;

        for (index, (data, label)) in self.into_iter().map(|data| data.to()).enumerate() {
            unsafe {
                let tuple = pyo3::ffi::PyTuple_New(2);
                pyo3::ffi::PyTuple_SetItem(tuple, 0, to_tensor.call1((data.into_pyarray(python),))?.into_ptr());
                pyo3::ffi::PyTuple_SetItem(tuple, 1, label.into_py(python).into_ptr());
                pyo3::ffi::PyTuple_SetItem(result, index as Py_ssize_t, tuple);
            }
        }

        Result::Ok(unsafe { PyTuple::from_owned_ptr(python, result) })
    }

}

impl IntoIterator for Container {
    type Item = Data;
    type IntoIter = ContainerIterator;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Origin(vec) => Self::IntoIter::Origin(vec.into_iter()),
            Self::Unique(set) => Self::IntoIter::Unique(set.into_iter())
        }
    }

}

enum ContainerIterator {
    Origin(VecIterator<Data>),
    Unique(SetIterator<Data>)
}

impl Iterator for ContainerIterator {
    type Item = Data;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Origin(vec) => vec.next(),
            Self::Unique(set) => set.next()
        }
    }

}

#[derive(Clone, Debug)]
pub struct Dataset {
    container: Container
}

impl Dataset {

    #[inline]
    pub fn new(unique: bool) -> Self {
        Self { container: Container::new(unique) }
    }

    #[inline]
    pub fn push(&mut self, data: Data) {
        self.container.push(data);
    }

    #[inline]
    pub fn extend<const N: usize>(&mut self, data: [Data; N]) {
        self.container.extend(data);
    }

    #[inline]
    pub fn append(&mut self, other: Self) {
        self.container.append(other.container)
    }

    #[inline]
    pub fn to(self, python: Python) -> Result<&PyTuple> {
        self.container.to(python)
    }

}
