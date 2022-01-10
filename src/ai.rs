use anyhow::Result;

use ndarray::Array2;
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
    PyAny,
    types::PyModule
};

use numpy::{
    PyArray2,
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
            Result::Ok(Py::from(PyModule::from_code(python, include_str!("airs.py"), "airs.py", "airs")?.getattr("get_ai")?.call0()?))
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
        let player_data = game.board().player();
        let opponent_data = game.board().opponent();
        let mut player = Array2::from_elem((8, 8), 0.0f32);
        let mut opponent = Array2::from_elem((8, 8), 0.0f32);

        for row in 0..8 {
            for column in 0..8 {
                let index = Index::at(row, column).unwrap();

                if player_data.is_set(index) {
                    player[(row, column)] = 1.0;
                }

                if opponent_data.is_set(index) {
                    opponent[(row, column)] = 1.0;
                }

            }
        }

        #[inline]
        fn get_output(python: Python, ai: &Py<PyAny>, player: Array2<f32>, opponent: Array2<f32>) -> Result<Array2<f32>>{
            Result::Ok(ai.call_method1(python, "guess", (player.into_pyarray(python), opponent.into_pyarray(python)))?.extract::<&PyArray2<f32>>(python)?.to_owned_array())
        }

        let output: Array2<f32> = Python::with_gil(|python| get_output(python, &self.ai, player, opponent))?;
        self.buffer.clear();
        let legal = game.board().legal();
        let mut min = f32::NEG_INFINITY;

        for row in 0..8 {
            for column in 0..8 {
                let index = Index::at(row, column).unwrap();

                if legal.is_set(index) {
                    let value = output[(row, column)];

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
