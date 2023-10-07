use crate::dto::piece_move_dto::PieceMoveDTO;

use super::piece_utils::{piece_value_from_fen, PieceType};

#[derive(Debug, Clone, PartialEq)]
pub struct PieceMove {
    pub from_position: i8,
    pub to_position: i8,
    pub promotion_type: i8,
    pub is_promotion: bool,
    pub is_en_passant: bool,
}

impl PieceMove {
    pub fn new(from: i8, to: i8) -> Self {
        Self {
            from_position: from,
            to_position: to,
            promotion_type: PieceType::Empty as i8,
            is_promotion: false,
            is_en_passant: false,
        }
    }

    pub fn from_dto(piece_move_dto: PieceMoveDTO) -> Self {
        Self {
            from_position: piece_move_dto.from_position,
            to_position: piece_move_dto.to_position,
            promotion_type: piece_value_from_fen(&piece_move_dto.promotion_type),
            is_promotion: piece_move_dto.is_promotion,
            is_en_passant: piece_move_dto.is_en_passant,
        }
    }

    pub fn clone(&self) -> PieceMove {
        PieceMove {
            from_position: self.from_position,
            to_position: self.to_position,
            promotion_type: self.promotion_type,
            is_promotion: self.is_promotion,
            is_en_passant: self.is_en_passant,
        }
    }

    pub fn set_is_promotion(&mut self, is_promotion: bool) {
        self.is_promotion = is_promotion
    }

    pub fn is_promotion(&self) -> bool {
        self.is_promotion
    }

    pub fn eq(&self, piece_move: &Self) -> bool {
        (self.promotion_type == piece_move.promotion_type)
            && (self.from_position == piece_move.from_position)
            && (self.to_position == piece_move.to_position)
            && (self.is_promotion == piece_move.is_promotion)
            && (self.is_en_passant == piece_move.is_en_passant)
    }
}
