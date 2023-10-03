use std::sync::{Arc, Mutex};

use crate::common::{
    board_piece::BoardPiece,
    piece_utils::{get_piece_type, is_piece_of_type, is_white_piece, PieceColor, PieceType},
};

use super::{board_state::BoardState, move_generator::MoveGenerator};

#[derive(Debug, Clone)]
pub struct Board {
    // move_generator: Option<MoveGenerator>,
    state: Arc<Mutex<BoardState>>,
    state_history: Vec<BoardState>,
}

impl Board {
    pub fn new(state: Arc<Mutex<BoardState>>) -> Self {
        Board {
            state,
            state_history: Vec::new(),
        }
    }

    pub fn get_state_reference(&self) -> &Arc<Mutex<BoardState>> {
        &self.state
    }

    fn get_move_generator(&self) -> MoveGenerator {
        MoveGenerator::new(Arc::new(Mutex::new(self.clone())), self.state.clone())
    }

    pub fn get_pieces(&mut self) -> Vec<Option<BoardPiece>> {
        let mut move_generator = self.get_move_generator();

        move_generator.get_available_moves(self)
    }

    fn place_piece(&mut self, index: i8, piece: i8) {
        let mut state = self.state.lock().unwrap();

        if state.is_valid_position(index) {
            let current_piece = state.get_piece(index);

            if !is_piece_of_type(current_piece, PieceType::Empty) {
                let is_white = is_white_piece(current_piece);
                if is_white {
                    state.append_black_capture(current_piece);
                } else {
                    state.append_white_capture(current_piece);
                }

                if is_piece_of_type(current_piece, PieceType::King) {
                    if is_white {
                        state.set_winner(PieceColor::Black as i8);
                    } else {
                        state.set_winner(PieceColor::White as i8);
                    }
                }
            }

            state.place_piece(index, piece);
        }
    }

    pub fn get_winner_fen(&self) -> String {
        match self.state.lock().unwrap().winner() {
            x if x == (PieceColor::White as i8) => String::from("w"),
            x if x == (PieceColor::Black as i8) => String::from("b"),
            x if x == (PieceColor::Black as i8 | PieceColor::White as i8) => String::from("bw"),
            _ => String::from("-"),
        }
    }

    pub fn get_pawn_promotion_position(&self) -> i8 {
        self.state.lock().unwrap().get_pawn_promotion_position()
    }

    pub fn load_state_and_clear_history(&mut self, state: Arc<Mutex<BoardState>>) {
        self.state_history = Vec::new();
        self.state = state;
    }

    pub fn move_piece(&mut self, from_index: i8, to_index: i8) -> Result<(), &'static str> {
        let state = self.state.lock().unwrap();

        self.state_history.push(state.clone());

        drop(state);

        self._move_piece(from_index, to_index, false)
    }

    pub fn undo_move(&mut self) {
        if let Some(state) = self.state_history.pop() {
            self.state = Arc::new(Mutex::new(state))
        }
    }

    pub fn get_state_clone(&self) -> BoardState {
        self.state.lock().unwrap().clone()
    }

    fn _move_piece(
        &mut self,
        from_index: i8,
        to_index: i8,
        rook_castling: bool,
    ) -> Result<(), &'static str> {
        let mut state = self.state.lock().unwrap();

        if state.is_valid_position(from_index) && state.is_valid_position(to_index) {
            let moving_piece = state.get_piece(from_index);
            let replaced_piece = state.get_piece(to_index);

            drop(state);
            if moving_piece == PieceType::Empty as i8 {
                return Err("No piece at the position");
            } else if get_piece_type(replaced_piece) == PieceType::King {
                return Err("Can't capture king at position");
            }

            if self.is_en_passant_capture(moving_piece, to_index) {
                self.capture_en_passant(moving_piece);
            } else if moving_piece == (PieceColor::White as i8 | PieceType::King as i8)
                || moving_piece == (PieceColor::Black as i8 | PieceType::King as i8)
            {
                self.handle_king_move(from_index, moving_piece, to_index);
            }

            self.place_piece(to_index, moving_piece);

            state = self.state.lock().unwrap();

            state.place_piece(from_index, PieceType::Empty as i8);

            drop(state);

            self.register_en_passant(from_index, moving_piece, to_index);

            state = self.state.lock().unwrap();

            if !state.is_white_move() {
                state.increment_full_moves();
            }

            if !rook_castling {
                let is_white_move = state.is_white_move();

                state.set_is_white_move(!is_white_move);
            }

            // Remove hook ability due to rook move
            if get_piece_type(moving_piece) == PieceType::Rook {
                state.update_castling_ability(from_index, from_index < 8, from_index % 8 == 7);
            } else if get_piece_type(replaced_piece) == PieceType::Rook {
                state.update_castling_ability(to_index, to_index < 8, to_index % 8 == 7);
            }

            return Ok(());
        }

        Err("Invalid board position")
    }

    fn handle_king_move(&mut self, from_index: i8, moving_piece: i8, to_index: i8) {
        let mut state = self.state.lock().unwrap();

        let white_piece = is_white_piece(moving_piece);

        let is_castle_move = (from_index - to_index).abs() == 2;
        if is_castle_move
            && ((white_piece && !state.white_king_moved())
                || (!white_piece && !state.black_king_moved()))
        {
            // The error will never occur, indexes are 0..=63
            drop(state);
            let _ = self.castle(from_index, to_index, white_piece);
            state = self.state.lock().unwrap();
        }

        if white_piece {
            state.set_white_king_moved(true);
        } else {
            state.set_black_king_moved(true);
        }
    }

    fn castle(
        &mut self,
        from_index: i8,
        to_index: i8,
        white_piece: bool,
    ) -> Result<(), &'static str> {
        let (queen_side_rook_position, king_side_rook_position) =
            if white_piece { (56, 63) } else { (0, 7) };

        let rook_position = if from_index > to_index {
            queen_side_rook_position
        } else {
            king_side_rook_position
        };

        let new_rook_position = if from_index > to_index {
            from_index - 1
        } else {
            from_index + 1
        };

        let mut state = self.state.lock().unwrap();

        if white_piece {
            state.set_white_able_to_queen_side_castle(false);
            state.set_white_able_to_king_side_castle(false);
        } else {
            state.set_black_able_to_queen_side_castle(false);
            state.set_black_able_to_king_side_castle(false);
        }

        drop(state);

        self._move_piece(rook_position, new_rook_position, true)
    }

    fn capture_en_passant(&mut self, moving_piece: i8) {
        let mut state = self.state.lock().unwrap();

        let white_piece = is_white_piece(moving_piece);

        if white_piece {
            let position = state.black_en_passant() + 8;
            let piece_value = state.get_piece(position);

            state.append_white_capture(piece_value);
            state.place_piece(position, PieceType::Empty as i8);
            state.set_black_en_passant(-1);
        } else {
            let position = state.white_en_passant() - 8;
            let piece_value = state.get_piece(position);

            state.append_black_capture(piece_value);
            state.place_piece(position, PieceType::Empty as i8);
            state.set_white_en_passant(-1);
        }
    }

    fn is_en_passant_capture(&self, piece: i8, to_index: i8) -> bool {
        if is_piece_of_type(piece, PieceType::Pawn) {
            let white_piece = is_white_piece(piece);

            let state = self.state.lock().unwrap();

            let en_passant = if white_piece {
                state.black_en_passant()
            } else {
                state.white_en_passant()
            };

            return en_passant != -1 && to_index == en_passant;
        }

        false
    }

    fn register_en_passant(&mut self, from_index: i8, piece_value: i8, to_index: i8) {
        if is_piece_of_type(piece_value, PieceType::Pawn) {
            let white_piece = is_white_piece(piece_value);

            let mut state = self.state.lock().unwrap();

            if white_piece {
                state.set_white_en_passant(-1);

                // magic numbers...
                if from_index > 47 && from_index < 56 && to_index > 31 && to_index < 40 {
                    state.set_white_en_passant(to_index + 8);
                }
            } else {
                state.set_black_en_passant(-1);

                // magic numbers...
                if from_index > 7 && from_index < 16 && to_index > 23 && to_index < 32 {
                    state.set_black_en_passant(to_index - 8);
                }
            }
        }
    }

    pub fn load_position(&mut self, fen_position: &str) {
        let mut state = BoardState::new();

        state.load_position(fen_position);

        self.state = Arc::new(Mutex::new(state));
    }

    pub fn black_captures_to_fen(&self) -> Vec<String> {
        self.state.lock().unwrap().black_captures_to_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<String> {
        self.state.lock().unwrap().white_captures_to_fen()
    }

    pub fn is_white_move(&self) -> bool {
        self.state.lock().unwrap().is_white_move()
    }

    pub fn is_game_finished(&self) -> bool {
        self.get_winner_fen() != "-"
    }

    pub fn get_squares(&self) -> [i8; 64] {
        *self.state.lock().unwrap().squares()
    }
}
