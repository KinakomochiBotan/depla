use anyhow::Result;

use ndarray::{
    Array3,
    Array4
};

use rand::Rng;

use othello::{
    Index,
    game::{
        Game,
        player::Player
    }
};

use pyo3::{
    Python,
    Py,
    PyAny
};

use numpy::{
    PyArray3,
    IntoPyArray
};

#[derive(Debug)]
pub struct AI<R> {
    ai: Py<PyAny>,
    rng: R,
    buffer: Vec<Index>
}

impl<R> AI<R> {
    #[inline]
    pub fn new(rng: R) -> Result<Self> {

        #[inline]
        fn get_ai(python: Python) -> Result<Py<PyAny>> {
            python.import("sys")?.getattr("path")?.call_method1("append", ("run/python",))?;
            return Result::Ok(Py::from(python.import("ai")?.getattr("AI")?.call0()?));
        }

        Result::Ok(Self {
            ai: Python::with_gil(|python| get_ai(python))?,
            rng,
            buffer: Vec::new()
        })

    }
}

impl<R: Rng> Player for AI<R> {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Index> {
        let player = game.board().player();
        let opponent = game.board().opponent();
        let mut data = Array4::zeros((1, 2, 8, 8));

        for row in 0..8 {
            for column in 0..8 {
                let index = Index::at(row, column).unwrap();

                if player.is_set(index) {
                    data[(0, 0, row, column)] = 1.0;
                }

                if opponent.is_set(index) {
                    data[(0, 1, row, column)] = 1.0;
                }

            }
        }

        #[inline]
        fn get_output(python: Python, ai: &Py<PyAny>, data: Array4<f32>) -> Result<Array3<f32>> {
            Result::Ok(ai.call_method1(python, "guess", (data.into_pyarray(python),))?.extract::<&PyArray3<f32>>(python)?.to_owned_array())
        }

        let output: Array3<f32> = Python::with_gil(|python| get_output(python, &self.ai, data))?;
        self.buffer.clear();
        let legal = game.board().legal();
        let mut min = f32::NEG_INFINITY;

        for row in 0..8 {
            for column in 0..8 {
                let index = Index::at(row, column).unwrap();

                if legal.is_set(index) {
                    let value = output[(0, row, column)];

                    if value > min {
                        self.buffer.clear();
                        self.buffer.push(index);
                        min = value;
                    } else if value >= min {
                        self.buffer.push(index);
                    }

                }

            }
        }

        return Result::Ok(self.buffer[self.rng.gen_range(0..self.buffer.len())]);
    }
}
