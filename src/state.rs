// src/state.rs
use floem::prelude::*;
use floem::reactive::RwSignal;
use pyo3::prelude::*;
use std::cell::RefCell;

// Global state management
thread_local! {
    pub static GRID: RefCell<Option<RwSignal<String>>> = RefCell::new(None);
    pub static MESSAGE: RefCell<Option<RwSignal<String>>> = RefCell::new(None);
    pub static CELL_CALLBACK: RefCell<Option<Py<PyAny>>> = RefCell::new(None);
    pub static HIDDEN_BALL: RefCell<Option<bool>> = RefCell::new(None);
}

#[pyfunction]
pub fn update_grid(new_grid: &str) -> PyResult<()> {
    GRID.with(|grid_cell| {
        if let Some(ref grid_signal) = *grid_cell.borrow() {
            grid_signal.set(new_grid.to_string());
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Grid not initialized",
            ))
        }
    })
}

#[pyfunction]
pub fn update_message(new_message: &str) -> PyResult<()> {
    MESSAGE.with(|message_cell| {
        if let Some(ref message_signal) = *message_cell.borrow() {
            message_signal.set(new_message.to_string());
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Message not initialized",
            ))
        }
    })
}

#[pyfunction]
pub fn toggle_hidden_ball() -> PyResult<bool> {
    HIDDEN_BALL.with(|hidden_ball_cell| {
        if let Some(ref mut hidden_ball) = *hidden_ball_cell.borrow_mut() {
            *hidden_ball = !*hidden_ball;
            Ok(*hidden_ball)
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Hidden ball not initialized",
            ))
        }
    })
}
