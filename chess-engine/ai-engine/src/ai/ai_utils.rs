use crate::{
    common::{
        board_piece::BoardPiece,
        contants::EMPTY_PIECE,
        piece_move::PieceMove,
        piece_utils::{get_piece_type, get_piece_worth, is_white_piece}, enums::PieceType,
    },
    game::{board::Board, board_state::BoardState, move_generator_helper::get_adjacent_position},
};

pub fn get_sorted_moves(board: &Board, max: bool, pieces: Vec<BoardPiece>) -> Vec<PieceMove> {
    let mut moves: Vec<PieceMove> = pieces
        .iter()
        .filter(|piece| piece.is_white() == board.is_white_move())
        .flat_map(|piece| piece.get_moves_clone())
        .collect();

    let attacked_positions: Vec<i8> = pieces
        .iter()
        .filter(|piece| piece.is_white() != board.is_white_move())
        .flat_map(|piece| piece.get_moves_reference())
        .map(|_move| _move.get_to_position())
        .collect();

    let board_state = board.get_state_reference();

    for _move in moves.iter_mut() {
        let moving_piece = board_state.get_piece(_move.get_from_position());
        let target_piece = board_state.get_piece(_move.get_to_position());

        // Capturing move
        if _move.is_capture() {
            _move.set_move_worth(9 * get_piece_worth(target_piece) - get_piece_worth(moving_piece))
        }

        if _move.is_promotion() {
            _move.sum_to_move_worth(9);
        }

        // Penalize pieces from moving to a attacked position
        if attacked_positions.contains(&_move.get_to_position()) {
            _move.sum_to_move_worth(get_piece_worth(moving_piece))
        }
    }

    // TODO order also based on the hashmap with previous generated states

    if max {
        moves.sort_by_key(|k| std::cmp::Reverse(k.get_move_worth()));
    } else {
        moves.sort_by_key(|k| k.get_move_worth());
    }

    moves
}

pub fn get_board_value(board: &mut Board, pieces: &[BoardPiece]) -> i32 {
    // The evaluation
    // f(p) = 200(K-K')
    //         + 9(Q-Q')
    //         + 5(R-R')
    //         + 3(B-B' + N-N')
    //         + 1(P-P')
    //         - 0.5(D-D' + S-S' + I-I')
    //         + 0.1(M-M') + ...
    //
    // ' means the opponent score
    // KQRBNP = number of kings, queens, rooks, bishops, knights and pawns
    // D,S,I = doubled, blocked and isolated pawns
    // M = Mobility (the number of legal moves)

    let mut k = 0;
    let mut q = 0;
    let mut r = 0;
    let mut b = 0;
    let mut n = 0;
    let mut p = 0;

    let mut d = 0;
    let mut s = 0;
    let mut i = 0;
    let mut m = 0;

    let board_state = board.get_state_reference();

    for piece in pieces.iter() {
        if piece.get_value() == EMPTY_PIECE {
            continue;
        }

        let factor: i32 = if piece.is_white() == board.is_white_move() {
            1
        } else {
            -1
        };

        let piece_type = get_piece_type(piece.get_value());

        match piece_type {
            PieceType::King => k += factor,
            PieceType::Queen => q += factor,
            PieceType::Rook => r += factor,
            PieceType::Bishop => b += factor,
            PieceType::Knight => n += factor,
            PieceType::Pawn => p += factor,
            // Additional cases for D, S, I, and M are handled below
            _ => (),
        }

        if piece_type == PieceType::Pawn {
            if is_doubled_pawn(board_state, piece.get_position(), piece.is_white()) {
                d += factor;
            }

            if is_blocked_pawn(board_state, piece.get_position(), piece.is_white()) {
                s += 1;
            }

            if is_isolated_pawn(board_state, piece.get_position(), piece.is_white()) {
                i += 1;
            }
        }

        for _move in piece.get_moves_reference().iter() {
            if piece.is_white() == board.is_white_move() {
                m += 1
            } else {
                m -= 1
            };
        }
    }

    (200 * k) + (9 * q) + (5 * r) + (3 * (b + n)) + p - ((d + s + i) / 2) + (m / 10)
}

pub fn is_isolated_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
    let positions = [
        get_adjacent_position(position, position - 1),
        get_adjacent_position(position, position + 1),
        get_adjacent_position(position, position - 9),
        get_adjacent_position(position, position - 8),
        get_adjacent_position(position, position - 7),
        get_adjacent_position(position, position + 7),
        get_adjacent_position(position, position + 8),
        get_adjacent_position(position, position + 9),
    ];

    for adjacent_position in positions {
        if !board_state.is_valid_position(adjacent_position) {
            continue;
        }

        let piece = board_state.get_piece(adjacent_position);

        if piece == EMPTY_PIECE {
            continue;
        }

        if get_piece_type(piece) == PieceType::Pawn && is_white_piece(piece) == white_piece {
            return false;
        }
    }

    true
}

pub fn is_blocked_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
    let offset: i8 = if white_piece { -8 } else { 8 };

    let frontal_piece = board_state.get_piece(position + offset);

    if get_piece_type(frontal_piece) != PieceType::Empty
    // && white_piece != is_white_piece(frontal_piece)
    {
        let mut diagonal_left = 0;
        let mut diagonal_right = 0;

        if position % 8 != 0 {
            let diagonal_offset = if white_piece { -1 } else { 1 };

            diagonal_left = board_state.get_piece(position + offset + diagonal_offset);
        }

        if (position + 1) % 8 != 0 {
            let diagonal_offset = if white_piece { 1 } else { -1 };

            diagonal_right = board_state.get_piece(position + offset + diagonal_offset);
        }

        let diagonal_left_color = is_white_piece(diagonal_left);
        let diagonal_right_color = is_white_piece(diagonal_right);

        if diagonal_left == 0 && diagonal_right == 0 {
            return true;
        } else if diagonal_left != 0 && diagonal_right == 0 {
            return diagonal_left_color == white_piece;
        } else if diagonal_right != 0 && diagonal_left == 0 {
            return diagonal_right_color == white_piece;
        }

        return diagonal_left_color == white_piece && diagonal_right_color == white_piece;
    }

    false
}

pub fn is_doubled_pawn(board_state: &BoardState, position: i8, white_piece: bool) -> bool {
    let offset: i8 = if white_piece { -8 } else { 8 };

    let mut _position = position + offset;
    while board_state.is_valid_position(_position) {
        let frontal_piece = board_state.get_piece(_position);

        _position += offset;

        if frontal_piece == EMPTY_PIECE {
            continue;
        }

        let piece_type = get_piece_type(frontal_piece);

        if piece_type != PieceType::Pawn {
            return false;
        } else if white_piece == is_white_piece(frontal_piece) {
            return true;
        }
    }

    false
}
