use anyhow::{
    Result,
    Context as _
};

use ndarray::Array4;

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
    PyAny
};

use numpy::{
    PyArray2,
    IntoPyArray as _
};

pub struct PyAI<'py> {
    python: Python<'py>,
    guess: &'py PyAny
}

impl<'py> PyAI<'py> {

    #[inline]
    pub fn new(python: Python<'py>, cnn: &'py PyAny) -> PyResult<Self> {
        PyResult::Ok(Self::of(python, cnn.getattr("guess")?))
    }

    #[inline]
    pub fn of(python: Python<'py>, guess: &'py PyAny) -> Self {
        Self { python, guess }
    }

}

impl<'py> Moves for PyAI<'py> {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        let player = game.board().player();
        let opponent = game.board().opponent();
        let mut data = Array4::<f32>::zeros((1, 2, 8, 8));

        for position in Position::iter() {
            if player.is_set(position) { data[(0, 0, position.row() as usize, position.column() as usize)] = 1.0 }
            if opponent.is_set(position) { data[(0, 1, position.row() as usize, position.column() as usize)] = 1.0 }
        }

        let output = self.guess.call1((data.into_pyarray(self.python),))?.extract::<&PyArray2<f32>>()?.to_owned_array();
        let legal = game.board().legal();
        let mut result = Option::None;
        let mut max = f32::NEG_INFINITY;

        for position in Position::iter().filter(|position| legal.is_set(*position)) {
            let value = output[(0, position.value() as usize)];

            if value > max {
                result = Option::Some(position);
                max = value
            }

        }

        result.context("failed to guess")
    }
}
