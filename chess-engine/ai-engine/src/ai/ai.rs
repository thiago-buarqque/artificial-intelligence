use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn fibonnaci(x: i64) -> i64 {
    if x < 2 {
        return x
    }

    fibonnaci(x - 1) + fibonnaci(x - 2)
}

/// A Python module implemented in Rust.
#[pymodule]
fn ai_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonnaci, m)?)?;
    Ok(())
}