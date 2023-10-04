use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct PieceMoveDTO {
    #[pyo3(get)]
    pub from_position: i8,
    #[pyo3(get)]
    pub to_position: i8,
    #[pyo3(get)]
    pub promotion_type: char,
    #[pyo3(get)]
    pub is_promotion: bool,
    #[pyo3(get)]
    pub is_en_passant: bool,
}

#[pymethods]
impl PieceMoveDTO {
    #[new]
    pub fn new(
        from_position: i8,
        to_position: i8,
        promotion_type: char,
        is_promotion: bool,
        is_en_passant: bool,
    ) -> Self {
        Self {
            from_position,
            to_position,
            promotion_type,
            is_promotion,
            is_en_passant,
        }
    }
}
