mod ai;
mod board_wrapper;
mod common;
mod game;

// use ai::random_player::RandomPlayer;
use board_wrapper::BoardWrapper;
use pyo3::{exceptions::PyValueError, prelude::*, types::PyList};

use common::piece::Piece;

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

// #[pyfunction]
// fn get_ai_move(board: &PyAny, white_player: bool) -> (i8, i8) {
//     let random_player = RandomPlayer {};

//     if let Ok(pieces) = process_pieces(py_pieces) {
//         random_player.make_move(pieces, white_player)
//     } else {
//         (-1, -1)
//     }
// }

#[pymodule]
fn ai_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Piece>()?;
    m.add_class::<BoardWrapper>()?;

    // m.add_function(wrap_pyfunction!(get_ai_move, m)?)?;

    Ok(())
}
