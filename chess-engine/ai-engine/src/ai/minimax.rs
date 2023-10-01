use std::sync::{Arc, Mutex};

use crate::{
    common::piece_utils::{get_piece_worth, is_white_piece, PieceType},
    game::board::Board,
};

pub struct MiniMax {
    pub states_checked: usize,
}

impl MiniMax {
    pub fn new() -> Self {
        MiniMax { states_checked: 0 }
    }

    pub fn make_move(&mut self, board: &Arc<Mutex<Board>>) -> (i32, (i8, i8)) {
        let depth: u8 = 4;

        self.minimax(board, depth, true)
    }

    fn minimax(&mut self, board: &Arc<Mutex<Board>>, depth: u8, max: bool) -> (i32, (i8, i8)) {
        let mut locked_board = board.lock().unwrap();

        if depth == 0 || locked_board.is_game_finished() {
            drop(locked_board);

            return (self.get_board_value(board), (-1, -1));
        }

        let mut value = if max { i32::MIN } else { i32::MAX };
        let mut best_move: (i8, i8) = (-1, -1);

        let pieces = locked_board.get_pieces();

        // get_available_moves should only return the pieces, not empties
        for piece in pieces.iter().flatten() {
            if (piece.get_value() == PieceType::Empty as i8)
                || (is_white_piece(piece.get_value()) != locked_board.is_white_move())
            {
                continue;
            }

            for piece_move in piece.get_immutable_moves().iter() {
                let state = locked_board.get_state().clone();
                let _ = locked_board.move_piece(piece.get_position(), *piece_move);

                self.states_checked += 1;

                drop(locked_board);

                if max {
                    let current_move_value = self.minimax(board, depth - 1, false);

                    if current_move_value.0 > value {
                        value = current_move_value.0;
                        best_move = (piece.get_position(), *piece_move);
                    }
                } else {
                    let current_move_value = self.minimax(board, depth - 1, true);

                    if current_move_value.0 < value {
                        value = current_move_value.0;
                        best_move = (piece.get_position(), *piece_move);
                    }
                }

                locked_board = board.lock().unwrap();

                locked_board.load_state_and_clear_history(state);
            }
        }

        (value, best_move)
    }

    fn get_board_value(&self, board: &Arc<Mutex<Board>>) -> i32 {
        // For this first algorithm version the sum of the pieces will give the board value
        let locked_board = board.lock().unwrap();

        let mut board_value = 0;
        for piece in locked_board.get_squares().iter() {
            if *piece == PieceType::Empty as i8 {
                continue;
            }

            let piece_worth = get_piece_worth(*piece);

            if is_white_piece(*piece) {
                board_value += piece_worth;
            } else {
                board_value -= piece_worth;
            }
        }

        board_value
    }
}
