mod ai;
mod board_wrapper;
mod common;
mod game;
mod dto;

use board_wrapper::BoardWrapper;
use dto::piece_dto::PieceDTO;
use pyo3::{exceptions::PyValueError, prelude::*, types::PyList};

#[pymodule]
fn ai_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    // env::set_var("RUST_BACKTRACE", "1");
    m.add_class::<PieceDTO>()?;
    m.add_class::<BoardWrapper>()?;

    Ok(())
}
