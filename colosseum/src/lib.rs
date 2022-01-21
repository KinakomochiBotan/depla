mod random;

use self::random::RandomMoves;
use std::cmp::Ordering;
use othello::processor::PlayersMoves;
use ai::PyAI;

use pyo3::{
    Python,
    PyResult,
    PyAny,
    types::PyModule
};

#[pyo3::pymodule]
fn colosseum(_: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(random, module)?)
}

#[pyo3::pyfunction]
fn random(python: Python, cnn: &PyAny, times: u32) -> PyResult<(u32, u32, u32)> {
    let mut ai = PyAI::new(python, cnn)?;
    let mut random = RandomMoves::new(rand::thread_rng());
    let mut players = PlayersMoves::new(&mut ai, &mut random);
    let mut ai = 0;
    let mut draw = 0;
    let mut random = 0;

    for _ in 0..times {
        let result = othello::processor::play(&mut players)?;

        match result.0.cmp(&result.1) {
            Ordering::Greater => ai += 1,
            Ordering::Equal => draw += 1,
            Ordering::Less => random += 1
        }

    }

    PyResult::Ok((ai, draw, random))
}
