use std::sync::{Arc, Mutex};

use pyo3::{exceptions, prelude::*};

use crate::{
    ai::minimax::MiniMax,
    common::{
        contants::INITIAL_FEN,
        piece_utils::{is_white_piece, PieceType}, piece_move::PieceMove,
    },
    dto::{piece_dto::PieceDTO, piece_move_dto::PieceMoveDTO, dto_utils::piece_move_dto_from_piece_move},
    game::{board::Board, board_state::BoardState},
};

#[pyclass]
pub struct BoardWrapper {
    board: Board,
    mini_max: MiniMax,
}

// is this a facade?
#[pymethods]
impl BoardWrapper {
    #[new]
    fn default() -> BoardWrapper {
        let state = Arc::new(Mutex::new(BoardState::new()));

        let mut board: Board = Board::new(state.clone());

        board.load_position(INITIAL_FEN);

        BoardWrapper {
            board,
            mini_max: MiniMax::new(),
        }
    }

    pub fn get_ai_move(&mut self, depth: u8) -> (i32, PieceMoveDTO) {
        let result = self.mini_max.make_move(&mut self.board, depth);

        println!("Evaluated {} states", self.mini_max.states_checked);

        (result.0, piece_move_dto_from_piece_move(result.1))
    }

    pub fn get_move_generation_count(&mut self, depth: usize) -> u64 {
        move_generation_count(&mut self.board, depth)
    }

    pub fn black_captures_to_fen(&self) -> Vec<char> {
        self.board.black_captures_to_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<char> {
        self.board.white_captures_to_fen()
    }

    pub fn get_available_moves(&mut self) -> Vec<PieceDTO> {
        let mut pieces: Vec<PieceDTO> = Vec::new();

        //println!("Just entered RUST");

        for piece in self.board.get_pieces().iter().flatten() {
            pieces.push(PieceDTO::new(
                piece.get_fen(),
                piece.get_immutable_moves(),
                piece.get_position(),
                piece.is_white(),
            ));
        }
        // println!("Pieces at the very end: {:?}", pieces);
        pieces
    }

    pub fn get_winner_fen(&self) -> String {
        self.board.get_winner_fen()
    }

    pub fn is_white_move(&self) -> bool {
        self.board.is_white_move()
    }

    pub fn load_position(&mut self, fen: &str) {
        self.board.load_position(fen)
    }

    pub fn move_piece(&mut self, piece_move: PieceMoveDTO) -> PyResult<()> {
        match self.board.move_piece(PieceMove::from_dto(piece_move)) {
            Ok(()) => Ok(()),
            Err(error) => Err(exceptions::PyValueError::new_err(error)),
        }
    }
}

fn move_generation_count(board: &mut Board, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    let pieces = board.get_pieces();
    let mut num_positions: u64 = 0;

    // get_available_moves should only return the pieces, not empties
    for piece in pieces.iter().flatten() {
        for piece_move in piece.get_immutable_moves().iter() {
            if (piece.get_value() == PieceType::Empty as i8)
                || (is_white_piece(piece.get_value()) != board.is_white_move())
            {
                continue;
            }

            let _ = board.move_piece(piece_move.clone());

            num_positions += move_generation_count(board, depth - 1);

            board.undo_move();
        }
    }

    num_positions
}
