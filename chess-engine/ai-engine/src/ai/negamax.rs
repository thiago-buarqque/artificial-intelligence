use std::{
    ops::Add,
    sync::{Arc, Mutex},
};

use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    common::{
        board_piece::BoardPiece,
        piece_move::PieceMove,
        piece_utils::{get_promotion_options, is_white_piece},
    },
    game::board::Board,
};

use super::ai_utils::{get_board_value, get_sorted_moves};

pub struct Negamax {}

impl Negamax {
    pub fn new() -> Self {
        Negamax {}
    }

    pub fn make_move(&mut self, board: &mut Board, depth: u8) -> (i32, PieceMove) {
        let best_move = Arc::new(Mutex::new(PieceMove::new(-1, 0, -1)));
        let moves_count: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
        let value = Arc::new(Mutex::new(i32::MIN));

        let pieces: Vec<BoardPiece> = board.get_pieces();

        let mut moves: Vec<PieceMove> = get_sorted_moves(board, true, pieces, false);

        moves.par_iter_mut().for_each(|_move| {
            let mut board_copy = board.clone();

            self.search_parallel(
                _move,
                &mut board_copy,
                depth - 1,
                &value,
                &moves_count,
                &best_move,
            );
        });

        println!("Evaluated {} states", moves_count.lock().unwrap());

        let locked_value = value.lock().unwrap();

        let locked_best_move = best_move.lock().unwrap();

        (locked_value.to_owned(), locked_best_move.to_owned())
    }

    fn search_parallel(
        &self,
        _move: &mut PieceMove,
        board: &mut Board,
        depth: u8,
        value: &Arc<Mutex<i32>>,
        moves_count: &Arc<Mutex<u64>>,
        best_move: &Arc<Mutex<PieceMove>>,
    ) {
        let promotion_options = if _move.is_promotion {
            get_promotion_options(is_white_piece(_move.piece_value))
        } else {
            vec![_move.promotion_type]
        };

        for promotion_option in promotion_options {
            _move.promotion_type = promotion_option;

            let _ = board.move_piece(_move);

            let node_results = Self::negamax(board, i32::MIN, i32::MAX, false, depth - 1);

            let mut locked_moves_count = moves_count.lock().unwrap();

            *locked_moves_count = locked_moves_count.add(node_results.1);

            drop(locked_moves_count);

            let mut locked_value = value.lock().unwrap();

            if -node_results.0 > *locked_value {
                *locked_value = -node_results.0;

                let mut locked_best_move = best_move.lock().unwrap();

                *locked_best_move = _move.clone();

                drop(locked_best_move)
            }

            drop(locked_value);

            board.undo_move();
        }
    }

    fn negamax(
        board: &mut Board,
        _alpha: i32,
        _beta: i32,
        max: bool,
        depth: u8,
    ) -> (i32, u64) {
        let mut alpha = _alpha;
        let beta = _beta;

        let pieces: Vec<BoardPiece> = board.get_pieces();

        if depth == 0 || board.is_game_finished() {
            // Start a new search that look for positions with no captures available
            return (Self::search_captures(board, alpha, beta, max, 1), 1);
        }

        let mut moves_count = 0;
        let mut value = i32::MIN;

        let mut moves: Vec<PieceMove> = get_sorted_moves(board, max, pieces, false);

        'piece_move_loop: for _move in moves.iter_mut() {
            let promotion_options = if _move.is_promotion {
                get_promotion_options(is_white_piece(_move.piece_value))
            } else {
                vec![_move.promotion_type]
            };

            for promotion_option in promotion_options {
                _move.promotion_type = promotion_option;

                let _ = board.move_piece(_move);

                let node_results = Self::negamax(board, -beta, -alpha, !max, depth - 1);

                board.undo_move();

                moves_count += node_results.1;

                value = value.max(-node_results.0);

                alpha = alpha.max(value);

                if alpha >= beta {
                    break 'piece_move_loop;
                }
            }
        }

        (value, moves_count)
    }

    fn search_captures(
        board: &mut Board,
        _alpha: i32,
        _beta: i32,
        max: bool,
        depth: u32,
    ) -> i32 {
        let mut alpha = _alpha;
        let beta = _beta;
        
        let pieces: Vec<BoardPiece> = board.get_pieces();
        
        let mut value = get_board_value(board, &pieces);
        
        if value >= beta {
            return beta;
        }

        alpha = alpha.max(value);

        let mut capture_moves: Vec<PieceMove> = get_sorted_moves(board, max, pieces, true);
        
        // println!("Depth: {} | {} captures", depth, capture_moves.len());
        if board.is_game_finished() || capture_moves.is_empty() {
            return value;
        }

        for _move in capture_moves.iter_mut() {
            let promotion_options = if _move.is_promotion {
                get_promotion_options(is_white_piece(_move.piece_value))
            } else {
                vec![_move.promotion_type]
            };

            for promotion_option in promotion_options {
                _move.promotion_type = promotion_option;

                let _ = board.move_piece(_move);

                value = Self::search_captures(board, -beta, -alpha, !max, depth + 1);

                board.undo_move();
            
                if value >= beta {
                    return beta;
                }

                alpha = alpha.max(value);
            }
        }

        alpha
    }
}
