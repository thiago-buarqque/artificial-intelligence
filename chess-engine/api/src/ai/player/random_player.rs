use crate::piece::piece::Piece;

use rand::Rng;

#[derive(Debug)]
pub struct RandomPlayer {}

impl RandomPlayer {
    pub fn make_move(&self, pieces: Vec<Piece>, white_player: bool) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        let mut default_piece = &pieces[0];

        for piece in pieces.iter() {
            if white_player == piece.white && !piece.moves.is_empty() {
                default_piece = piece;

                if rng.gen_range(1..=10) >= 8 {
                    let moves = &piece.moves;

                    let destination = moves[rng.gen_range(0..moves.len())];

                    return (piece.position, destination);
                }
            }
        }

        let moves = &default_piece.moves;

        let destination = moves[rng.gen_range(0..moves.len())];

        (default_piece.position, destination)
    }
}
