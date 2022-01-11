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
        Player
    }
};

use pyo3::{
    Python,
    Py,
    PyAny,
    types::IntoPyDict
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
            python.import("sys")?.getattr("path")?.call_method1("append", ("run",))?;
            let torch = python.import("torch")?;
            let depla = python.import("depla")?;
            let device = torch.call_method1("device", ("cuda",))?;
            let cnn = depla.call_method1("CNN", (8, 128, 128 * 8, device))?;
            let dataset = depla.call_method1("Dataset", ((2010..=2020).map(|year| format!("run/wthor/WTH_{}.wtb", year)).collect::<Vec<_>>(),))?;
            let loader = torch.getattr("utils")?.getattr("data")?.call_method("DataLoader", (dataset, 1024, true), Option::Some(vec![("drop_last", true)].into_py_dict(python)))?;
            let criterion = torch.getattr("nn")?.call_method0("CrossEntropyLoss")?;
            let optimizer = torch.getattr("optim")?.call_method("SGD", (cnn.call_method0("parameters")?, 0.0001, 0.9), Option::Some(vec![("weight_decay", 0.005)].into_py_dict(python)))?;
            let ai = depla.call_method1("AI", (cnn, loader, 16, criterion, optimizer, device))?;
            return Result::Ok(Py::from(ai));
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
        let mut max = f32::NEG_INFINITY;

        for row in 0..8 {
            for column in 0..8 {
                let index = Index::at(row, column).unwrap();

                if legal.is_set(index) {
                    let value = output[(0, row, column)];

                    if value > max {
                        self.buffer.clear();
                        self.buffer.push(index);
                        max = value;
                    } else if value == max {
                        self.buffer.push(index);
                    }

                }

            }
        }

        return Result::Ok(self.buffer[self.rng.gen_range(0..self.buffer.len())]);
    }
}
