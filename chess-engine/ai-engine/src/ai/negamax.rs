use crate::{
    common::{
        board_piece::BoardPiece,
        piece_move::PieceMove,
        piece_utils::{get_piece_type, get_promotion_options, is_white_piece, PieceType},
    },
    game::{board::Board, board_state::BoardState, move_generator_helper::get_adjacent_position},
};

use super::ai_utils::get_ordered_moves;

pub struct Negamax {
    pub states_checked: usize,
}

impl Negamax {
    pub fn new() -> Self {
        Negamax { states_checked: 0 }
    }

    pub fn make_move(&mut self, board: &mut Board, depth: u8) -> (i32, PieceMove) {
        let result = self.negamax(board, i32::MIN, i32::MAX, depth);

        println!("Evaluated {} states", result.2);

        (result.0, result.1)
    }

    fn negamax(
        &mut self,
        board: &mut Board,
        _alpha: i32,
        _beta: i32,
        depth: u8,
    ) -> (i32, PieceMove, u64) {
        let mut alpha = _alpha;
        let beta = _beta;

        let pieces: Vec<BoardPiece> = board.get_pieces();

        if depth == 0 || board.is_game_finished() {
            // Start a new search the look for positions where doesn't have captures
            return (
                self.get_board_value(board, &pieces),
                PieceMove::new(-1, 0, -1),
                1,
            );
        }

        let mut best_move: PieceMove = PieceMove::new(-1, 0, -1);
        let mut moves_count = 0;
        let mut value = i32::MIN;

        let mut moves: Vec<PieceMove> = get_ordered_moves(board, pieces);

        'piece_move_loop: for _move in moves.iter_mut() {
            let mut promotion_options = vec![_move.promotion_type];

            if _move.is_promotion {
                promotion_options = get_promotion_options(is_white_piece(_move.piece_value));
            }

            for promotion_option in promotion_options {
                _move.promotion_type = promotion_option;

                board.move_piece(_move);

                let node_results = self.negamax(board, -beta, -alpha, depth - 1);

                moves_count += node_results.2;

                if -node_results.0 > value {
                    value = -node_results.0;
                    best_move = _move.clone();
                }

                alpha = alpha.max(value);

                board.undo_move();

                if alpha >= beta {
                    break 'piece_move_loop;
                }
            }
        }

        (value, best_move, moves_count)
    }

    fn get_board_value(&self, board: &mut Board, pieces: &[BoardPiece]) -> i32 {
        // f(p) = 200(K-K')
        //         + 9(Q-Q')
        //         + 5(R-R')
        //         + 3(B-B' + N-N')
        //         + 1(P-P')
        //         - 0.5(D-D' + S-S' + I-I')
        //         + 0.1(M-M') + ...

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

        // Pieces worth
        for piece in pieces.iter() {
            if piece.get_value() == PieceType::Empty as i8 {
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
                if self.is_doubled_pawn(board_state, piece.get_position(), piece.is_white()) {
                    d += factor;
                }

                if self.is_blocked_pawn(board_state, piece.get_position(), piece.is_white()) {
                    s += 1;
                }

                if self.is_isolated_pawn(board_state, piece.get_position(), piece.is_white()) {
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

        200 * k + 9 * q + 5 * r + 3 * (b + n) + p - ((d + s + i) / 2) + (m / 10)
    }

    fn is_isolated_pawn(&self, board_state: &BoardState, position: i8, white_piece: bool) -> bool {
        let position = position;

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

        for position in positions {
            if !board_state.is_valid_position(position) {
                continue;
            }

            let piece = board_state.get_piece(position);

            if piece == 0 {
                continue;
            }

            if get_piece_type(piece) == PieceType::Pawn && 
                is_white_piece(piece) == white_piece {
                return false;
            }
        }

        true
    }

    fn is_blocked_pawn(&self, board_state: &BoardState, position: i8, white_piece: bool) -> bool {
        let offset: i8 = if white_piece { -8 } else { 8 };

        let frontal_pawn = board_state.get_piece(position + offset);

        if get_piece_type(frontal_pawn) != PieceType::Empty
            && white_piece != is_white_piece(frontal_pawn)
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

    fn is_doubled_pawn(&self, board_state: &BoardState, position: i8, white_piece: bool) -> bool {
        let offset: i8 = if white_piece { -8 } else { 8 };

        let frontal_pawn = board_state.get_piece(position + offset);

        if get_piece_type(frontal_pawn) == PieceType::Pawn
            && white_piece == is_white_piece(frontal_pawn)
        {
            return true;
        }

        false
    }
}
