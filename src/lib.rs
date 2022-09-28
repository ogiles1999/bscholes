mod formula;

use pyo3::prelude::*;
use formula::add_one;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn sum_as_string_add_one(a: usize, b: usize) -> PyResult<String> {
    Ok(add_one(a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn bscholes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string_add_one, m)?)?;
    Ok(())
}
