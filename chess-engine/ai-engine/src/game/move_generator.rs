use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::common::{
    board_piece::BoardPiece,
    piece_utils::{
        get_piece_type, is_same_color, is_white_piece, piece_fen_from_value, PieceColor, PieceType,
    },
};

use super::{board::Board, board_state::BoardState};

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
    board_state: Arc<Mutex<BoardState>>,
    board: Arc<Mutex<Board>>,
}

impl MoveGenerator {
    pub fn new(board: Arc<Mutex<Board>>, board_state: Arc<Mutex<BoardState>>) -> Self {
        Self { board, board_state }
    }

    fn generate_moves(&self, piece_type: PieceType, position: i8) -> Vec<i8> {
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

    pub fn load_state(&mut self, state: Arc<Mutex<BoardState>>) {
        self.board_state = state;
    }

    pub fn get_available_moves(&mut self) -> Vec<Option<BoardPiece>> {
        let mut black_moves: Vec<i8> = Vec::new();
        let mut white_moves: Vec<i8> = Vec::new();
        let mut pieces: Vec<Option<BoardPiece>> = Vec::new();

        let mut white_king_position: i8 = -1;
        let mut black_king_position: i8 = -1;

        let squares_guard = self.board_state.lock().unwrap();

        let board_squares = squares_guard.clone();

        drop(squares_guard);

        for (position, &piece_value) in board_squares.squares().iter().enumerate() {
            let white_piece = is_white_piece(piece_value);
            let piece_type = get_piece_type(piece_value);

            if piece_type == PieceType::King {
                if white_piece {
                    white_king_position = position as i8;
                } else {
                    black_king_position = position as i8;
                }
                pieces.push(None);
            } else {
                let moves = self.generate_moves(piece_type, position as i8);

                let piece_fen = piece_fen_from_value(piece_value);
                let piece = BoardPiece::new(
                    piece_fen,
                    moves.clone(),
                    position as i8,
                    piece_value,
                    white_piece,
                );

                pieces.push(Some(piece));

                if white_piece {
                    white_moves.extend(moves);
                } else {
                    black_moves.extend(moves);
                }
            }
        }

        self.get_king_available_moves(
            black_king_position,
            &black_moves,
            &mut pieces,
            white_king_position,
            &white_moves,
        );

        self.remove_blocked_piece_moves(&mut pieces, black_king_position, white_king_position);

        pieces
    }

    fn get_king_available_moves(
        &self,
        black_king_position: i8,
        black_moves: &[i8],
        board_pieces: &mut [Option<BoardPiece>],
        white_king_position: i8,
        white_moves: &[i8],
    ) {
        let mut white_king_moves = self.generate_king_moves(black_moves, white_king_position);
        let mut black_king_moves = self.generate_king_moves(white_moves, black_king_position);

        let common_moves: Vec<i8> = white_king_moves
            .iter()
            .cloned()
            .filter(|&x| black_king_moves.contains(&x))
            .collect();

        white_king_moves.retain(|x| !common_moves.contains(x));
        black_king_moves.retain(|x| !common_moves.contains(x));

        let white_king_piece = BoardPiece::new(
            piece_fen_from_value(PieceColor::White as i8 | PieceType::King as i8),
            white_king_moves,
            white_king_position,
            self.board_state
                .lock()
                .unwrap()
                .get_piece(white_king_position),
            true,
        );

        let black_king_piece = BoardPiece::new(
            piece_fen_from_value(PieceColor::Black as i8 | PieceType::King as i8),
            black_king_moves,
            black_king_position,
            self.board_state
                .lock()
                .unwrap()
                .get_piece(black_king_position),
            false,
        );

        board_pieces[white_king_position as usize] = Some(white_king_piece);
        board_pieces[black_king_position as usize] = Some(black_king_piece);
    }

    pub fn remove_blocked_piece_moves(
        &mut self,
        pieces: &mut [Option<BoardPiece>],
        black_king_position: i8,
        white_king_position: i8,
    ) {
        let mut board_state = self.board_state.lock().unwrap();

        let king_position = if board_state.is_white_move() {
            white_king_position
        } else {
            black_king_position
        };

        let mut board = self.board.lock().unwrap();
        let mut player_moves: Vec<i8> = Vec::new();

        // Is it possible to brake the loop earlier if king is in check on the next move?
        for board_piece in pieces.iter_mut().flatten() {
            let piece_position = board_piece.get_position();
            let piece_value = board_state.get_piece(piece_position);
            let mut invalid_moves = Vec::new();

            if board_piece.is_white() != board_state.is_white_move() {
                continue;
            }

            for move_pos in board_piece.get_immutable_moves() {
                board.move_piece(piece_position, move_pos);

                let opponent_next_moves = self.get_next_moves_attack_to_king(king_position);

                if get_piece_type(piece_value) == PieceType::King
                    && opponent_next_moves.contains(&move_pos)
                {
                    invalid_moves.push(move_pos);
                    continue;
                }

                if get_piece_type(piece_value) == PieceType::King {
                    continue;
                }

                if opponent_next_moves.contains(&king_position) {
                    invalid_moves.push(move_pos);
                }

                board.undo_move();
            }

            board_piece
                .get_moves()
                .retain(|&x| !invalid_moves.contains(&x));

            if is_white_piece(piece_value) == board_state.is_white_move() {
                player_moves.extend(board_piece.get_immutable_moves());
            }
        }

        // TODO Check on when returning pieces on Board
        if player_moves.is_empty() {
            let winner = if board_state.is_white_move() {
                PieceColor::Black as i8
            } else {
                PieceColor::White as i8
            };

            board_state.set_winner(winner);
        }
    }

    fn get_next_moves_attack_to_king(&self, king_position: i8) -> Vec<i8> {
        let mut all_moves: Vec<i8> = Vec::new();

        for (position, &piece) in self
            .board_state
            .lock()
            .unwrap()
            .squares()
            .iter()
            .enumerate()
        {
            if is_white_piece(piece) != self.board_state.lock().unwrap().is_white_move() {
                continue;
            }

            let piece_type = get_piece_type(piece);

            if piece_type != PieceType::King {
                let moves = self.generate_moves(piece_type, position as i8);

                // Note for future me:
                // I can't declare moves before the if and use after the else. It is
                // moved for its declaration and can't be borrowed after the else. WHY???
                //
                // If the king is being attacked, we're abre to break the loop, no need
                // to check all piece in the board
                if moves.contains(&king_position) {
                    return all_moves;
                }

                all_moves.extend(moves);
            } else {
                let opponent_moves: Vec<i8> = Vec::new();
                // generates the current player king possible moves, even if they're invalid.
                // This is just to prevent kings to be aside with each other
                let moves = self.generate_king_moves(&opponent_moves, position as i8);

                // Note for future me:
                // I can't declare moves before the if and use after the else. It is
                // moved for its declaration and can't be borrowed after the else. WHY???
                //
                // If the king is being attacked, we're abre to break the loop, no need
                // to check all piece in the board
                if moves.contains(&king_position) {
                    return all_moves;
                }

                all_moves.extend(moves);
            }
        }

        all_moves
    }

    fn get_knight_move(&self, lines_apart: i8, new_position: i8, current_position: i8) -> i8 {
        if self.get_line_distance_between_positions(current_position, new_position) == lines_apart {
            return new_position;
        }

        -1
    }

    // Get positions line distance
    fn get_line_distance_between_positions(&self, position1: i8, position2: i8) -> i8 {
        let line_start1 = position1 - (position1 % 8);
        let line_start2 = position2 - (position2 % 8);

        if line_start1 > line_start2 {
            return (line_start1 - line_start2) / 8;
        }

        (line_start2 - line_start1) / 8
    }

    pub fn generate_knight_moves(&self, position: i8) -> Vec<i8> {
        let board_state = self.board_state.lock().unwrap();

        let positions = [
            self.get_knight_move(2, position - 17, position),
            self.get_knight_move(2, position - 15, position),
            self.get_knight_move(1, position - 10, position),
            self.get_knight_move(1, position - 6, position),
            self.get_knight_move(1, position + 6, position),
            self.get_knight_move(1, position + 10, position),
            self.get_knight_move(2, position + 15, position),
            self.get_knight_move(2, position + 17, position),
        ];

        let mut moves = Vec::new();
        let knight_piece = board_state.get_piece(position);

        for position in positions {
            if board_state.is_valid_position(position) {
                let current_piece = board_state.get_piece(position);

                if current_piece == PieceType::Empty as i8
                    || !is_same_color(knight_piece, current_piece)
                {
                    moves.push(position);
                }
            }
        }

        moves
    }

    fn get_king_move(&self, current_position: i8, new_position: i8) -> i8 {
        if !(0..=63).contains(&new_position) {
            return -1;
        }

        // Is on the left side of the board
        if current_position % 8 == 0
            && (new_position == current_position - 1
                || new_position == current_position - 9
                || new_position == current_position + 7)
        {
            return -1;
        }

        // Is on the right side of the board
        if (current_position + 1) % 8 == 0
            && (new_position == current_position + 1
                || new_position == current_position - 7
                || new_position == current_position + 9)
        {
            return -1;
        }

        new_position
    }

    pub fn generate_king_moves(&self, opponent_moves: &[i8], king_position: i8) -> Vec<i8> {
        let positions = [
            self.get_king_move(king_position, king_position - 1),
            self.get_king_move(king_position, king_position + 1),
            self.get_king_move(king_position, king_position - 9),
            self.get_king_move(king_position, king_position - 8),
            self.get_king_move(king_position, king_position - 7),
            self.get_king_move(king_position, king_position + 7),
            self.get_king_move(king_position, king_position + 8),
            self.get_king_move(king_position, king_position + 9),
        ];

        let mut moves = Vec::new();

        let board_state = self.board_state.lock().unwrap();

        let king = board_state.get_piece(king_position);
        let pawn_offset = if is_white_piece(king_position) { -8 } else { 8 };

        for position in positions {
            if position < 0 {
                continue;
            }

            if opponent_moves.contains(&position) {
                // Is a pawn straight attacking the position?
                let mut possible_pawn = board_state.get_piece(position + pawn_offset);

                let mut piece_type = get_piece_type(possible_pawn);

                if piece_type == PieceType::Pawn {
                    moves.push(position);
                    continue;
                } else if piece_type != PieceType::Empty {
                    continue;
                }

                possible_pawn = board_state.get_piece(position + (pawn_offset * 2));

                piece_type = get_piece_type(possible_pawn);

                if piece_type == PieceType::Pawn {
                    moves.push(position);
                }
            } else if board_state.is_valid_position(position) {
                let piece = board_state.get_piece(position);

                if piece == PieceType::Empty as i8 || !is_same_color(king, piece) {
                    moves.push(position);
                }
            }
        }

        if !opponent_moves.contains(&king_position) {
            self.generate_castle_moves(king, &mut moves, opponent_moves, king_position);
        }

        moves
    }

    fn is_path_clear(&self, start: i8, end: i8, step: i8) -> bool {
        let board_state = self.board_state.lock().unwrap();

        let mut i = start;

        while i != end {
            if board_state.get_piece(i) != PieceType::Empty as i8 {
                return false;
            }
            i += step;
        }

        true
    }

    fn position_is_not_attacked(&self, n: i8, opponent_moves: &[i8]) -> bool {
        !opponent_moves.contains(&n)
    }

    fn is_able_to_castle_queen_side(&self, white_king: bool) -> bool {
        let board_state = self.board_state.lock().unwrap();

        (white_king && board_state.white_able_to_queen_side_castle())
            || (!white_king && board_state.black_able_to_queen_side_castle())
    }

    fn is_able_to_castle_king_side(&self, white_king: bool) -> bool {
        let board_state = self.board_state.lock().unwrap();

        (white_king && board_state.white_able_to_king_side_castle())
            || (!white_king && board_state.black_able_to_king_side_castle())
    }

    fn generate_castle_moves(
        &self,
        king_piece: i8,
        moves: &mut Vec<i8>,
        opponent_moves: &[i8],
        position: i8,
    ) {
        let board_state = self.board_state.lock().unwrap();

        let is_white_king = is_white_piece(king_piece);

        if (is_white_king && !board_state.white_king_moved())
            || (!is_white_king && !board_state.black_king_moved())
        {
            let (queen_side_rook_position, king_side_rook_position) =
                if is_white_king { (56, 63) } else { (0, 7) };

            let able_to_castle_queen_side = self.is_able_to_castle_queen_side(is_white_king);
            let able_to_castle_king_side = self.is_able_to_castle_king_side(is_white_king);

            if able_to_castle_queen_side
                && self.is_path_clear(position - 1, queen_side_rook_position, -1)
            {
                let new_position = position - 2;

                // The next two squares on the left are not attacked
                if self.position_is_not_attacked(new_position, opponent_moves)
                    && self.position_is_not_attacked(position - 1, opponent_moves)
                {
                    moves.push(new_position);
                }
            }

            if able_to_castle_king_side
                && self.is_path_clear(position + 1, king_side_rook_position, 1)
            {
                let new_position = position + 2;

                // The next two squares on the right are not attacked
                if self.position_is_not_attacked(new_position, opponent_moves)
                    && self.position_is_not_attacked(position + 1, opponent_moves)
                {
                    moves.push(new_position);
                }
            }
        }
    }

    pub fn generate_queen_moves(&self, position: i8) -> Vec<i8> {
        let mut moves = vec![];

        moves.extend(self.generate_bishop_moves(position));
        moves.extend(self.generate_rook_moves(position));

        moves
    }

    pub fn generate_bishop_moves(&self, position: i8) -> Vec<i8> {
        let piece = self.board_state.lock().unwrap().get_piece(position);
        let mut moves = vec![];

        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::TopLeft);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::TopRight);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::BottomLeft);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::BottomRight);

        moves
    }

    pub fn generate_rook_moves(&self, position: i8) -> Vec<i8> {
        let piece = self.board_state.lock().unwrap().get_piece(position);
        let mut moves = vec![];

        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::LineAbove);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::Left);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::Right);
        self.generate_sliding_moves(&mut moves, piece, position, SquareOffset::LineBelow);

        moves
    }

    fn generate_sliding_moves(
        &self,
        moves: &mut Vec<i8>,
        piece: i8,
        position: i8,
        offset: SquareOffset,
    ) {
        for i in 0..7 {
            // Is on the right side of the board
            if (offset == SquareOffset::BottomRight || offset == SquareOffset::TopRight)
                && (position + 1) % 8 == 0
            {
                break;
            }

            // Is on the left side of the board
            if (offset == SquareOffset::BottomLeft || offset == SquareOffset::TopLeft)
                && position % 8 == 0
            {
                break;
            }

            // Should go left/right but is on the edge of the board
            if (offset == SquareOffset::Left && position % 8 == 0)
                || (offset == SquareOffset::Right && (position + 1) % 8 == 0)
            {
                break;
            }

            let current_position = position + ((i + 1) as i8 * offset.value());

            let board_state = self.board_state.lock().unwrap();

            if !board_state.is_valid_position(current_position) {
                break;
            }

            let current_piece = board_state.get_piece(current_position);

            if current_piece == PieceType::Empty as i8 {
                moves.push(current_position);
            } else if !is_same_color(piece, current_piece) {
                moves.push(current_position);
                break;
            } else {
                break;
            }

            if offset != SquareOffset::LineAbove && offset != SquareOffset::LineBelow {
                let righty_offset = offset == SquareOffset::Right
                    || offset == SquareOffset::TopRight
                    || offset == SquareOffset::BottomRight;

                // Arrived at the edge of the board
                if (current_position + if righty_offset { 1 } else { 0 }) % 8 == 0 {
                    break;
                }
            }
        }
    }

    pub fn generate_pawn_moves(&self, position: i8) -> Vec<i8> {
        let mut moves: Vec<i8> = Vec::new();

        let board_state = self.board_state.lock().unwrap();

        let white_piece = is_white_piece(board_state.get_piece(position));

        let offset = if white_piece { -8 } else { 8 };

        let next_line_position = position + offset;

        // Actually the pawn should be already promoted
        if !board_state.is_valid_position(next_line_position) {
            return moves;
        }

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
        moves: &mut Vec<i8>,
        next_line_position: i8,
        offset: i8,
        position: i8,
        white_piece: bool,
    ) {
        let board_state = self.board_state.lock().unwrap();

        let existing_piece = board_state.get_piece(next_line_position);

        if existing_piece == PieceType::Empty as i8 {
            moves.push(next_line_position);
        }

        if self.is_pawn_first_move(white_piece, position)
            && get_piece_type(board_state.get_piece(position + offset)) == PieceType::Empty
        {
            let two_lines_position = position + (offset * 2);

            let existing_piece = board_state.get_piece(two_lines_position);

            if existing_piece == PieceType::Empty as i8 {
                moves.push(two_lines_position);
            }
        }
    }

    fn is_pawn_first_move(&self, white_piece: bool, piece_position: i8) -> bool {
        if white_piece && (48..=55).contains(&piece_position) {
            return true;
        }

        if !white_piece && (8..=15).contains(&piece_position) {
            return true;
        }

        false
    }

    fn generate_pawn_diagonal_captures(
        &self,
        moves: &mut Vec<i8>,
        next_line_position: i8,
        position: i8,
        white_piece: bool,
        left_diagonal: bool,
    ) {
        if (left_diagonal && (position % 8 == 0)) || !left_diagonal && ((position + 1) % 8 == 0) {
            return;
        }

        let diagonal = next_line_position - if left_diagonal { 1 } else { -1 };

        let board_state = self.board_state.lock().unwrap();

        let existing_piece = board_state.get_piece(diagonal);

        if board_state.is_valid_position(diagonal)
            && existing_piece != PieceType::Empty as i8
            && is_white_piece(existing_piece) != white_piece
        {
            moves.push(diagonal);
        }
    }

    // Recenty refactored
    fn generate_pawn_capturing_moves(
        &self,

        moves: &mut Vec<i8>,
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

        moves: &mut Vec<i8>,
        offset: i8,
        position: i8,
        white_piece: bool,
    ) {
        let left_square = if position % 8 != 0 { position - 1 } else { -1 };

        let right_square = if (position + 1) % 8 != 0 {
            position + 1
        } else {
            -1
        };

        let board_state = self.board_state.lock().unwrap();

        let en_passant = if white_piece {
            board_state.black_en_passant()
        } else {
            board_state.white_en_passant()
        };

        if en_passant == -1 {
            return;
        }

        let en_passant_target = if white_piece {
            en_passant + 8
        } else {
            en_passant - 8
        };

        if left_square == en_passant_target {
            moves.push(left_square + offset);
        } else if right_square == en_passant_target {
            moves.push(right_square + offset);
        }
    }
}
