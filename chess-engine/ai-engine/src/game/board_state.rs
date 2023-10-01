use crate::common::piece_utils::{piece_value_from_fen, pieces_to_fen, get_piece_type, PieceType};

#[derive(Debug, Clone)]
pub struct BoardState {
    black_able_to_king_side_castle: bool,
    black_able_to_queen_side_castle: bool,
    black_captures: Vec<i8>,
    black_en_passant: i8,
    black_king_moved: bool,
    full_moves: i8,
    half_moves: i8,
    is_white_move: bool,
    pawn_promotion_position: i8,
    squares: [i8; 64],
    white_able_to_king_side_castle: bool,
    white_able_to_queen_side_castle: bool,
    white_captures: Vec<i8>,
    white_en_passant: i8,
    white_king_moved: bool,
    winner: i8,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            black_able_to_king_side_castle: true,
            black_able_to_queen_side_castle: true,
            black_captures: Vec::new(),
            black_en_passant: -1,
            black_king_moved: false,
            full_moves: 0,
            half_moves: 0,
            is_white_move: true,
            pawn_promotion_position: -1,
            squares: [0; 64],
            white_able_to_king_side_castle: true,
            white_able_to_queen_side_castle: true,
            white_captures: Vec::new(),
            white_en_passant: -1,
            white_king_moved: false,
            winner: 0,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            black_able_to_king_side_castle: self.black_able_to_king_side_castle,
            black_able_to_queen_side_castle: self.black_able_to_queen_side_castle,
            black_captures: self.black_captures.clone(),
            black_en_passant: self.black_en_passant,
            black_king_moved: self.black_king_moved,
            full_moves: self.full_moves,
            half_moves: self.half_moves,
            is_white_move: self.is_white_move,
            pawn_promotion_position: self.pawn_promotion_position,
            squares: self.squares,
            white_able_to_king_side_castle: self.white_able_to_king_side_castle,
            white_able_to_queen_side_castle: self.white_able_to_queen_side_castle,
            white_captures: self.white_captures.clone(),
            white_en_passant: self.white_en_passant,
            white_king_moved: self.white_king_moved,
            winner: self.winner,
        }
    }

    pub fn white_captures_to_fen(&self) -> Vec<String> {
        pieces_to_fen(&self.white_captures)
    }

    pub fn black_captures_to_fen(&self) -> Vec<String> {
        pieces_to_fen(&self.black_captures)
    }

    pub fn get_piece(&self, index: i8) -> i8 {
        if self.is_valid_position(index) {
            return self.squares[index as usize];
        }

        -1
    }

    pub fn place_piece(&mut self, index: i8, piece: i8) {
        if get_piece_type(self.get_piece(index)) == PieceType::King {
            println!("Someone is replacing the king")
        }
        self.squares[index as usize] = piece;
    }

    pub fn is_valid_position(&self, index: i8) -> bool {
        index >= 0 && index < self.squares.len() as i8
    }

    pub fn load_position(&mut self, fen_position: &str) {
        let fields: Vec<&str> = fen_position.split_whitespace().collect();

        self.generate_pieces_from_fen(fields[0]);
        self.load_active_color(fields[1]);
        self.load_castling(fields[2]);
        self.load_en_passant(fields[3]);

        // Ideally every fen shoul have all fields, but sometimes
        // I copy some that don't.
        if fields.len() > 4 {
            self.load_half_move_clock(fields[4]);

            if fields.len() > 5 {
                self.load_full_move_number(fields[5]);
            }
        }
    }

    fn load_half_move_clock(&mut self, half_move: &str) {
        if let Ok(value) = half_move.parse::<i8>() {
            self.half_moves = value;
        } else {
            self.half_moves = 0;
        }
    }

    fn load_full_move_number(&mut self, moves: &str) {
        if let Ok(value) = moves.parse::<i8>() {
            self.full_moves = value;
        } else {
            self.full_moves = 0;
        }
    }

    fn load_en_passant(&mut self, en_passant: &str) {
        if en_passant == "-" {
            self.white_en_passant = -1;
            self.black_en_passant = -1;
        } else {
            let column = en_passant.chars().nth(0).unwrap();
            let row: i32 = en_passant.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;

            let mut is_white = false;
            let row = if row == 3 {
                is_white = true;
                4
            } else {
                3
            };

            const LETTER_A_UNICODE: u8 = b'a';
            let position = (column as u8 - LETTER_A_UNICODE + (row * 8)) - 8;

            if is_white {
                self.white_en_passant = position as i8;
                self.black_en_passant = -1;
            } else {
                self.black_en_passant = position as i8;
                self.white_en_passant = -1;
            }
        }
    }

    fn load_castling(&mut self, castling: &str) {
        if castling == "-" {
            self.black_able_to_queen_side_castle = false;
            self.black_able_to_king_side_castle = false;
            self.white_able_to_queen_side_castle = false;
            self.white_able_to_king_side_castle = false;
            self.black_king_moved = true;
            self.white_king_moved = true;
        } else {
            self.white_able_to_king_side_castle = castling.contains('K');
            self.white_able_to_queen_side_castle = castling.contains('Q');
            self.black_able_to_king_side_castle = castling.contains('k');
            self.black_able_to_queen_side_castle = castling.contains('q');

            self.white_king_moved = false;
            self.black_king_moved = false;
        }
    }

    fn load_active_color(&mut self, active_color: &str) {
        match active_color {
            "w" => self.is_white_move = true,
            "b" => self.is_white_move = false,
            _ => self.is_white_move = true,
        }
    }

    fn generate_pieces_from_fen(&mut self, board_rows: &str) {
        let rows: Vec<&str> = board_rows.split('/').collect();
        let mut index = 0;
        for row in rows.iter() {
            for char in row.chars() {
                if char.is_numeric() {
                    index += char.to_digit(10).unwrap() as usize;
                } else {
                    self.squares[index] = piece_value_from_fen(&char);
                    index += 1;
                }
            }
        }
    }

    // Getters
    pub fn black_able_to_king_side_castle(&self) -> bool {
        self.black_able_to_king_side_castle
    }

    pub fn black_able_to_queen_side_castle(&self) -> bool {
        self.black_able_to_queen_side_castle
    }

    pub fn black_captures(&self) -> &Vec<i8> {
        &self.black_captures
    }

    pub fn black_en_passant(&self) -> i8 {
        self.black_en_passant
    }

    pub fn black_king_moved(&self) -> bool {
        self.black_king_moved
    }

    pub fn full_moves(&self) -> i8 {
        self.full_moves
    }

    pub fn half_moves(&self) -> i8 {
        self.half_moves
    }

    pub fn is_white_move(&self) -> bool {
        self.is_white_move
    }

    pub fn squares(&self) -> &[i8; 64] {
        &self.squares
    }

    pub fn get_pawn_promotion_position(&self) -> i8 {
        self.pawn_promotion_position
    }

    pub fn white_able_to_king_side_castle(&self) -> bool {
        self.white_able_to_king_side_castle
    }

    pub fn white_able_to_queen_side_castle(&self) -> bool {
        self.white_able_to_queen_side_castle
    }

    pub fn white_captures(&self) -> &Vec<i8> {
        &self.white_captures
    }

    pub fn white_en_passant(&self) -> i8 {
        self.white_en_passant
    }

    pub fn white_king_moved(&self) -> bool {
        self.white_king_moved
    }

    pub fn winner(&self) -> i8 {
        self.winner
    }

    // Setters
    pub fn set_winner(&mut self, value: i8) {
        self.winner = value;
    }

    pub fn set_is_white_move(&mut self, value: bool) {
        self.is_white_move = value;
    }

    pub fn increment_full_moves(&mut self) {
        self.full_moves += 1;
    }

    pub fn append_black_capture(&mut self, piece_value: i8) {
        self.black_captures.push(piece_value)
    }

    pub fn append_white_capture(&mut self, piece_value: i8) {
        self.white_captures.push(piece_value)
    }

    pub fn set_black_en_passant(&mut self, value: i8) {
        self.black_en_passant = value;
    }

    pub fn set_white_en_passant(&mut self, value: i8) {
        self.white_en_passant = value;
    }

    pub fn set_black_king_moved(&mut self, value: bool) {
        self.black_king_moved = value;
    }

    pub fn set_white_king_moved(&mut self, value: bool) {
        self.white_king_moved = value;
    }

    pub fn set_white_able_to_king_side_castle(&mut self, value: bool) {
        self.white_able_to_king_side_castle = value;
    }

    pub fn set_pawn_promotion_position(&mut self, pawn_promotion_position: i8) {
        self.pawn_promotion_position = pawn_promotion_position
    }

    pub fn set_white_able_to_queen_side_castle(&mut self, value: bool) {
        self.white_able_to_queen_side_castle = value;
    }

    pub fn set_black_able_to_king_side_castle(&mut self, value: bool) {
        self.black_able_to_king_side_castle = value;
    }

    pub fn set_black_able_to_queen_side_castle(&mut self, value: bool) {
        self.black_able_to_queen_side_castle = value;
    }

    pub fn update_castling_ability(&mut self, index: i8, is_black: bool, is_king_side: bool) {
        match (index, is_black, is_king_side) {
            (0, true, false) => self.black_able_to_queen_side_castle = false,
            (7, true, true) => self.black_able_to_king_side_castle = false,
            (56, false, false) => self.white_able_to_queen_side_castle = false,
            (63, false, true) => self.white_able_to_king_side_castle = false,
            _ => {}
        }
    }
}
