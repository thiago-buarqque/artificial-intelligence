use crate::common::piece_utils::{is_same_color, is_white_piece, PieceType, get_piece_type};

use super::board::Board;

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
pub struct MoveGenerator {}

impl MoveGenerator {
    fn get_knight_position(&self, lines_apart: i8, new_position: i8, current_position: i8) -> i8 {
        if self.get_line_distance_between_positions(
                current_position, new_position) == lines_apart {
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

    pub fn generate_knight_moves(&self, board: &Board, position: i8) -> Vec<i8> {
        let positions = [
            self.get_knight_position(2, position - 17, position),
            self.get_knight_position(2, position - 15, position),
            self.get_knight_position(1, position - 10, position),
            self.get_knight_position(1, position - 6, position),
            self.get_knight_position(1, position + 6, position),
            self.get_knight_position(1, position + 10, position),
            self.get_knight_position(2, position + 15, position),
            self.get_knight_position(2, position + 17, position),
        ];

        let mut moves = Vec::new();
        let knight_piece = board.get_piece(position);

        for position in positions {
            if board.is_valid_position(position) {
                let current_piece = board.get_piece(position);

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

    pub fn generate_king_moves(
        &self,
        board: &Board,
        opponent_moves: &[i8],
        king_position: i8,
    ) -> Vec<i8> {
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
        let king = board.get_piece(king_position);
        let pawn_offset = if is_white_piece(king_position) { -8 } else { 8 };

        for position in positions {
            if position < 0 {
                continue;
            }

            if opponent_moves.contains(&position) {
                // Is a pawn straight attacking the position?
                let mut possible_pawn = board.get_piece(position + pawn_offset);

                let mut piece_type = get_piece_type(possible_pawn);

                if piece_type == PieceType::Pawn {
                    moves.push(position);
                    continue;
                } else if piece_type != PieceType::Empty {
                    continue;
                }

                possible_pawn = board.get_piece(position + (pawn_offset * 2));

                piece_type = get_piece_type(possible_pawn);

                if piece_type == PieceType::Pawn {
                    moves.push(position);
                }
            }
            else if board.is_valid_position(position) {
                let piece = board.get_piece(position);

                if piece == PieceType::Empty as i8 || !is_same_color(king, piece) {
                    moves.push(position);
                }
            }
        }

        if !opponent_moves.contains(&king_position) {
            self.generate_castle_moves(board, king, &mut moves, opponent_moves, king_position);
        }

        moves
    }

    fn is_path_clear(&self, board: &Board, start: i8, end: i8, step: i8) -> bool {
        let mut i = start;

        while i != end {
            if board.get_piece(i) != PieceType::Empty as i8 {
                return false;
            }
            i += step;
        }
        
        true
    }

    fn position_is_not_attacked(&self, n: i8, opponent_moves: &[i8]) -> bool {
        !opponent_moves.contains(&n)
    }

    fn is_able_to_castle_queen_side(&self, board: &Board, white_king: bool) -> bool {
        (white_king && board.is_white_able_to_queen_side_castle())
            || (!white_king && board.is_black_able_to_queen_side_castle())
    }

    fn is_able_to_castle_king_side(&self, board: &Board, white_king: bool) -> bool {
        (white_king && board.is_white_able_to_king_side_castle())
            || (!white_king && board.is_black_able_to_king_side_castle())
    }

    fn generate_castle_moves(
        &self,
        board: &Board,
        king_piece: i8,
        moves: &mut Vec<i8>,
        opponent_moves: &[i8],
        position: i8,
    ) {
        let is_white_king = is_white_piece(king_piece);

        if (is_white_king && !board.white_king_moved())
            || (!is_white_king && !board.black_king_moved())
        {
            let (queen_side_rook_position, king_side_rook_position) =
                if is_white_king { (56, 63) } else { (0, 7) };

            let able_to_castle_queen_side = self.is_able_to_castle_queen_side(board, is_white_king);
            let able_to_castle_king_side = self.is_able_to_castle_king_side(board, is_white_king);

            if able_to_castle_queen_side
                && self.is_path_clear(board, position - 1, queen_side_rook_position, -1)
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
                && self.is_path_clear(board, position + 1, king_side_rook_position, 1)
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

    pub fn generate_queen_moves(&self, board: &Board, position: i8) -> Vec<i8> {
        let mut moves = vec![];

        moves.extend(self.generate_bishop_moves(board, position));
        moves.extend(self.generate_rook_moves(board, position));

        moves
    }

    pub fn generate_bishop_moves(&self, board: &Board, position: i8) -> Vec<i8> {
        let piece = board.get_piece(position);
        let mut moves = vec![];

        self.generate_sliding_moves(board, &mut moves, piece, position, SquareOffset::TopLeft);
        self.generate_sliding_moves(board, &mut moves, piece, position, SquareOffset::TopRight);
        self.generate_sliding_moves(board, &mut moves, piece, position, SquareOffset::BottomLeft);
        self.generate_sliding_moves(
            board,
            &mut moves,
            piece,
            position,
            SquareOffset::BottomRight,
        );

        moves
    }

    pub fn generate_rook_moves(&self, board: &Board, position: i8) -> Vec<i8> {
        let piece = board.get_piece(position);
        let mut moves = vec![];

        self.generate_sliding_moves(board, &mut moves, piece, position, SquareOffset::LineAbove);
        self.generate_sliding_moves(board, &mut moves, piece, position, SquareOffset::Left);
        self.generate_sliding_moves(board, &mut moves, piece, position, SquareOffset::Right);
        self.generate_sliding_moves(board, &mut moves, piece, position, SquareOffset::LineBelow);

        moves
    }

    fn generate_sliding_moves(
        &self,
        board: &Board,
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

            if !board.is_valid_position(current_position) {
                break;
            }

            let current_piece = board.get_piece(current_position);

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

    pub fn generate_pawn_moves(&self, board: &Board, position: i8) -> Vec<i8> {
        let mut moves: Vec<i8> = Vec::new();

        let white_piece = is_white_piece(board.get_piece(position));

        let offset = if white_piece { -8 } else { 8 };

        let next_line_position = position + offset;

        // Actually the pawn should be already promoted
        if !board.is_valid_position(next_line_position) {
            return moves;
        }

        self.generate_pawn_regular_moves(
            board,
            &mut moves,
            next_line_position,
            offset,
            position,
            white_piece,
        );

        self.generate_pawn_capturing_moves(
            board,
            &mut moves,
            next_line_position,
            position,
            white_piece,
        );

        self.generate_en_passant_moves(board, &mut moves, offset, position, white_piece);

        moves
    }

    fn generate_pawn_regular_moves(
        &self,
        board: &Board,
        moves: &mut Vec<i8>,
        next_line_position: i8,
        offset: i8,
        position: i8,
        white_piece: bool,
    ) {
        let existing_piece = board.get_piece(next_line_position);

        if existing_piece == PieceType::Empty as i8 {
            moves.push(next_line_position);
        }

        if self.is_pawn_first_move(white_piece, position) &&        
            get_piece_type(board.get_piece(position + offset)) == PieceType::Empty {        
            let two_lines_position = position + (offset * 2);

            let existing_piece = board.get_piece(two_lines_position);

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
        board: &Board,
        moves: &mut Vec<i8>,
        next_line_position: i8,
        position: i8,
        white_piece: bool,
        left_diagonal: bool
    ) {
        if (left_diagonal && (position % 8 == 0)) || 
            !left_diagonal && ((position + 1) % 8 == 0) {
            return;
        }

        let diagonal = next_line_position - if left_diagonal {1} else {-1};

        let existing_piece = board.get_piece(diagonal);

        if board.is_valid_position(diagonal)
            && existing_piece != PieceType::Empty as i8
            && is_white_piece(existing_piece) != white_piece
        {
            moves.push(diagonal);
        }
    }

    // Recenty refactored
    fn generate_pawn_capturing_moves(
        &self,
        board: &Board,
        moves: &mut Vec<i8>,
        next_line_position: i8,
        position: i8,
        white_piece: bool,
    ) {
        self.generate_pawn_diagonal_captures(
            board, moves, next_line_position, position, white_piece, true
        );

        self.generate_pawn_diagonal_captures(
            board, moves, next_line_position, position, white_piece, false
        );
    }

    fn generate_en_passant_moves(
        &self,
        board: &Board,
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

        let en_passant = if white_piece {
            board.get_black_en_passant()
        } else {
            board.get_white_en_passant()
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
