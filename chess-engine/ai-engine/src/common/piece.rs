use pyo3::prelude::*;
use crate::common::board_piece::BoardPiece;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Piece {
    #[pyo3(get, set)]
    pub fen: String,
    #[pyo3(get, set)]
    pub moves: Vec<i8>,
    #[pyo3(get, set)]
    pub position: i8,
    #[pyo3(get, set)]
    pub white: bool,
}

#[pymethods]
impl Piece {
    #[new]
    pub fn new(fen: String, moves: Vec<i8>, position: i8, white: bool) -> Self {
        Piece {
            fen,
            moves,
            position,
            white,
        }
    }
}
