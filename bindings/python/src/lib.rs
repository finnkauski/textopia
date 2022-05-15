use pyo3::prelude::*;
use textopia::*;

/// Example function that would clean the whole of the input
#[pyfunction]
fn clean(input: &str) -> PyResult<String> {
    // TODO: uber unsafe here and messy
    Ok(to_string(replace(tokens(input).unwrap().1)))
}

/// This module contains the wrappers of example Rust crate for text processing
#[pymodule]
fn pytextopia(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(clean, m)?)?;
    Ok(())
}
