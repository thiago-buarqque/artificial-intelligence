mod piece;
mod player;

use pyo3::{exceptions::PyValueError, prelude::*, types::PyList};

use piece::{piece::Piece, piece_utils};
use player::{random_player::RandomPlayer, ai_player::AI};

pub fn process_pieces(py_list: &PyAny) -> PyResult<Vec<Piece>> {
    // Try to convert PyAny to PyList

    if let Ok(py_list) = py_list.downcast::<PyList>() {
        let mut pieces: Vec<Piece> = vec![];

        // Convert PyList to Vec<Piece>
        for py_item in py_list.iter() {
            let piece: Piece = py_item.extract()?;
            pieces.push(piece);
        }

        Ok(pieces)
    } else {
        Err(PyValueError::new_err("Expected a list"))
    }
}

#[pyfunction]
fn get_ai_move(py_pieces: &PyAny, white_player: bool) -> (i32, i32) {
    let random_player = AI {};

    if let Ok(pieces) = process_pieces(py_pieces) {
        random_player.make_move(pieces, white_player)
    } else {
        (-1, -1)
    }
}

#[pymodule]
fn ai_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Piece>()?;

    m.add_function(wrap_pyfunction!(get_ai_move, m)?)?;
    Ok(())
}

fn main() {
    let piece_value = piece_utils::piece_value_from_fen(&'B');
    let fen = piece_utils::piece_fen_from_value(piece_value);
    println!("Piece value: {}", piece_value);
    println!("FEN: {}", fen);
}