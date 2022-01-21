mod python;

pub use self::python::*;

use anyhow::Result;

use othello::{
    api::{
        Position,
        Game
    },
    processor::Moves
};

use pyo3::{
    Python,
    PyResult,
    Py,
    PyAny
};

pub struct AI {
    guess: Py<PyAny>
}

impl AI {

    #[inline]
    pub fn new(python: Python, cnn: Py<PyAny>) -> PyResult<Self> {
        PyResult::Ok(Self::of(cnn.getattr(python, "guess")?))
    }

    #[inline]
    pub fn of(guess: Py<PyAny>) -> Self {
        Self { guess }
    }

}

impl Moves for AI {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        Python::with_gil(|python| PyAI::of(python, self.guess.as_ref(python)).get_move(game))
    }
}
