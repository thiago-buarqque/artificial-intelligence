use serde_json::{json, Value};

use super::piece_utils::PieceType;

#[derive(Debug, Clone, PartialEq)]
pub struct PieceMove {
    pub from: i8,
    pub to: i8,
    pub promotion_type: i8,
    pub is_en_passant: bool,
}

impl PieceMove {
    pub fn new(from: i8, to: i8) -> Self {
        Self {
            from,
            to,
            promotion_type: PieceType::Empty as i8,
            is_en_passant: false,
        }
    }

    pub fn clone(&self) -> PieceMove {
        PieceMove {
            from: self.from,
            to: self.to,
            promotion_type: self.promotion_type,
            is_en_passant: self.is_en_passant,
        }
    }

    pub fn is_promotion(&self) -> bool {
        self.promotion_type != PieceType::Empty as i8
    }

    pub fn eq(&self, piece_move: &Self) -> bool {
        if self.promotion_type == piece_move.promotion_type {
            return (self.from == piece_move.from)
                && (self.to == piece_move.to)
                && (self.is_en_passant == piece_move.is_en_passant);
        }

        false
    }
}
