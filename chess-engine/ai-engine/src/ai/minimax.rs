use crate::{
    common::{
        piece_move::PieceMove,
        piece_utils::{get_piece_worth, is_white_piece, PieceType, get_promotion_char_options, piece_value_from_fen, piece_fen_from_value},
    },
    game::board::Board,
};

pub struct MiniMax {
    pub states_checked: usize,
}

impl MiniMax {
    pub fn new() -> Self {
        MiniMax { states_checked: 0 }
    }

    pub fn make_move(&mut self, board: &mut Board, depth: u8) -> (i32, PieceMove) {
        self.states_checked = 0;

        self.minimax(board, depth, true)
    }

    fn minimax(&mut self, board: &mut Board, depth: u8, max: bool) -> (i32, PieceMove) {
        self.states_checked += 1;

        if depth == 0 || board.is_game_finished() {
            return (self.get_board_value(board), PieceMove::new(-1, -1));
        }

        let mut value = if max { i32::MIN } else { i32::MAX };
        let mut best_move: PieceMove = PieceMove::new(-1, -1);

        let pieces = board.get_pieces().clone();

        // get_available_moves should only return the pieces, not empties
        for piece in pieces.iter().flatten() {
            if (piece.get_value() == PieceType::Empty as i8)
                || (piece.is_white()) != board.is_white_move()
            {
                continue;
            }

            for piece_move in piece.get_immutable_moves().iter() {
                let mut promotion_char_options = vec![piece_fen_from_value(piece_move.promotion_type)];

                if piece_move.is_promotion {
                    promotion_char_options = get_promotion_char_options(piece.is_white());
                }

                let mut piece_move = piece_move.clone();

                for promotion_option in promotion_char_options {
                    piece_move.promotion_type = piece_value_from_fen(&promotion_option);

                    let _ = board.move_piece(piece_move.clone());

                    if max {
                        let current_move_value = self.minimax(board, depth - 1, false);

                        if current_move_value.0 > value {
                            value = current_move_value.0;
                            best_move = piece_move.clone();
                        }
                    } else {
                        let current_move_value = self.minimax(board, depth - 1, true);

                        if current_move_value.0 < value {
                            value = current_move_value.0;
                            best_move = piece_move.clone();
                        }
                    }

                    board.undo_move();
                }
            }
        }

        (value, best_move)
    }

    fn get_board_value(&self, board: &mut Board) -> i32 {
        // For this first algorithm version the
        // sum of the pieces will give the state value
        let mut board_value = 0;
        for piece in board.get_squares().iter() {
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
