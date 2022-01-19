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
    FromPyPointer,
    IntoPyPointer,
    types::PyTuple,
    ffi::Py_ssize_t
};

use numpy::IntoPyArray;

#[derive(Clone, Debug)]
pub struct Dataset {
    collection: Collection,
    unique: bool,
    augmentation: bool
}

impl Dataset {

    #[inline]
    pub fn new(unique: bool, augmentation: bool) -> Self {
        Self {
            collection: match unique {
                true => Collection::Unique(HashSet::new()),
                false => Collection::Origin(Vec::new())
            },
            unique,
            augmentation
        }
    }

    #[inline]
    pub fn push(&mut self, data: Data) {
        match (&mut self.collection, self.augmentation) {
            (Collection::Origin(vec), true) => vec.extend(data.augment()),
            (Collection::Origin(vec), false) => vec.push(data),
            (Collection::Unique(set), true) => set.extend(data.augment()),
            (Collection::Unique(set), false) => set.insert(data)
        }
    }

    #[inline]
    pub fn append(&mut self, other: Self) {
        match (&mut self.collection, other.collection) {
            (Collection::Origin(vec), Collection::Origin(mut other)) => vec.append(&mut other),
            (Collection::Origin(vec), Collection::Unique(other)) => vec.extend(other),
            (Collection::Unique(set), Collection::Origin(other)) => set.extend(other),
            (Collection::Unique(set), Collection::Unique(other)) => set.extend(other)
        }
    }

    #[inline]
    pub fn child(&self) -> Self {
        Self::new(self.unique, self.augmentation)
    }

    #[inline]
    pub fn into(self, python: Python) -> Result<&PyTuple> {

        let result = unsafe {
            pyo3::ffi::PyTuple_New(match &self.collection {
                Collection::Origin(vec) => vec.len(),
                Collection::Unique(set) => set.len()
            } as Py_ssize_t)
        };

        let tensor = python.import("torch")?.getattr("from_numpy")?;

        for (index, (data, label)) in self.collection.into_iter().map(|data| data.into()).enumerate() {
            unsafe {
                let tuple = pyo3::ffi::PyTuple_New(2);
                pyo3::ffi::PyTuple_SetItem(tuple, 0, tensor.call1((data.into_pyarray(python),))?.into_ptr());
                pyo3::ffi::PyTuple_SetItem(tuple, 1, label.into_py(python).into_ptr());
                pyo3::ffi::PyTuple_SetItem(result, index as Py_ssize_t, tuple);
            }
        }

        Result::Ok(unsafe { PyTuple::from_owned_ptr(python, result) })
    }

}

enum Collection {
    Origin(Vec<Data>),
    Unique(HashSet<Data>)
}

impl IntoIterator for Collection {
    type Item = Data;
    type IntoIter = CollectionIterator;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Origin(vec) => Self::IntoIter::Origin(vec.into_iter()),
            Self::Unique(set) => Self::IntoIter::Unique(set.into_iter())
        }
    }

}

enum CollectionIterator {
    Origin(VecIterator<Data>),
    Unique(SetIterator<Data>)
}

impl Iterator for CollectionIterator {
    type Item = Data;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Origin(vec) => vec.next(),
            Self::Unique(set) => set.next()
        }
    }

}
