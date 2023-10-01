#[derive(Debug, Clone)]
pub struct BoardPiece {
    fen: String,
    moves: Vec<i8>,
    position: i8,
    value: i8,
    white: bool,
}

impl BoardPiece {
    pub fn new(fen: String, moves: Vec<i8>, position: i8, value: i8, white: bool) -> Self {
        BoardPiece {
            fen,
            moves,
            position,
            value,
            white,
        }
    }

    pub fn get_fen(&self) -> String {
        self.fen.clone()
    }
    pub fn get_moves(&mut self) -> &mut Vec<i8> {
        &mut self.moves
    }

    pub fn get_immutable_moves(&self) -> Vec<i8> {
        self.moves.clone()
    }

    pub fn get_position(&self) -> i8 {
        self.position
    }
    pub fn get_value(&self) -> i8 {
        self.value
    }
    pub fn is_white(&self) -> bool {
        self.white
    }
}
