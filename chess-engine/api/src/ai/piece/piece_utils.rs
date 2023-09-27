#[derive(PartialEq, Eq)]
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

// You may need to adjust this based on how Piece is implemented in your Rust code
pub struct Piece {
    moves: Vec<i32>,
    position: i32,
    fen: String,
    white: bool,
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

pub fn pieces_to_fen(pieces: Vec<i32>) -> Vec<String> {
    pieces.iter().map(|&piece| piece_fen_from_value(piece)).collect()
}

pub fn is_piece_of_type(piece: i32, piece_type: PieceType) -> bool {
    get_piece_type(piece) == (piece_type as i32)
}

pub fn piece_value_from_fen(piece_fen: &char) -> i32 {
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

    (color as i32) | (piece_type as i32)
}

pub fn piece_fen_from_value(piece_value: i32) -> String {
    // Since we're using enum values as i32, we need to match directly with these i32 values.
    // In a more idiomatic Rust code, you might want to use pattern matching with enums directly.
    String::from(match piece_value {
        0 => "-",
        17 => "B",
        18 => "K",
        19 => "N",
        20 => "P",
        21 => "Q",
        22 => "R",
        9 => "b",
        10 => "k",
        11 => "n",
        12 => "p",
        13 => "q",
        14 => "r",
        _ => "",
    })
}

pub fn get_piece_type(piece_value: i32) -> i32 {
    // Similar to above, matching i32 directly.
    (match piece_value {
        0 => PieceType::Empty,
        17 | 9 => PieceType::Bishop,
        18 | 10 => PieceType::King,
        19 | 11 => PieceType::Knight,
        20 | 12 => PieceType::Pawn,
        21 | 13 => PieceType::Queen,
        22 | 14 => PieceType::Rook,
        _ => PieceType::Empty,
    }) as i32
}

// fn create_piece(moves: Vec<i32>, position: i32, fen: &str, white: bool) -> Piece {
//     Piece {
//         moves,
//         position,
//         fen: fen.to_string(),
//         white,
//     }
// }

// Uncomment this to run a quick test
// fn main() {
//     let piece_value = piece_value_from_fen("B");
//     let fen = piece_fen_from_value(piece_value);
//     println!("Piece value: {}", piece_value);
//     println!("FEN: {}", fen);
// }
