use pyo3::{prelude::*, exceptions};

use crate::{game::board::Board, common::{piece::Piece, contants::INITIAL_FEN}};

#[pyclass]
pub struct BoardWrapper {
    board: Board
}

// is this a facade?
#[pymethods]
impl BoardWrapper {
    #[new]
    fn default() -> BoardWrapper {
        let mut board = Board::new();

        board.load_position(INITIAL_FEN);

        BoardWrapper {
            board
        }
    }

    pub fn black_captures_to_fen(&self) -> Vec<String> {
        self.board.black_captures_to_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<String> {
        self.board.white_captures_to_fen()
    }

    pub fn get_available_moves(&mut self) -> Vec<Option<Piece>>{
        self.board.get_available_moves()
    }

    pub fn get_winner_fen(&self) -> String {
        self.board.get_winner_fen()
    }

    pub fn is_white_move(&self) -> bool {
        self.board.is_white_move()
    }

    pub fn get_black_en_passant(&self) -> i8 {
        self.board.get_black_en_passant()
    }

    pub fn get_white_en_passant(&self) -> i8 {
        self.board.get_white_en_passant()
    }

    pub fn load_position(&mut self, fen: &str) {
        self.board.load_position(fen)
    }

    pub fn move_piece(&mut self, from_index: i8, to_index: i8) -> PyResult<()>{
        match self.board.move_piece(from_index, to_index, false) {
            Ok(()) => Ok(()),
            Err(error) => Err(exceptions::PyValueError::new_err(error))

        }
    }
}