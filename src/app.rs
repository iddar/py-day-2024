// src/app.rs
use crate::state::{CELL_CALLBACK, GRID, HIDDEN_BALL, MESSAGE};
use crate::views::app_view;
use floem::kurbo::Size;
use floem::reactive::RwSignal;
use floem::window::WindowConfig;
use floem::Application;
use pyo3::prelude::*;

#[pyfunction]
pub fn launch_ui(_py: Python<'_>, initial_grid: &str, callback: PyObject) -> PyResult<()> {
    // Initialize callback and grid state
    CELL_CALLBACK.with(|callback_cell| {
        *callback_cell.borrow_mut() = Some(callback);
    });

    GRID.with(|grid_cell| {
        *grid_cell.borrow_mut() = Some(RwSignal::new(initial_grid.to_string()));
    });

    MESSAGE.with(|message_cell| {
        *message_cell.borrow_mut() = Some(RwSignal::new("New Game".to_string()));
    });

    HIDDEN_BALL.with(|hidden_ball_cell| {
        *hidden_ball_cell.borrow_mut() = Some(true);
    });

    // Launch application
    Application::new()
        .window(
            |_| app_view(),
            Some(
                WindowConfig::default()
                    .size(Size::new(800.0, 600.0))
                    .title("Color Palette"),
            ),
        )
        .run();
    Ok(())
}
