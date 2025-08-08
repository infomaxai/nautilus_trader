use pyo3::prelude::*;

#[pyfunction]
fn greet() -> PyResult<&'static str> {
    Ok("Hello from Rust!")
}

#[pymodule]
fn _hello(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}
