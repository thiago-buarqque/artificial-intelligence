use crate::{
    common::{
        board_piece::BoardPiece,
        piece_move::PieceMove,
        piece_utils::{get_piece_worth, get_promotion_options, is_white_piece, PieceType},
    },
    game::board::Board,
};

use super::ai_utils::get_ordered_moves;

pub struct MiniMaxAlphaBetaMoveOrder {
    pub states_checked: usize,
}

impl MiniMaxAlphaBetaMoveOrder {
    pub fn new() -> Self {
        MiniMaxAlphaBetaMoveOrder { states_checked: 0 }
    }

    pub fn make_move(&mut self, board: &mut Board, depth: u8) -> (i32, PieceMove) {
        let result = self.minimax(board, i32::MIN, i32::MAX, depth, true);

        println!("Evaluated {} states", result.2);

        (result.0, result.1)
    }

    fn minimax(
        &mut self,
        board: &mut Board,
        _alpha: i32,
        _beta: i32,
        depth: u8,
        max: bool,
    ) -> (i32, PieceMove, u64) {
        let mut alpha = _alpha;
        let mut beta = _beta;

        if depth == 0 || board.is_game_finished() {
            return (self.get_board_value(board), PieceMove::new(-1, 0, -1), 1);
        }

        let mut moves_count = 0;
        let mut value = if max { i32::MIN } else { i32::MAX };
        let mut best_move: PieceMove = PieceMove::new(-1, 0, -1);

        let pieces = board.get_pieces();

        let mut moves: Vec<PieceMove> = get_ordered_moves(board, pieces);

        'piece_move_loop: for _move in moves.iter_mut() {
            let mut promotion_options = vec![_move.promotion_type];

            if _move.is_promotion {
                promotion_options = get_promotion_options(is_white_piece(_move.piece_value));
            }

            for promotion_option in promotion_options {
                _move.promotion_type = promotion_option;

                board.move_piece(_move);

                if max {
                    let current_move_value = self.minimax(board, alpha, beta, depth - 1, false);
                    
                    moves_count += current_move_value.2;

                    if current_move_value.0 > value {
                        value = current_move_value.0;
                        best_move = _move.clone();
                    }

                    alpha = alpha.max(value);

                    if value >= beta {
                        board.undo_move();
                        break 'piece_move_loop;
                    }
                } else {
                    let current_move_value = self.minimax(board, alpha, beta, depth - 1, true);
                    
                    moves_count += current_move_value.2;

                    if current_move_value.0 < value {
                        value = current_move_value.0;
                        best_move = _move.clone();
                    }

                    beta = value.min(value);

                    if value <= alpha {
                        board.undo_move();
                        break 'piece_move_loop;
                    }
                }

                board.undo_move();
            }
        }

        (value, best_move, moves_count)
    }

    fn get_board_value(&self, board: &mut Board) -> i32 {
        // For this first algorithm version the
        // sum of the pieces will give the state value
        let mut board_value: i32 = 0;
        for piece in board.get_squares().iter() {
            if *piece == PieceType::Empty as i8 {
                continue;
            }

            let piece_worth: i32 = get_piece_worth(*piece);

            if is_white_piece(*piece) {
                board_value += piece_worth;
            } else {
                board_value -= piece_worth;
            }
        }

        board_value
    }
}
