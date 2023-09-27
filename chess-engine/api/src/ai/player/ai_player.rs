use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use crate::piece::piece::Piece;

// Future TODOs
// [] Keep track (hashmap) of states previously computed so I dont have to evaluate them again in case they're found following another move combination
pub struct AI {

}

impl AI{
    fn new() -> Self {
        
        Self {}
    }

    pub fn make_move(&self, pieces: Vec<Piece>, white_player: bool) -> (i32, i32) {
        // Receives FEN, load in Board, get pieces and stuff.

        let mut code = String::from(include_str!("../../game/model/Board.py"));

        code.push_str("\nboard = Board()");

        Python::with_gil(|py| {
            // let module = ;

            if let Ok(module) = PyModule::from_code(py, &code, "", "") {
                if let Ok(instance) = module.getattr("board") {
                    let pieces = instance.call_method0("get_available_moves");
                    println!("{:#?}", pieces);
                }
            }
        });

        (-1, -1)
    }
}