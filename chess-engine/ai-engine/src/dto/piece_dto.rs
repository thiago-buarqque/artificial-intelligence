use pyo3::prelude::*;
use serde_json::json;

use crate::common::piece_move::PieceMove;

use super::piece_move_dto::PieceMoveDTO;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PieceDTO {
    #[pyo3(get, set)]
    pub fen: String,
    #[pyo3(get, set)]
    pub moves: Vec<PieceMoveDTO>,
    #[pyo3(get, set)]
    pub position: i8,
    #[pyo3(get, set)]
    pub white: bool,
}

impl PieceDTO {
    pub fn new(fen: String, moves: Vec<PieceMove>, position: i8, white: bool) -> Self {
        PieceDTO {
            fen,
            moves: moves.iter().map(
                |_move| PieceMoveDTO::from_piece_move(_move.clone())
            ).collect(),
            position,
            white,
        }
    }

    pub fn to_json_str(&self) -> String {
        json!({
            "fen": self.fen,
            "moves": json!(self.moves.iter().map(
                |_move| _move.to_json_str()
            ).collect::<Vec<_>>()),
            "position": self.position,
            "white": self.white
        }).to_string()
    }
}
