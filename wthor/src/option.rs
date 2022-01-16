use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

use pyo3::{
    PyResult,
    PyRefMut,
    types::PyModule
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[pyo3::pyclass]
pub struct LoadOption {
    unique: bool,
    win: bool,
    draw: bool,
    lose: bool
}

impl LoadOption {

    #[inline]
    pub fn check_valid(&self) -> bool {
        self.win || self.draw || self.lose
    }

    #[inline]
    pub fn load_unique(&self) -> bool {
        self.unique
    }

    #[inline]
    pub fn load_win(&self) -> bool {
        self.win
    }

    #[inline]
    pub fn load_draw(&self) -> bool {
        self.draw
    }

    #[inline]
    pub fn load_lose(&self) -> bool {
        self.lose
    }

}

#[pyo3::pymethods]
impl LoadOption {

    #[new]
    fn new() -> Self {
        Self {
            unique: false,
            win: false,
            draw: false,
            lose: false
        }
    }

    fn unique(mut this: PyRefMut<Self>) -> PyRefMut<Self> {
        this.unique = true;
        this
    }

    fn win(mut this: PyRefMut<Self>) -> PyRefMut<Self> {
        this.win = true;
        this
    }

    fn draw(mut this: PyRefMut<Self>) -> PyRefMut<Self> {
        this.draw = true;
        this
    }

    fn lose(mut this: PyRefMut<Self>) -> PyRefMut<Self> {
        this.lose = true;
        this
    }

}

impl Display for LoadOption {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "(unique: {}, win: {}, draw: {}, lose: {})", self.unique, self.win, self.draw, self.lose)
    }
}

#[inline]
pub fn add(module: &PyModule) -> PyResult<()> {
    module.add_class::<LoadOption>()
}
