use rand::Rng;

use crate::common::board_piece::BoardPiece;

#[derive(Debug)]
pub struct RandomPlayer {}

impl RandomPlayer {
    pub fn make_move(&self, pieces: Vec<BoardPiece>, white_player: bool) -> (i8, i8) {
        let mut rng = rand::thread_rng();

        let mut default_piece = &pieces[0];

        for piece in pieces.iter() {
            if white_player == piece.is_white() && !piece.get_immutable_moves().is_empty() {
                default_piece = piece;

                if rng.gen_range(1..=10) >= 8 {
                    let moves = &piece.get_immutable_moves();

                    let destination = moves[rng.gen_range(0..moves.len())];

                    return (piece.get_position(), destination);
                }
            }
        }

        let moves = &default_piece.get_immutable_moves();

        let destination = moves[rng.gen_range(0..moves.len())];

        (default_piece.get_position(), destination)
    }
}
