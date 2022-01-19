use std::path::PathBuf;

use anyhow::{
    Result,
    anyhow
};

use ndarray::{
    Array2,
    Array4
};

use rand::{
    Rng,
    seq::SliceRandom
};

use othello::{
    api::{
        Position,
        Game
    },
    players::Player
};

use pyo3::{
    Python,
    Py,
    PyAny
};

use numpy::{
    PyArray2,
    IntoPyArray
};

#[derive(Clone, Debug)]
pub struct AIFactory {
    device: Py<PyAny>,
    load: Py<PyAny>
}

impl AIFactory {

    #[inline]
    pub fn new(device: &str) -> Result<Self> {
        std::env::set_current_dir("run")?;

        #[inline]
        fn initialize(python: Python, device: &str) -> Result<(Py<PyAny>, Py<PyAny>)> {
            python.import("sys")?.getattr("path")?.call_method1("append", (".",))?;
            let device = python.import("torch")?.call_method1("device", (device,))?;
            let load = python.import("depla")?.getattr("CNN")?.getattr("load")?;
            Result::Ok((Py::from(device), Py::from(load)))
        }

        let (device, load) = Python::with_gil(|python| initialize(python, device))?;

        Result::Ok(Self {
            device,
            load
        })

    }

    #[inline]
    pub fn create<R: Rng>(&self, experiment: u32, sub_experiment: Option<u32>, rng: R) -> Result<AI<R>> {
        let mut path = PathBuf::new();
        path.push("result");
        path.push(experiment.to_string());
        if let Option::Some(sub_experiment) = sub_experiment { path.push(sub_experiment.to_string()); }
        path.push("cnn.pt");

        #[inline]
        fn load_ai(python: Python, load: &Py<PyAny>, path: &str, device: &Py<PyAny>) -> Result<Py<PyAny>> {
            Result::Ok(load.call1(python, (path, device))?.getattr(python, "guess")?)
        }

        Result::Ok(AI {
            guess: Python::with_gil(|python| load_ai(python, &self.load, path.to_str().ok_or_else(|| anyhow!("failed to generate a path"))?, &self.device))?,
            rng,
            buffer: Vec::new()
        })

    }

}

#[derive(Debug)]
pub struct AI<R> {
    guess: Py<PyAny>,
    rng: R,
    buffer: Vec<Position>
}

impl<R: Rng> Player for AI<R> {
    #[inline]
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        let player = game.board().player();
        let opponent = game.board().opponent();
        let mut data = Array4::zeros((1, 2, 8, 8));

        Position::iter().for_each(|position| {
            if player.is_set(position) { data[(0, 0, position.row() as usize, position.column() as usize)] = 1.0; }
            if opponent.is_set(position) { data[(0, 1, position.row() as usize, position.column() as usize)] = 1.0; }
        });

        #[inline]
        fn get_output(python: Python, guess: &Py<PyAny>, data: Array4<f32>) -> Result<Array2<f32>> {
            Result::Ok(guess.call1(python, (data.into_pyarray(python),))?.extract::<&PyArray2<f32>>(python)?.to_owned_array())
        }

        let output: Array2<f32> = Python::with_gil(|python| get_output(python, &self.guess, data))?;
        self.buffer.clear();
        let legal = game.board().legal();
        let mut max = f32::NEG_INFINITY;

        Position::iter().filter(|position| legal.is_set(*position)).for_each(|position| {
            let value = output[(0, position.value() as usize)];

            if value > max {
                self.buffer.clear();
                self.buffer.push(position);
                max = value;
            } else if value == max {
                self.buffer.push(position);
            }

        });

        Result::Ok(*self.buffer.choose(&mut self.rng).unwrap())
    }
}
