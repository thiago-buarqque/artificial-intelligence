use super::enums::{PieceColor, PieceType};

pub const EMPTY_PIECE: i8 = PieceType::Empty as i8;

pub const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const WHITE_LOWER_BOUND: i8 = PieceColor::White as i8 | PieceType::Bishop as i8;
pub const WHITE_UPPER_BOUND: i8 = PieceColor::White as i8 | PieceType::Rook as i8;