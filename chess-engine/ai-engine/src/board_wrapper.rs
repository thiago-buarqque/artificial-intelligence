use pyo3::{exceptions, prelude::*};

use crate::{
    common::{contants::INITIAL_FEN, piece::Piece, piece_utils::{PieceType, is_white_piece, get_piece_type}},
    game::board::Board, ai::minimax::MiniMax,
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
        let mut board = Board::new();

        board.load_position(INITIAL_FEN);

        BoardWrapper { board, mini_max: MiniMax::new() }
    }

    pub fn get_ai_move(&mut self) -> (i32, (i8, i8)) {
        let result = self.mini_max.make_move(&self.board);

        println!("Evaluated {} states", self.mini_max.states_checked);

        result
    }

    pub fn get_move_generation_count(&mut self, depth: usize) -> u64 {
        move_generation_count(self.board.clone(), depth)
    }

    pub fn black_captures_to_fen(&self) -> Vec<String> {
        self.board.black_captures_to_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<String> {
        self.board.white_captures_to_fen()
    }

    pub fn get_available_moves(&mut self) -> Vec<Option<Piece>> {
        let mut pieces: Vec<Option<Piece>> = Vec::new();

        for piece in self.board.get_available_moves().iter() {
            if let Some(piece) = piece {
                pieces.push(
                    Some(Piece::new(
                        piece.get_fen().clone(), 
                        piece.get_immutable_moves(), 
                        piece.get_position(), 
                        piece.is_white()
                    ))
                );
            } else {
                pieces.push(None);
            }
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

    pub fn get_black_en_passant(&self) -> i8 {
        self.board.get_black_en_passant()
    }

    pub fn get_white_en_passant(&self) -> i8 {
        self.board.get_white_en_passant()
    }

    pub fn load_position(&mut self, fen: &str) {
        self.board.load_position(fen)
    }

    pub fn move_piece(&mut self, from_index: i8, to_index: i8) -> PyResult<()> {
        match self.board.move_piece(from_index, to_index) {
            Ok(()) => Ok(()),
            Err(error) => Err(exceptions::PyValueError::new_err(error)),
        }
    }
}

fn move_generation_count(mut board: Board, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let pieces = board.get_available_moves();
    let mut num_positions: u64 = 0;

    // get_available_moves should only return the pieces, not empties
    for piece in pieces.iter().flatten() {           
        for piece_move in piece.get_immutable_moves().iter() {
            if (piece.get_value() == PieceType::Empty as i8) ||
            (is_white_piece(piece.get_value()) != board.is_white_move()){
                continue;
            }

            let mut temp_board = board.clone();

            temp_board.move_piece(piece.get_position(), *piece_move);

            num_positions += move_generation_count(temp_board, depth -1);
        }
    }

    num_positions
}