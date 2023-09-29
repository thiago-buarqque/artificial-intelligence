use crate::common::{
    board_piece::BoardPiece,
    piece_utils::{
        get_piece_type, is_piece_of_type, is_same_color, is_white_piece, piece_fen_from_value,
        piece_value_from_fen, pieces_to_fen, PieceColor, PieceType,
    },
};

use super::move_generator::MoveGenerator;

#[derive(Debug, Clone)]
pub struct Board {
    squares: [i8; 64],
    white_captures: Vec<i8>,
    black_captures: Vec<i8>,
    black_en_passant: i8,
    white_en_passant: i8,
    black_king_moved: bool,
    white_king_moved: bool,
    is_white_move: bool,
    half_moves: i8,
    full_moves: i8,
    is_white_in_check: bool,
    is_black_in_check: bool,
    move_generator: MoveGenerator,
    winner: i8,
    black_able_to_queen_side_castle: bool,
    black_able_to_king_side_castle: bool,
    white_able_to_queen_side_castle: bool,
    white_able_to_king_side_castle: bool,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [0; 64],
            white_captures: Vec::new(),
            black_captures: Vec::new(),
            black_en_passant: -1,
            white_en_passant: -1,
            black_king_moved: false,
            white_king_moved: false,
            is_white_move: true,
            half_moves: 0,
            full_moves: 0,
            is_white_in_check: false,
            is_black_in_check: false,
            move_generator: MoveGenerator {},
            winner: 0,
            black_able_to_queen_side_castle: true,
            black_able_to_king_side_castle: true,
            white_able_to_queen_side_castle: true,
            white_able_to_king_side_castle: true,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            squares: self.squares.clone(),
            white_captures: self.white_captures.clone(),
            black_captures: self.black_captures.clone(),
            black_en_passant: self.black_en_passant,
            white_en_passant: self.white_en_passant,
            black_king_moved: self.black_king_moved,
            white_king_moved: self.white_king_moved,
            is_white_move: self.is_white_move,
            half_moves: self.half_moves,
            full_moves: self.full_moves,
            is_white_in_check: self.is_white_in_check,
            is_black_in_check: self.is_black_in_check,
            move_generator: MoveGenerator {},
            winner: self.winner,
            black_able_to_queen_side_castle: self.black_able_to_queen_side_castle,
            black_able_to_king_side_castle: self.black_able_to_king_side_castle,
            white_able_to_queen_side_castle: self.white_able_to_queen_side_castle,
            white_able_to_king_side_castle: self.white_able_to_king_side_castle,
        }
    }

    fn reset(&mut self) {
        self.squares = [0; 64];
        self.white_captures.clear();
        self.black_captures.clear();

        self.black_en_passant = -1;
        self.white_en_passant = -1;

        self.black_king_moved = false;
        self.white_king_moved = false;

        self.is_white_move = true;
        self.half_moves = 0;
        self.full_moves = 0;

        self.is_white_in_check = false;
        self.is_black_in_check = false;

        self.winner = 0;

        self.black_able_to_queen_side_castle = true;
        self.black_able_to_king_side_castle = true;
        self.white_able_to_queen_side_castle = true;
        self.white_able_to_king_side_castle = true;
    }

    pub fn get_squares(&self) -> &[i8; 64] {
        &self.squares
    }

    pub fn is_game_finished(&self) -> bool {
        self.get_winner_fen() != "-"
    }

    pub fn get_black_en_passant(&self) -> i8 {
        self.black_en_passant
    }

    pub fn get_white_en_passant(&self) -> i8 {
        self.white_en_passant
    }

    pub fn is_white_able_to_queen_side_castle(&self) -> bool {
        self.white_able_to_queen_side_castle
    }

    pub fn is_white_able_to_king_side_castle(&self) -> bool {
        self.white_able_to_king_side_castle
    }

    pub fn is_black_able_to_queen_side_castle(&self) -> bool {
        self.black_able_to_queen_side_castle
    }

    pub fn is_black_able_to_king_side_castle(&self) -> bool {
        self.black_able_to_king_side_castle
    }

    pub fn is_white_move(&self) -> bool {
        self.is_white_move
    }

    pub fn white_king_moved(&self) -> bool {
        self.white_king_moved
    }

    pub fn black_king_moved(&self) -> bool {
        self.black_king_moved
    }

    fn generate_moves(&self, piece_type: PieceType, position: i8) -> Vec<i8> {
        // King moves are generate after generating other pieces' moves

        match piece_type {
            PieceType::Bishop => self.move_generator.generate_bishop_moves(self, position),
            PieceType::Knight => self.move_generator.generate_knight_moves(self, position),
            PieceType::Pawn => self.move_generator.generate_pawn_moves(self, position),
            PieceType::Queen => self.move_generator.generate_queen_moves(self, position),
            PieceType::Rook => self.move_generator.generate_rook_moves(self, position),
            _ => vec![],
        }
    }

    pub fn white_captures_to_fen(&self) -> Vec<String> {
        pieces_to_fen(&self.white_captures)
    }

    pub fn black_captures_to_fen(&self) -> Vec<String> {
        pieces_to_fen(&self.black_captures)
    }

    pub fn get_available_moves(&mut self) -> Vec<Option<BoardPiece>> {
        let mut black_moves: Vec<i8> = Vec::new();
        let mut white_moves: Vec<i8> = Vec::new();
        let mut pieces: Vec<Option<BoardPiece>> = Vec::new();

        let mut white_king_position: i8 = -1;
        let mut black_king_position: i8 = -1;

        for (position, &piece_value) in self.squares.iter().enumerate() {
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

    pub fn remove_blocked_piece_moves(
        &mut self,
        pieces: &mut [Option<BoardPiece>],
        black_king_position: i8,
        white_king_position: i8,
    ) {
        let king_position = if self.is_white_move {
            white_king_position
        } else {
            black_king_position
        };

        let mut player_moves: Vec<i8> = Vec::new();

        // Is it possible to brake the loop earlier if king is in check on the next move?
        for board_piece in pieces.iter_mut().flatten() {
            let piece_position = board_piece.get_position();
            let piece_value = self.squares[piece_position as usize];
            let mut invalid_moves = Vec::new();

            if board_piece.is_white() != self.is_white_move {
                continue;
            }

            if get_piece_type(piece_value) == PieceType::King {
                println!(
                    "King moves: {:?} - pos: {}",
                    board_piece.get_immutable_moves(),
                    piece_position
                );
            }

            for move_pos in board_piece.get_immutable_moves() {
                let mut temp_board = self.clone();

                temp_board.move_piece(piece_position, move_pos);

                let opponent_next_moves = temp_board.get_player_moves_from_current_board();

                if get_piece_type(piece_value) == PieceType::King {
                    println!("For king in {}: move: {} - opponents: {:?}", 
                        piece_position, move_pos, opponent_next_moves);
                }

                if get_piece_type(piece_value) == PieceType::King
                    && opponent_next_moves.contains(&move_pos)
                {
                    println!(
                        "(1) Invalidating move of {} ({}): {} - opponents: {:?}",
                        piece_fen_from_value(piece_value), piece_position,
                        move_pos,
                        opponent_next_moves
                    );
                    invalid_moves.push(move_pos);
                    continue;
                }

                if get_piece_type(piece_value) == PieceType::King {
                    continue;
                }

                if (board_piece.is_white() && opponent_next_moves.contains(&white_king_position)) ||
                    (!board_piece.is_white() && opponent_next_moves.contains(&black_king_position)){
                    println!(
                        "(2) Invalidating move of {} ({}): {} - opponents: {:?}",
                        piece_fen_from_value(piece_value), piece_position,
                        move_pos,
                        opponent_next_moves
                    );

                    invalid_moves.push(move_pos);
                }
            }

            if piece_value != 0 {
                println!(
                    "Original moves of {}: {:?} - invalids: {:?}",
                    piece_fen_from_value(piece_value),
                    board_piece.get_immutable_moves(),
                    invalid_moves
                );                
            }


            board_piece
                .get_moves()
                .retain(|&x| !invalid_moves.contains(&x));

            if is_white_piece(piece_value) == self.is_white_move {
                player_moves.extend(board_piece.get_immutable_moves());
            } else {
                println!(
                    "Not counting moves for {}, its moves were: {:?}",
                    piece_fen_from_value(piece_value),
                    board_piece.get_immutable_moves()
                )
            }
        }

        if player_moves.is_empty() {
            println!("Player moves is empty, ending game");
            self.winner = if self.is_white_move {
                PieceColor::Black as i8
            } else {
                PieceColor::White as i8
            }; // Assume BLACK and WHITE are defined
        }
    }

    fn get_player_moves_from_current_board(&self) -> Vec<i8> {
        let mut all_moves: Vec<i8> = Vec::new();

        for (position, &piece) in self.squares.iter().enumerate() {
            if is_white_piece(piece) != self.is_white_move {
                continue;
            }

            let piece_type = get_piece_type(piece);

            if piece_type != PieceType::King {
                let moves = self.generate_moves(piece_type, position as i8);
                all_moves.extend(moves);
            } else {
                let opponent_moves: Vec<i8> = Vec::new();
                // generates the current player king possible moves, even if they're invalid.
                // This is just to prevent kings to be aside with each other
                let moves = self.move_generator.generate_king_moves(self, &opponent_moves, position as i8);
                
                all_moves.extend(moves);
            }
        }

        all_moves
    }

    fn get_king_available_moves(
        &self,
        black_king_position: i8,
        black_moves: &[i8],
        board_pieces: &mut [Option<BoardPiece>],
        white_king_position: i8,
        white_moves: &[i8],
    ) {
        let mut white_king_moves =
            self.move_generator
                .generate_king_moves(self, black_moves, white_king_position);
        let mut black_king_moves =
            self.move_generator
                .generate_king_moves(self, white_moves, black_king_position);

        println!("white king moves: {:?}", white_king_moves);
        println!("black king moves: {:?}", black_king_moves);

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
            self.get_piece(white_king_position),
            true,
        );

        let black_king_piece = BoardPiece::new(
            piece_fen_from_value(PieceColor::Black as i8 | PieceType::King as i8),
            black_king_moves,
            black_king_position,
            self.get_piece(black_king_position),
            false,
        );

        board_pieces[white_king_position as usize] = Some(white_king_piece);
        board_pieces[black_king_position as usize] = Some(black_king_piece);
    }

    pub fn place_piece(&mut self, index: i8, piece: i8) {
        // Assuming you have a validate_board_index method
        match self.validate_board_index(index) {
            Ok(()) => {
                let current_piece = self.squares[index as usize];

                if !is_piece_of_type(current_piece, PieceType::Empty) {
                    let is_white = is_white_piece(current_piece);
                    if is_white {
                        self.black_captures.push(current_piece);
                    } else {
                        self.white_captures.push(current_piece);
                    }

                    if is_piece_of_type(current_piece, PieceType::King) {
                        if is_white {
                            self.winner = PieceColor::Black as i8;
                        } else {
                            self.winner = PieceColor::White as i8;
                        }
                    }
                }

                self.squares[index as usize] = piece;
            }
            Err(error) => println!("{}", error),
        }
    }

    pub fn get_winner_fen(&self) -> String {
        match self.winner {
            x if x == (PieceColor::White as i8) => String::from("w"),
            x if x == (PieceColor::Black as i8) => String::from("b"),
            _ => String::from("-"),
        }
    }

    pub fn move_piece(&mut self, from_index: i8, to_index: i8) -> Result<(), &'static str> {
        self._move_piece(from_index, to_index, false)
    }

    fn _move_piece(
        &mut self,
        from_index: i8,
        to_index: i8,
        rook_castling: bool,
    ) -> Result<(), &'static str> {
        match self.validate_board_index(from_index) {
            Ok(()) => {
                let moving_piece = self.squares[from_index as usize];
                let replaced_piece = self.get_piece(to_index);

                if moving_piece == PieceType::Empty as i8 {
                    return Err("No piece at the position ");
                }

                if self.is_en_passant_capture(moving_piece, to_index) {
                    self.capture_en_passant(moving_piece);
                } else if moving_piece == (PieceColor::White as i8 | PieceType::King as i8)
                    || moving_piece == (PieceColor::Black as i8 | PieceType::King as i8)
                {
                    self.handle_king_move(from_index, moving_piece, to_index);
                }

                self.place_piece(to_index, moving_piece);
                self.squares[from_index as usize] = PieceType::Empty as i8;

                self.register_en_passant(from_index, moving_piece, to_index);

                if !self.is_white_move {
                    self.full_moves += 1;
                }

                if !rook_castling {
                    self.is_white_move = !self.is_white_move;
                }

                // Remove ability for rook
                if get_piece_type(moving_piece) == PieceType::Rook {
                    self.update_castling_ability(from_index, from_index < 8, from_index % 8 == 7);
                } else if get_piece_type(replaced_piece) == PieceType::Rook {
                    self.update_castling_ability(to_index, to_index < 8, to_index % 8 == 7);
                }

                Ok(())
            }
            Err(message) => Err(message),
        }
    }

    fn update_castling_ability(&mut self, index: i8, is_black: bool, is_king_side: bool) {
        match (index, is_black, is_king_side) {
            (0, true, false) => self.black_able_to_queen_side_castle = false,
            (7, true, true) => self.black_able_to_king_side_castle = false,
            (56, false, false) => self.white_able_to_queen_side_castle = false,
            (63, false, true) => self.white_able_to_king_side_castle = false,
            _ => {}
        }
    }

    fn handle_king_move(&mut self, from_index: i8, moving_piece: i8, to_index: i8) {
        let white_piece = is_white_piece(moving_piece);

        let is_castle_move = (from_index - to_index).abs() == 2;
        if is_castle_move
            && ((white_piece && !self.white_king_moved) || (!white_piece && !self.black_king_moved))
        {
            // The error will never occur, indexes are 0..=63
            let _ = self.castle(from_index, to_index, white_piece);
        }

        if white_piece {
            self.white_king_moved = true;
        } else {
            self.black_king_moved = true;
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

        if white_piece {
            self.white_able_to_queen_side_castle = false;
            self.white_able_to_king_side_castle = false;
        } else {
            self.black_able_to_queen_side_castle = false;
            self.black_able_to_king_side_castle = false;
        }

        self._move_piece(rook_position, new_rook_position, true)
    }

    fn capture_en_passant(&mut self, moving_piece: i8) {
        let white_piece = is_white_piece(moving_piece);

        if white_piece {
            self.white_captures
                .push(self.squares[(self.black_en_passant + 8) as usize]);
            self.squares[(self.black_en_passant + 8) as usize] = PieceType::Empty as i8;
            self.black_en_passant = -1;
        } else {
            self.black_captures
                .push(self.squares[(self.white_en_passant - 8) as usize]);
            self.squares[(self.white_en_passant - 8) as usize] = PieceType::Empty as i8;
            self.white_en_passant = -1;
        }
    }

    fn is_en_passant_capture(&self, piece: i8, to_index: i8) -> bool {
        if is_piece_of_type(piece, PieceType::Pawn) {
            let white_piece = is_white_piece(piece);

            let en_passant = if white_piece {
                self.black_en_passant
            } else {
                self.white_en_passant
            };

            return en_passant != -1 && to_index == en_passant;
        }

        false
    }

    fn register_en_passant(&mut self, from_index: i8, piece_value: i8, to_index: i8) {
        if is_piece_of_type(piece_value, PieceType::Pawn) {
            let white_piece = is_white_piece(piece_value);

            if white_piece {
                self.white_en_passant = -1;

                // magic numbers...
                if from_index > 47 && from_index < 56 && to_index > 31 && to_index < 40 {
                    self.white_en_passant = to_index + 8;
                }
            } else {
                self.black_en_passant = -1;

                // magic numbers...
                if from_index > 7 && from_index < 16 && to_index > 23 && to_index < 32 {
                    self.black_en_passant = to_index - 8;
                }
            }
        }
    }

    pub fn get_piece(&self, index: i8) -> i8 {
        match self.validate_board_index(index) {
            Ok(()) => self.squares[index as usize],
            _ => -1,
        }
    }

    pub fn is_valid_position(&self, index: i8) -> bool {
        index >= 0 && index < self.squares.len() as i8
    }

    pub fn load_position(&mut self, fen_position: &str) {
        self.reset();

        let fields: Vec<&str> = fen_position.split_whitespace().collect();

        self.generate_pieces_from_fen(fields[0]);
        self.load_active_color(fields[1]);
        self.load_castling(fields[2]);
        self.load_en_passant(fields[3]);

        // Every fen shoul have, but sometimes in dev I copy some that doesn't.
        // Just preventing errors
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

    fn validate_board_index(&self, index: i8) -> Result<(), &'static str> {
        if index >= 0 && index < self.squares.len() as i8 {
            Ok(())
        } else {
            Err("Invalid board index")
        }
    }
}
