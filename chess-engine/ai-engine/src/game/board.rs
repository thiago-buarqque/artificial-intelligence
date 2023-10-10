use crate::common::{
    board_piece::BoardPiece,
    piece_move::PieceMove,
    piece_utils::{get_piece_type, is_piece_of_type, is_white_piece, PieceColor, PieceType},
};

use super::{board_state::BoardState, move_generator::MoveGenerator};

#[derive(Debug, Clone)]
pub struct Board {
    state: BoardState,
    state_history: Vec<BoardState>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: BoardState::new(),
            state_history: Vec::new(),
        }
    }

    pub fn get_state_reference(&self) -> &BoardState {
        &self.state
    }

    fn get_move_generator(&self) -> MoveGenerator {
        MoveGenerator::new(self.state.clone())
    }

    pub fn get_pieces(&mut self) -> Vec<BoardPiece> {
        let mut move_generator = self.get_move_generator();

        move_generator.get_available_moves(self)
    }

    pub fn set_winner(&mut self, is_king_in_check: bool, is_white_move: bool) {
        self.state.set_winner(if is_king_in_check {
            if is_white_move {
                PieceColor::Black as i8
            } else {
                PieceColor::White as i8
            }
        } else {
            PieceColor::Black as i8 | PieceColor::White as i8 // Draw
        });
    }

    fn place_piece(&mut self, index: i8, piece: i8) {
        if self.state.is_valid_position(index) {
            let existing_piece = self.state.get_piece(index);

            if !is_piece_of_type(existing_piece, PieceType::Empty) {
                let is_white = is_white_piece(existing_piece);

                if is_white {
                    self.state.append_black_capture(existing_piece);
                } else {
                    self.state.append_white_capture(existing_piece);
                }

                // This shouldn't happen
                if is_piece_of_type(existing_piece, PieceType::King) {
                    if is_white {
                        self.state.set_winner(PieceColor::Black as i8);
                    } else {
                        self.state.set_winner(PieceColor::White as i8);
                    }
                }
            }

            self.state.place_piece(index, piece);
        }
    }

    pub fn get_winner_fen(&self) -> char {
        match self.state.winner() {
            x if x == (PieceColor::White as i8) => 'w',
            x if x == (PieceColor::Black as i8) => 'b',
            x if x == (PieceColor::Black as i8 | PieceColor::White as i8) => 'd', // draw
            _ => '-',
        }
    }

    pub fn move_piece(&mut self, piece_move: &PieceMove) -> Result<(), &'static str> {
        self.state_history.push(self.state.clone());

        self._move_piece(piece_move, false)
    }

    pub fn undo_move(&mut self) {
        if let Some(state) = self.state_history.pop() {
            self.state = state
        }
    }

    pub fn get_state_clone(&self) -> BoardState {
        self.state.clone()
    }

    fn _move_piece(
        &mut self,
        piece_move: &PieceMove,
        rook_castling: bool,
    ) -> Result<(), &'static str> {
        let from_index = piece_move.from_position;
        let to_index = piece_move.to_position;

        if self.state.is_valid_position(from_index) && self.state.is_valid_position(to_index) {
            let mut moving_piece = self.state.get_piece(from_index);
            let replaced_piece = self.state.get_piece(to_index);

            if moving_piece == PieceType::Empty as i8 {
                return Err("No piece at the position");
            } else if get_piece_type(replaced_piece) == PieceType::King {
                println!("Can't capture king!");
                return Err("Can't capture king at position");
            }

            if self.is_en_passant_capture(moving_piece, to_index) {
                self.capture_en_passant(moving_piece);
            } else if piece_move.is_promotion {
                if piece_move.promotion_type == 0 {
                    return Err("Pawn needs promotion type.");
                }

                moving_piece = piece_move.promotion_type;
            } else if moving_piece == (PieceColor::White as i8 | PieceType::King as i8)
                || moving_piece == (PieceColor::Black as i8 | PieceType::King as i8)
            {
                self.handle_king_move(from_index, moving_piece, to_index);
            }

            self.place_piece(to_index, moving_piece);

            self.state.place_piece(from_index, PieceType::Empty as i8);

            if !rook_castling {
                self.register_en_passant(from_index, moving_piece, to_index);

                if !self.state.is_white_move() {
                    self.state.increment_full_moves();
                }

                let is_white_move = self.state.is_white_move();

                self.state.set_is_white_move(!is_white_move);

                // Remove hook ability due to rook move
                if get_piece_type(moving_piece) == PieceType::Rook {
                    self.state.update_castling_ability(
                        from_index,
                        from_index < 8,
                        from_index % 8 == 7,
                    );
                } else if get_piece_type(replaced_piece) == PieceType::Rook {
                    self.state
                        .update_castling_ability(to_index, to_index < 8, to_index % 8 == 7);
                }
            }

            return Ok(());
        }

        Err("Invalid board position")
    }

    fn handle_king_move(&mut self, from_index: i8, moving_piece: i8, to_index: i8) {
        let white_piece = is_white_piece(moving_piece);

        let is_castle_move = (from_index - to_index).abs() == 2;
        if is_castle_move
            && ((white_piece && !self.state.white_king_moved())
                || (!white_piece && !self.state.black_king_moved()))
        {
            self.castle(from_index, self.state.get_piece(moving_piece), to_index, white_piece);
        }

        if white_piece {
            self.state.set_white_king_moved(true);
        } else {
            self.state.set_black_king_moved(true);
        }
    }

    fn castle(
        &mut self,
        from_index: i8,
        rook_value: i8,
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

        if white_piece {
            self.state.set_white_able_to_queen_side_castle(false);
            self.state.set_white_able_to_king_side_castle(false);
        } else {
            self.state.set_black_able_to_queen_side_castle(false);
            self.state.set_black_able_to_king_side_castle(false);
        }

        let rook_move = PieceMove::new(rook_position, rook_value, new_rook_position);

        self._move_piece(&rook_move, true)
    }

    fn capture_en_passant(&mut self, moving_piece: i8) {
        let white_piece = is_white_piece(moving_piece);

        if white_piece {
            let position = self.state.black_en_passant() + 8;
            let piece_value = self.state.get_piece(position);

            self.state.append_white_capture(piece_value);
            self.state.place_piece(position, PieceType::Empty as i8);
            self.state.set_black_en_passant(-1);
        } else {
            let position = self.state.white_en_passant() - 8;
            let piece_value = self.state.get_piece(position);

            self.state.append_black_capture(piece_value);
            self.state.place_piece(position, PieceType::Empty as i8);
            self.state.set_white_en_passant(-1);
        }
    }

    fn is_en_passant_capture(&self, piece: i8, to_index: i8) -> bool {
        if is_piece_of_type(piece, PieceType::Pawn) {
            let white_piece = is_white_piece(piece);

            let en_passant = if white_piece {
                self.state.black_en_passant()
            } else {
                self.state.white_en_passant()
            };

            return en_passant != -1 && to_index == en_passant;
        }

        false
    }

    fn register_en_passant(&mut self, from_index: i8, piece_value: i8, to_index: i8) {
        self.state.set_white_en_passant(-1);
        self.state.set_black_en_passant(-1);

        if is_piece_of_type(piece_value, PieceType::Pawn) {
            let white_piece = is_white_piece(piece_value);

            if white_piece {
                // magic numbers...
                if (48..=55).contains(&from_index) && (32..=39).contains(&to_index) {
                    self.state.set_white_en_passant(to_index + 8);
                }
            } else {
                // magic numbers...
                if (8..=15).contains(&from_index) && (24..=31).contains(&to_index) {
                    self.state.set_black_en_passant(to_index - 8);
                }
            }
        }
    }

    pub fn load_position(&mut self, fen_position: &str) {
        let mut state = BoardState::new();

        state.load_position(fen_position);

        self.state = state;
    }

    pub fn black_captures_to_fen(&self) -> Vec<char> {
        self.state.black_captures_to_fen()
    }

    pub fn white_captures_to_fen(&self) -> Vec<char> {
        self.state.white_captures_to_fen()
    }

    pub fn is_white_move(&self) -> bool {
        self.state.is_white_move()
    }

    pub fn is_game_finished(&self) -> bool {
        self.get_winner_fen() != '-'
    }

    pub fn get_squares(&self) -> [i8; 64] {
        *self.state.squares()
    }
}
