use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Piece {
    #[pyo3(get, set)]
    pub fen: String,
    #[pyo3(get, set)]
    pub moves: Vec<i32>,
    #[pyo3(get, set)]
    pub position: i32,
    #[pyo3(get, set)]
    pub white: bool,
}

#[pymethods]
impl Piece {
    #[new]
    pub fn new(fen: String, moves: Vec<i32>, position: i32, white: bool) -> Self {
        Piece {
            fen,
            moves,
            position,
            white,
        }
    }

    // pub fn get_fen(&self) -> &String {
    //     &self.fen
    // }

    // pub fn get_moves(&self) -> &Vec<i32> {
    //     &self.moves
    // }

    // pub fn get_position(&self) -> &i32 {
    //     &self.position
    // }

    // pub fn is_white(&self) -> &bool {
    //     &self.white
    // }
}
