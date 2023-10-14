use crate::{common::{piece_move::PieceMove, board_piece::BoardPiece, piece_utils::{PieceType, get_piece_worth}}, game::board::Board};

pub fn get_ordered_moves(board: &Board, pieces: Vec<BoardPiece>) -> Vec<PieceMove> {
    let mut moves: Vec<PieceMove> = pieces
        .iter()
        .filter(|piece| piece.is_white() == board.is_white_move())
        .flat_map(|piece| piece.get_immutable_moves())
        .collect();

    let attacked_positions: Vec<i8> = pieces
        .iter()
        .filter(|piece| piece.is_white() != board.is_white_move())
        .flat_map(|piece| piece.get_immutable_moves())
        .map(|_move| _move.to_position)
        .collect();

    let board_state = board.get_state_reference();

    for _move in moves.iter_mut() {
        let moving_piece = board_state.get_piece(_move.from_position);
        let target_piece = board_state.get_piece(_move.to_position);

        // Capturing move
        if target_piece != PieceType::Empty as i8 {
            _move.move_worth += get_piece_worth(target_piece)
        }

        if _move.is_promotion {
            _move.move_worth += 9
        }

        // Penalize pieces from moving to a attacked position
        if attacked_positions.contains(&_move.to_position) {
            _move.move_worth -= get_piece_worth(moving_piece)
        }
    }

    // if max {
    moves.sort_by_key(|k| std::cmp::Reverse(k.move_worth));
    // } else {
    //     moves.sort_by_key(|k| k.move_worth);
    // }
    // moves.sort_by_key(|_move| _move.move_worth);

    moves
}
