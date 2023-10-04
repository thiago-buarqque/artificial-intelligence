#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PieceType {
    Empty = 0,
    Bishop = 1,
    King = 2,
    Knight = 3,
    Pawn = 4,
    Queen = 5,
    Rook = 6,
}

pub enum PieceColor {
    Black = 8,
    White = 16,
}

// static PIECE_SYMBOLS: HashMap<&'static str, &'static str> = [
//     ("B", "♗"),
//     ("K", "♔"),
//     ("N", "♘"),
//     ("P", "♙"),
//     ("Q", "♕"),
//     ("R", "♖"),
//     ("b", "♝"),
//     ("k", "♚"),
//     ("n", "♞"),
//     ("p", "♟︎"),
//     ("q", "♛"),
//     ("r", "♜"),
// ]
// .iter()
// .copied()
// .collect();

pub fn pieces_to_fen(pieces: &[i8]) -> Vec<char> {
    pieces
        .iter()
        .map(|&piece| piece_fen_from_value(piece))
        .collect()
}

pub fn is_piece_of_type(piece: i8, piece_type: PieceType) -> bool {
    get_piece_type(piece) == piece_type
}

pub fn piece_value_from_fen(piece_fen: &char) -> i8 {
    let color = if piece_fen.is_uppercase() {
        PieceColor::White
    } else {
        PieceColor::Black
    };

    let piece_type = match piece_fen.to_lowercase().next().unwrap() {
        'b' => PieceType::Bishop,
        'k' => PieceType::King,
        'n' => PieceType::Knight,
        'p' => PieceType::Pawn,
        'q' => PieceType::Queen,
        'r' => PieceType::Rook,
        _ => PieceType::Empty,
    };

    (color as i8) | (piece_type as i8)
}

pub fn piece_fen_from_value(piece_value: i8) -> char {
    match piece_value {
        17 => 'B',
        18 => 'K',
        19 => 'N',
        20 => 'P',
        21 => 'Q',
        22 => 'R',
        9 => 'b',
        10 => 'k',
        11 => 'n',
        12 => 'p',
        13 => 'q',
        14 => 'r',
        _ => '-',
    }
}

pub fn get_promotion_char_options(white: bool) -> Vec<char> {
    if white {
        return vec!['q', 'r', 'b', 'n'];
    }
    vec!['Q', 'R', 'B', 'N']
}

pub fn get_piece_type(piece_value: i8) -> PieceType {
    match piece_value {
        0 => PieceType::Empty,
        17 | 9 => PieceType::Bishop,
        18 | 10 => PieceType::King,
        19 | 11 => PieceType::Knight,
        20 | 12 => PieceType::Pawn,
        21 | 13 => PieceType::Queen,
        22 | 14 => PieceType::Rook,
        _ => PieceType::Empty,
    }
}

pub fn get_piece_worth(piece_value: i8) -> i32 {
    match piece_value {
        17 | 9 => 330,
        18 | 10 => 20000,
        19 | 11 => 320,
        20 | 12 => 100,
        21 | 13 => 900,
        22 | 14 => 500,
        _ => 0,
    }
}

const WHITE_LOWER_BOUND: i8 = PieceColor::White as i8 | PieceType::Bishop as i8;
const WHITE_UPPER_BOUND: i8 = PieceColor::White as i8 | PieceType::Rook as i8;

pub fn is_white_piece(piece_value: i8) -> bool {
    (WHITE_LOWER_BOUND..=WHITE_UPPER_BOUND).contains(&piece_value)
}

pub fn is_same_color(piece1: i8, piece2: i8) -> bool {
    is_white_piece(piece1) == is_white_piece(piece2)
}
