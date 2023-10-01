use std::sync::{Arc, Mutex};

use pyo3::{exceptions, prelude::*};

use crate::{
    ai::minimax::MiniMax,
    common::{contants::INITIAL_FEN, piece::Piece},
    game::{board::Board, board_state::BoardState, move_generator::MoveGenerator},
};

#[pyclass]
pub struct BoardWrapper {
    board: Arc<Mutex<Board>>,
    mini_max: MiniMax,
}

// is this a facade?
#[pymethods]
impl BoardWrapper {
    #[new]
    fn default() -> BoardWrapper {
        let state = Arc::new(Mutex::new(BoardState::new()));

        // let mut board = Board::new(state.clone());

        let board: Arc<Mutex<Board>> = Arc::new(Mutex::new(Board::new(state.clone())));

        let mut locked_board = board.lock().unwrap();

        locked_board.load_position(INITIAL_FEN);

        drop(locked_board);

        // board.load_position(INITIAL_FEN);

        let move_generator = MoveGenerator::new(board.clone(), state);

        locked_board = board.lock().unwrap();

        // locked_board.set_move_generator(Some(move_generator));

        drop(locked_board);

        BoardWrapper {
            board,
            mini_max: MiniMax::new(),
        }
    }

    pub fn get_ai_move(&mut self) -> (i32, (i8, i8)) {
        let result = self.mini_max.make_move(&self.board);

        println!("Evaluated {} states", self.mini_max.states_checked);

        result
    }

    pub fn get_move_generation_count(&mut self, depth: usize) -> u64 {
        1
        // move_generation_count(board, depth)
    }

    pub fn black_captures_to_fen(&self) -> Vec<String> {
        let board = self.board.lock().unwrap();

        board.black_captures_to_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<String> {
        let board = self.board.lock().unwrap();

        board.white_captures_to_fen()
    }

    pub fn get_available_moves(&mut self) -> Vec<Option<Piece>> {
        let mut pieces: Vec<Option<Piece>> = Vec::new();

        println!("Just entered RUST");

        for piece in self.board.lock().unwrap().get_pieces().iter().flatten() {
            pieces.push(Some(Piece::new(
                piece.get_fen().clone(),
                piece.get_immutable_moves(),
                piece.get_position(),
                piece.is_white(),
            )));
        }
        // println!("Pieces at the very end: {:?}", pieces);
        pieces
    }

    pub fn get_winner_fen(&self) -> String {
        let board = self.board.lock().unwrap();

        board.get_winner_fen()
    }

    pub fn get_pawn_promotion_position(&self) -> i8 {
        self.board.lock().unwrap().get_pawn_promotion_position()
    }

    pub fn is_white_move(&self) -> bool {
        let board = self.board.lock().unwrap();

        board.is_white_move()
    }

    pub fn load_position(&mut self, fen: &str) {
        let mut board = self.board.lock().unwrap();

        board.load_position(fen)
    }

    pub fn move_piece(&mut self, from_index: i8, to_index: i8) -> PyResult<()> {
        let mut board = self.board.lock().unwrap();

        match board.move_piece(from_index, to_index) {
            Ok(()) => Ok(()),
            Err(error) => Err(exceptions::PyValueError::new_err(error)),
        }
    }
}

fn move_generation_count(mut board: Board, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    // let pieces = board.get_pieces();
    let mut num_positions: u64 = 0;

    // get_available_moves should only return the pieces, not empties
    // for piece in pieces.iter().flatten() {
    //     for piece_move in piece.get_immutable_moves().iter() {
    //         if (piece.get_value() == PieceType::Empty as i8) ||
    //         (is_white_piece(piece.get_value()) != board.is_white_move()){
    //             continue;
    //         }

    //         let mut temp_board = board.clone();

    //         let _ = temp_board.move_piece(piece.get_position(), *piece_move);

    //         num_positions += move_generation_count(temp_board, depth -1);
    //     }
    // }

    num_positions
}
