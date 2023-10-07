use crate::common::{
    board_piece::BoardPiece,
    piece_move::PieceMove,
    piece_utils::{
        get_piece_type, is_same_color, is_white_piece, piece_fen_from_value, PieceColor, PieceType,
    },
};

use super::{
    board::Board,
    board_state::BoardState,
    contants::{
        BLACK_KING_ROOK_POS, BLACK_QUEEN_ROOK_POS, WHITE_KING_ROOK_POS, WHITE_QUEEN_ROOK_POS,
    },
    move_generator_helper::{
        get_king_move, get_knight_move, is_path_clear, is_pawn_first_move, position_is_not_attacked,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SquareOffset {
    LineBelow = 8,
    LineAbove = -8,
    TopRight = -7,
    TopLeft = -9,
    BottomRight = 9,
    BottomLeft = 7,
    Left = -1,
    Right = 1,
}

impl SquareOffset {
    fn value(&self) -> i8 {
        *self as i8
    }
}

#[derive(Debug, Clone)]
pub struct MoveGenerator {
    board_state: BoardState,
}

impl MoveGenerator {
    pub fn new(board_state: BoardState) -> Self {
        Self { board_state }
    }

    fn generate_moves(&self, piece_type: PieceType, position: i8) -> Vec<PieceMove> {
        // King moves are generated after generating the other pieces moves
        match piece_type {
            PieceType::Bishop => self.generate_bishop_moves(position),
            PieceType::Knight => self.generate_knight_moves(position),
            PieceType::Pawn => self.generate_pawn_moves(position),
            PieceType::Queen => self.generate_queen_moves(position),
            PieceType::Rook => self.generate_rook_moves(position),
            _ => vec![],
        }
    }

    // TODO from a state to the next one, only one piece moves. No need to regenreate all
    // pieces moves, only those who moved or where attacking the square where was the piece
    pub fn get_available_moves(&mut self, board: &mut Board) -> Vec<Option<BoardPiece>> {
        let (black_moves, white_moves, mut pieces) = self.generate_pieces_moves(board);

        // Only the two kings on the board
        if black_moves.is_empty() && white_moves.is_empty() {
            // Game ends in draw
            board.set_winner(false, false /* Doesn't matter*/);
        } else {
            self.get_king_available_moves(&black_moves, &mut pieces, &white_moves);

            self.remove_locked_and_invalid_moves(&mut pieces, board);
        }

        pieces
    }

    fn generate_pieces_moves(
        &mut self,
        board: &mut Board,
    ) -> (Vec<PieceMove>, Vec<PieceMove>, Vec<Option<BoardPiece>>) {
        let mut black_moves: Vec<PieceMove> = Vec::new();
        let mut white_moves: Vec<PieceMove> = Vec::new();
        let mut pieces: Vec<Option<BoardPiece>> = vec![];

        let squares = board.get_state_reference().squares();

        for (position, &piece_value) in squares.iter().enumerate() {
            let mut moves = Vec::new();
            let white_piece = is_white_piece(piece_value);
            let piece_type = get_piece_type(piece_value);

            if piece_type != PieceType::Empty && piece_type != PieceType::King {
                moves = self.generate_moves(piece_type, position as i8);

                if white_piece {
                    white_moves.extend(moves.clone());
                } else {
                    black_moves.extend(moves.clone());
                }
            }

            pieces.push(Some(BoardPiece::new(
                piece_fen_from_value(piece_value),
                moves.clone(),
                position as i8,
                piece_value,
                white_piece,
            )));
        }
        (black_moves, white_moves, pieces)
    }

    fn get_king_available_moves(
        &self,
        black_moves: &[PieceMove],
        board_pieces: &mut [Option<BoardPiece>],
        white_moves: &[PieceMove],
    ) {
        let white_king_position = self.board_state.get_white_king_position();
        let black_king_position = self.board_state.get_black_king_position();

        let mut white_king_moves = self.generate_king_raw_moves(black_moves, white_king_position);
        let mut black_king_moves = self.generate_king_raw_moves(white_moves, black_king_position);

        let common_moves: Vec<PieceMove> = white_king_moves
            .iter()
            .cloned()
            .filter(|x| black_king_moves.contains(x))
            .collect();

        white_king_moves.retain(|x| !common_moves.contains(x));
        black_king_moves.retain(|x| !common_moves.contains(x));

        // TODO the king should always be present or am i going to allow FEN with no kings?
        if let Some(ref mut white_king) = board_pieces[white_king_position as usize] {
            white_king.set_moves(white_king_moves)
        }

        if let Some(ref mut black_king) = board_pieces[black_king_position as usize] {
            black_king.set_moves(black_king_moves)
        }
    }

    pub fn remove_locked_and_invalid_moves(
        &mut self,
        pieces: &mut [Option<BoardPiece>],
        board: &mut Board,
    ) {
        let (is_white_move, king_position) = self.determine_move_and_king_position();
        let mut player_moves: Vec<PieceMove> = Vec::new();
        let is_king_in_check = self.is_king_in_check(pieces, king_position, is_white_move);

        for board_piece in pieces.iter_mut().flatten() {
            self.filter_invalid_moves(
                board_piece,
                king_position,
                board,
                &mut player_moves,
                is_white_move,
            );
        }

        if player_moves.is_empty() {
            board.set_winner(is_king_in_check, is_white_move);
        }
    }

    fn determine_move_and_king_position(&self) -> (bool, i8) {
        let is_white_move = self.board_state.is_white_move();

        let king_position = if is_white_move {
            self.board_state.get_white_king_position()
        } else {
            self.board_state.get_black_king_position()
        };

        (is_white_move, king_position)
    }

    fn is_king_in_check(
        &self,
        pieces: &[Option<BoardPiece>],
        king_position: i8,
        is_white_move: bool,
    ) -> bool {
        for board_piece in pieces.iter().flatten() {
            if board_piece.get_value() == PieceType::Empty as i8 {
                continue;
            }
            if board_piece.is_white() != is_white_move
                && board_piece
                    .get_immutable_moves()
                    .iter()
                    .any(|m| m.to_position == king_position)
            {
                return true;
            }
        }
        false
    }

    fn filter_invalid_moves(
        &mut self,
        board_piece: &mut BoardPiece,
        king_position: i8,
        board: &mut Board,
        player_moves: &mut Vec<PieceMove>,
        is_white_move: bool,
    ) {
        let piece_value = board_piece.get_value();

        if piece_value == PieceType::Empty as i8 || board_piece.is_white() != is_white_move {
            return;
        }

        let mut invalid_moves = Vec::new();

        for piece_move in board_piece.get_immutable_moves() {
            let is_king = get_piece_type(piece_value) == PieceType::King;

            self.validate_move(
                piece_move,
                king_position,
                board,
                &mut invalid_moves,
                is_king,
                piece_value,
            );
        }

        board_piece
            .get_moves()
            .retain(|x| !invalid_moves.contains(x));

        player_moves.extend(board_piece.get_immutable_moves());
    }

    fn validate_move(
        &mut self,
        piece_move: PieceMove,
        king_position: i8,
        board: &mut Board,
        invalid_moves: &mut Vec<PieceMove>,
        is_king: bool,
        piece_value: i8,
    ) {
        let mut piece_move_clone = piece_move.clone();
        let target_position = if is_king {
            piece_move.to_position
        } else {
            king_position
        };

        if piece_move_clone.is_promotion {
            piece_move_clone.promotion_type = piece_value;
        }

        board.move_piece(piece_move_clone);

        if self.is_king_attacked_on_state(target_position, board.get_state_clone()) {
            invalid_moves.push(piece_move);
        }

        board.undo_move();
    }

    fn is_king_attacked_on_state(
        &mut self,
        king_position: i8,
        new_board_state: BoardState,
    ) -> bool {
        let old_board_state = self.board_state.clone();

        // Set new state for the functions that reads it to generate the moves
        self.board_state = new_board_state;

        let is_white_move = self.board_state.is_white_move();

        let squares = self.board_state.squares();

        for (position, &piece) in squares.iter().enumerate() {
            if (piece == (PieceType::Empty as i8)) || is_white_piece(piece) != is_white_move {
                continue;
            }

            let piece_type = get_piece_type(piece);

            let moves = if piece_type != PieceType::King {
                self.generate_moves(piece_type, position as i8)
            } else {
                // Generates the current player king possible moves, even if they're invalid.
                // This is just to prevent kings to be aside with each other
                self.generate_king_raw_moves(&Vec::new(), position as i8)
            };

            if moves.iter().any(|_move| _move.to_position == king_position) {
                self.board_state = old_board_state;

                return true;
            }
        }

        // Set the old state
        self.board_state = old_board_state;

        false
    }

    pub fn generate_knight_moves(&self, position: i8) -> Vec<PieceMove> {
        let positions = [
            get_knight_move(2, position - 17, position),
            get_knight_move(2, position - 15, position),
            get_knight_move(1, position - 10, position),
            get_knight_move(1, position - 6, position),
            get_knight_move(1, position + 6, position),
            get_knight_move(1, position + 10, position),
            get_knight_move(2, position + 15, position),
            get_knight_move(2, position + 17, position),
        ];

        let mut moves: Vec<PieceMove> = Vec::new();

        let knight_piece = self.board_state.get_piece(position);

        for new_position in positions {
            if self.board_state.is_valid_position(new_position) {
                let current_piece = self.board_state.get_piece(new_position);

                if current_piece == PieceType::Empty as i8
                    || !is_same_color(knight_piece, current_piece)
                {
                    moves.push(PieceMove::new(position, new_position));
                }
            }
        }

        moves
    }

    pub fn generate_king_raw_moves(
        &self,
        opponent_moves: &[PieceMove],
        king_position: i8,
    ) -> Vec<PieceMove> {
        let positions = [
            get_king_move(king_position, king_position - 1),
            get_king_move(king_position, king_position + 1),
            get_king_move(king_position, king_position - 9),
            get_king_move(king_position, king_position - 8),
            get_king_move(king_position, king_position - 7),
            get_king_move(king_position, king_position + 7),
            get_king_move(king_position, king_position + 8),
            get_king_move(king_position, king_position + 9),
        ];

        let mut moves: Vec<PieceMove> = Vec::new();

        let king = self.board_state.get_piece(king_position);

        for position in positions {
            if position < 0 || !self.board_state.is_valid_position(position) {
                continue;
            }

            if let Some(attacking_move) = opponent_moves
                .iter()
                .find(|_move| _move.to_position == position)
            {
                let possible_pawn = self.board_state.get_piece(attacking_move.from_position);

                // Is a pawn straight attacking the position?
                // The king can move in front of the pawn
                if get_piece_type(possible_pawn) == PieceType::Pawn {
                    moves.push(PieceMove::new(king_position, position));
                    continue;
                }
            } else {
                let piece = self.board_state.get_piece(position);

                if piece == PieceType::Empty as i8 || !is_same_color(king, piece) {
                    moves.push(PieceMove::new(king_position, position));
                }
            }
        }

        if !opponent_moves
            .iter()
            .any(|_move| _move.to_position == king_position)
        {
            self.generate_castle_moves(king, &mut moves, opponent_moves, king_position);
        }

        moves
    }

    fn generate_castle_moves(
        &self,
        king_piece: i8,
        moves: &mut Vec<PieceMove>,
        opponent_moves: &[PieceMove],
        king_position: i8,
    ) {
        let is_white_king = is_white_piece(king_piece);
        let has_king_moved = if is_white_king {
            self.board_state.white_king_moved()
        } else {
            self.board_state.black_king_moved()
        };

        if has_king_moved {
            return;            
        }

        let (queen_side_rook_position, king_side_rook_position) = if is_white_king {
            (WHITE_QUEEN_ROOK_POS, WHITE_KING_ROOK_POS)
        } else {
            (BLACK_QUEEN_ROOK_POS, BLACK_KING_ROOK_POS)
        };

        let able_to_castle_queen_side =
            self.board_state.is_able_to_castle_queen_side(is_white_king);
        let able_to_castle_king_side =
            self.board_state.is_able_to_castle_king_side(is_white_king);

        let pawn_value = if is_white_king {
            PieceColor::Black as i8 | PieceType::Pawn as i8
        } else {
            PieceColor::White as i8 | PieceType::Pawn as i8
        };

        // Check for queen side castling
        if able_to_castle_queen_side
            && is_path_clear(
                &self.board_state,
                king_position - 1,
                queen_side_rook_position,
                -1,
            )
        {
            self.try_castle(
                is_white_king,
                king_position,
                false,
                king_position - 2,
                pawn_value,
                opponent_moves,
                moves,
            );
        }

        // Check for king side castling
        if able_to_castle_king_side
            && is_path_clear(
                &self.board_state,
                king_position + 1,
                king_side_rook_position,
                1,
            )
        {
            self.try_castle(
                is_white_king,
                king_position,
                true,
                king_position + 2,
                pawn_value,
                opponent_moves,
                moves,
            );
        }
    }

    fn try_castle(
        &self,
        is_white_king: bool,
        king_position: i8,
        is_king_side: bool,
        new_position: i8,
        pawn_value: i8,
        opponent_moves: &[PieceMove],
        moves: &mut Vec<PieceMove>,
    ) {
        if position_is_not_attacked(new_position, opponent_moves)
            && position_is_not_attacked(
                king_position + if new_position < king_position { -1 } else { 1 },
                opponent_moves,
            )
        {
            let offset = if is_white_king { -8 } else { 8 };

            let frontal_pawn_position = king_position + offset;
            let mut side_diagonal_pawn_position = if is_king_side {
                frontal_pawn_position + 2
            } else {
                frontal_pawn_position - 2
            };


            // Check if there is pawn attacking the side squares
            if self.board_state.get_piece(frontal_pawn_position) != pawn_value
                && self.board_state.get_piece(side_diagonal_pawn_position) != pawn_value
            {
                moves.push(PieceMove::new(king_position, new_position));
            }
        }
    }

    pub fn generate_queen_moves(&self, position: i8) -> Vec<PieceMove> {
        let mut moves = vec![];

        moves.extend(self.generate_bishop_moves(position));
        moves.extend(self.generate_rook_moves(position));

        moves
    }

    pub fn generate_bishop_moves(&self, position: i8) -> Vec<PieceMove> {
        let piece = self.board_state.get_piece(position);

        let mut moves: Vec<PieceMove> = vec![];

        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::TopLeft);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::TopRight);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::BottomLeft);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::BottomRight);

        moves
    }

    pub fn generate_rook_moves(&self, position: i8) -> Vec<PieceMove> {
        let piece = self.board_state.get_piece(position);

        let mut moves = vec![];

        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::LineAbove);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::Left);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::Right);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::LineBelow);

        moves
    }

    fn generate_sliding_moves(
        &self,
        moves: &mut Vec<PieceMove>,
        piece: i8,
        position: i8,
        offset: SquareOffset,
    ) {
        for i in 0..7 {
            // Should go somewhere on the right but is on the right side of the board
            if (offset == SquareOffset::BottomRight
                || offset == SquareOffset::TopRight
                || offset == SquareOffset::Right)
                && (position + 1) % 8 == 0
            {
                break;
            }

            // Should go somewhere on the left but is on the left side of the board
            if (offset == SquareOffset::BottomLeft
                || offset == SquareOffset::TopLeft
                || offset == SquareOffset::Left)
                && position % 8 == 0
            {
                break;
            }

            let new_position = position + ((i + 1) as i8 * offset.value());

            if !self.board_state.is_valid_position(new_position) {
                break;
            }

            let existing_piece = self.board_state.get_piece(new_position);

            if existing_piece == PieceType::Empty as i8 {
                moves.push(PieceMove::new(position, new_position));
            } else if !is_same_color(piece, existing_piece) {
                moves.push(PieceMove::new(position, new_position));
                break;
            } else {
                break;
            }

            if offset != SquareOffset::LineAbove && offset != SquareOffset::LineBelow {
                let righty_offset = offset == SquareOffset::Right
                    || offset == SquareOffset::TopRight
                    || offset == SquareOffset::BottomRight;

                // Arrived at the edge of the board
                if (new_position + if righty_offset { 1 } else { 0 }) % 8 == 0 {
                    break;
                }
            }
        }
    }

    pub fn generate_pawn_moves(&self, position: i8) -> Vec<PieceMove> {
        let white_piece = is_white_piece(self.board_state.get_piece(position));

        let offset = if white_piece { -8 } else { 8 };

        let next_line_position = position + offset;

        // The pawn should be already promoted
        if !self.board_state.is_valid_position(next_line_position) {
            return Vec::new();
        }

        let mut moves: Vec<PieceMove> = Vec::new();

        self.generate_pawn_regular_moves(
            &mut moves,
            next_line_position,
            offset,
            position,
            white_piece,
        );

        self.generate_pawn_capturing_moves(&mut moves, next_line_position, position, white_piece);

        self.generate_en_passant_moves(&mut moves, offset, position, white_piece);

        moves
    }

    fn generate_pawn_regular_moves(
        &self,
        moves: &mut Vec<PieceMove>,
        next_line_position: i8,
        offset: i8,
        position: i8,
        white_piece: bool,
    ) {
        let existing_piece = self.board_state.get_piece(next_line_position);

        if existing_piece == PieceType::Empty as i8 {
            moves.push(PieceMove::new(position, next_line_position));

            if is_pawn_first_move(white_piece, position) {
                let two_lines_position = position + (offset * 2);

                let existing_piece = self.board_state.get_piece(two_lines_position);

                if existing_piece == PieceType::Empty as i8 {
                    moves.push(PieceMove::new(position, two_lines_position));
                }
            } else if (0..=7).contains(&next_line_position)
                || (56..=63).contains(&next_line_position)
            {
                let last_index_pos = moves.len() - 1;

                moves[last_index_pos].set_is_promotion(true);
            }
        }
    }

    fn generate_pawn_diagonal_captures(
        &self,
        moves: &mut Vec<PieceMove>,
        next_line_position: i8,
        position: i8,
        white_piece: bool,
        left_diagonal: bool,
    ) {
        if (left_diagonal && (position % 8 == 0)) || !left_diagonal && ((position + 1) % 8 == 0) {
            return;
        }

        let diagonal = next_line_position - if left_diagonal { 1 } else { -1 };

        let existing_piece = self.board_state.get_piece(diagonal);

        if self.board_state.is_valid_position(diagonal)
            && existing_piece != PieceType::Empty as i8
            && is_white_piece(existing_piece) != white_piece
        {
            moves.push(PieceMove::new(position, diagonal));

            if !(0..=7).contains(&diagonal) && !(56..=63).contains(&diagonal) {
                return;
            }

            let last_index_pos = moves.len() - 1;

            moves[last_index_pos].set_is_promotion(true);
        }
    }

    fn generate_pawn_capturing_moves(
        &self,
        moves: &mut Vec<PieceMove>,
        next_line_position: i8,
        position: i8,
        white_piece: bool,
    ) {
        self.generate_pawn_diagonal_captures(
            moves,
            next_line_position,
            position,
            white_piece,
            true,
        );

        self.generate_pawn_diagonal_captures(
            moves,
            next_line_position,
            position,
            white_piece,
            false,
        );
    }

    fn generate_en_passant_moves(
        &self,
        moves: &mut Vec<PieceMove>,
        offset: i8,
        position: i8,
        white_piece: bool,
    ) {
        let en_passant = if white_piece {
            self.board_state.black_en_passant()
        } else {
            self.board_state.white_en_passant()
        };

        if en_passant == -1 {
            return;
        }

        let left_square = if position % 8 != 0 { position - 1 } else { -1 };

        let right_square = if (position + 1) % 8 != 0 {
            position + 1
        } else {
            -1
        };

        let en_passant_target = if white_piece {
            en_passant + 8
        } else {
            en_passant - 8
        };

        let side_square;

        if left_square == en_passant_target {
            side_square = left_square;
        } else if right_square == en_passant_target {
            side_square = right_square;
        } else {
            return;
        }

        let mut _move = PieceMove::new(position, side_square + offset);

        _move.is_en_passant = true;

        moves.push(_move);
    }
}
// 890 lines
