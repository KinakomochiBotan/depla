mod random;

use self::random::RandomMoves;
use std::cmp::Ordering;
use anyhow::Result;

use othello::processor::{
    Moves,
    PlayersMoves
};

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
fn random(python: Python, cnn: &PyAny, times: u32) -> PyResult<((u32, u32, u32), (u32, u32, u32))> {
    let mut ai = PyAI::new(python, cnn)?;
    let mut random = RandomMoves::new(rand::thread_rng());
    let result1 = play(&mut PlayersMoves::new(&mut ai, &mut random), times)?;
    let result2 = play(&mut PlayersMoves::new(&mut random, &mut ai), times)?;
    PyResult::Ok(((result1.0, result1.1, result1.2), (result2.2, result2.1, result2.0)))
}

#[inline]
fn play<M: Moves>(moves: &mut M, times: u32) -> Result<(u32, u32, u32)> {
    let mut black = 0;
    let mut draw = 0;
    let mut white = 0;

    for _ in 0..times {
        let result = othello::processor::play(moves)?;

        match result.0.cmp(&result.1) {
            Ordering::Greater => black += 1,
            Ordering::Equal => draw += 1,
            Ordering::Less => white += 1
        }

    }

    Result::Ok((black, draw, white))
}
