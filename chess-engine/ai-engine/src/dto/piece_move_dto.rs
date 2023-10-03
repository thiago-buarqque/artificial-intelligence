use pyo3::prelude::*;
use serde_json::json;

use crate::common::piece_move::PieceMove;
use crate::common::piece_utils::piece_fen_from_value;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PieceMoveDTO {
    #[pyo3(get)]
    pub from: i8,
    #[pyo3(get)]
    pub to: i8,
    #[pyo3(get)]
    pub promotion_type: String,
    #[pyo3(get)]
    pub is_en_passant: bool,
}

impl PieceMoveDTO {
    pub fn from_piece_move(piece_move: PieceMove) -> PieceMoveDTO {
        PieceMoveDTO {
            from: piece_move.from,
            to: piece_move.to,
            promotion_type: piece_fen_from_value(piece_move.promotion_type),
            is_en_passant: piece_move.is_en_passant,
        }
    }

    pub fn to_json_str(&self) -> String {
        json!({
            "from": self.from,
            "to": self.to,
            "is_en_passant": self.is_en_passant,
            "promotion_type": self.promotion_type
        }).to_string()
    }
}