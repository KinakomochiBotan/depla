use anyhow::Result;

use ndarray::{
    Array2,
    Array4
};

use rand::{
    Rng,
    seq::SliceRandom
};

use othello::{
    game::{
        Position,
        Game
    },
    players::Player
};

use pyo3::{
    Python,
    Py,
    PyAny,
    types::IntoPyDict
};

use numpy::{
    PyArray2,
    IntoPyArray
};

#[derive(Debug)]
pub struct AI<R> {
    ai: Py<PyAny>,
    rng: R,
    buffer: Vec<Position>
}

impl<R> AI<R> {
    #[inline]
    pub fn new(rng: R) -> Result<Self> {

        #[inline]
        fn get_ai(python: Python) -> Result<Py<PyAny>> {
            python.import("sys")?.getattr("path")?.call_method1("append", ("run",))?;
            let torch = python.import("torch")?;
            let depla = python.import("depla")?;
            let device = torch.call_method1("device", ("cuda",))?;
            let cnn = depla.call_method1("CNN", (device,))?;
            let dataset = depla.call_method1("Dataset", ((2010..=2019).map(|year| format!("run/wthor/WTH_{}.wtb", year)).collect::<Vec<_>>(),))?;
            let loader = torch.getattr("utils")?.getattr("data")?.call_method("DataLoader", (dataset, 2048, true), Option::Some(vec![("drop_last", true)].into_py_dict(python)))?;
            let criterion = torch.getattr("nn")?.call_method0("CrossEntropyLoss")?;
            let optimizer = torch.getattr("optim")?.call_method("SGD", (cnn.call_method0("parameters")?, 0.01, 0.95), Option::Some(vec![("weight_decay", 0.0005)].into_py_dict(python)))?;
            let ai = depla.call_method1("AI", (cnn, loader, 8, criterion, optimizer, device))?;
            Result::Ok(Py::from(ai))
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
    fn get_move(&mut self, game: &Game) -> Result<Position> {
        let player = game.board().player();
        let opponent = game.board().opponent();
        let mut data = Array4::zeros((1, 2, 8, 8));

        Position::iter().for_each(|position| {
            if player.is_set(position) { data[(0, 0, position.row() as usize, position.column() as usize)] = 1.0; }
            if opponent.is_set(position) { data[(0, 1, position.row() as usize, position.column() as usize)] = 1.0; }
        });

        #[inline]
        fn get_output(python: Python, ai: &Py<PyAny>, data: Array4<f32>) -> Result<Array2<f32>> {
            Result::Ok(ai.call_method1(python, "guess", (data.into_pyarray(python),))?.extract::<&PyArray2<f32>>(python)?.to_owned_array())
        }

        let output: Array2<f32> = Python::with_gil(|python| get_output(python, &self.ai, data))?;
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
