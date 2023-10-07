use pyo3::{exceptions, prelude::*};

use crate::{
    ai::minimax::MiniMax,
    common::{
        contants::INITIAL_FEN,
        piece_move::PieceMove,
        piece_utils::{
            get_promotion_options, piece_fen_from_value, piece_value_from_fen, PieceType,
        },
    },
    dto::{
        dto_utils::piece_move_dto_from_piece_move, piece_dto::PieceDTO,
        piece_move_dto::PieceMoveDTO,
    },
    game::board::Board,
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
        let mut board: Board = Board::new();

        board.load_position(INITIAL_FEN);

        BoardWrapper {
            board,
            mini_max: MiniMax::new(),
        }
    }

    pub fn get_ai_move(&mut self, depth: u8) -> (i32, PieceMoveDTO) {
        let result = self.mini_max.make_move(&mut self.board, depth);

        (result.0, piece_move_dto_from_piece_move(result.1.clone()))
    }

    pub fn get_move_generation_count(&mut self, depth: usize) -> u64 {
        let nodes_searched = move_generation_count(&mut self.board, depth, true);
        println!("\nNodes searched: {}", nodes_searched);

        nodes_searched
    }

    pub fn black_captures_to_fen(&self) -> Vec<char> {
        self.board.black_captures_to_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<char> {
        self.board.white_captures_to_fen()
    }

    pub fn get_available_moves(&mut self) -> Vec<PieceDTO> {
        let mut pieces: Vec<PieceDTO> = Vec::new();

        for piece in self.board.get_pieces().iter() {
            pieces.push(PieceDTO::new(
                piece.get_fen(),
                piece.get_immutable_moves(),
                piece.get_position(),
                piece.is_white(),
            ));
        }

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

fn move_generation_count(board: &mut Board, depth: usize, track_moves: bool) -> u64 {
    if depth == 0 || board.is_game_finished() {
        return 1;
    }

    let pieces = board.get_pieces();

    let mut num_positions: u64 = 0;

    for piece in pieces.iter() {
        if (piece.get_value() == PieceType::Empty as i8)
            || (piece.is_white() != board.is_white_move())
        {
            continue;
        }

        for piece_move in piece.get_immutable_moves().iter() {
            let mut promotion_char_options = vec![piece_move.promotion_type];

            if piece_move.is_promotion {
                promotion_char_options = get_promotion_options(piece.is_white());
            }

            let mut piece_move = piece_move.clone();

            for promotion_option in promotion_char_options {
                piece_move.promotion_type = promotion_option;

                board.move_piece(piece_move.clone());

                let moves_count = move_generation_count(board, depth - 1, false);
                num_positions += moves_count;

                if track_moves {
                    if piece_move.is_promotion {
                        println!(
                            "{}{}: {}",
                            get_move_string(piece_move.clone()),
                            promotion_option.clone(),
                            moves_count
                        )
                    } else {
                        println!("{}: {}", get_move_string(piece_move.clone()), moves_count)
                    }
                }

                board.undo_move();
            }
        }
    }

    num_positions
}

fn get_position_line_number(position: i8) -> usize {
    (8 - ((position - (position % 8)) / 8)) as usize
}

fn get_position_column_number(position: i8) -> usize {
    (position - (position - (position % 8))) as usize
}

fn get_move_string(piece_move: PieceMove) -> String {
    let columns = ["a", "b", "c", "d", "e", "f", "g", "h"];

    let from_position_line = get_position_line_number(piece_move.from_position);

    let mut move_str = format!(
        "{}{}",
        columns[get_position_column_number(piece_move.from_position)],
        from_position_line
    );

    let to_position_line = get_position_line_number(piece_move.to_position);

    let to_position = format!(
        "{}{}",
        columns[get_position_column_number(piece_move.to_position)],
        to_position_line
    );

    move_str.push_str(to_position.as_str());

    move_str
}
