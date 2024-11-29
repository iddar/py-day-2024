// src/lib.rs
mod app;
mod state;
mod ui;
mod views;

use app::launch_ui;
use pyo3::prelude::*;
use state::{toggle_hidden_ball, update_grid, update_message};

#[pymodule]
fn game_floem(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(launch_ui, m)?)?;
    m.add_function(wrap_pyfunction!(update_grid, m)?)?;
    m.add_function(wrap_pyfunction!(update_message, m)?)?;
    m.add_function(wrap_pyfunction!(toggle_hidden_ball, m)?)?;
    Ok(())
}
