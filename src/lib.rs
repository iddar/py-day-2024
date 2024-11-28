use floem::{event::EventPropagation, prelude::*, reactive::RwSignal};
use pyo3::prelude::*;
use std::cell::RefCell;

thread_local! {
    static MESSAGE: RefCell<Option<RwSignal<String>>> = RefCell::new(None);
    static PY_CALLBACK: RefCell<Option<Py<PyAny>>> = RefCell::new(None);
}

/// Update the message displayed in the UI from Python.
#[pyfunction]
fn update_message(new_msg: &str) -> PyResult<()> {
    MESSAGE.with(|message_cell| {
        if let Some(ref message_signal) = *message_cell.borrow() {
            message_signal.set(new_msg.to_string());
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Message signal not initialized",
            ))
        }
    })
}

/// Run the application and pass a Python callback for the button.
#[pyfunction]
fn run_app(_py: Python<'_>, py_callback: PyObject) -> PyResult<()> {
    // Store the callback
    PY_CALLBACK.with(|callback_cell| {
        *callback_cell.borrow_mut() = Some(py_callback);
    });

    // Initialize the message signal
    MESSAGE.with(|message_cell| {
        *message_cell.borrow_mut() = Some(RwSignal::new("Hello, World!".to_string()));
    });

    // Run the UI (this will block until the UI is closed)
    floem::launch(app_view);

    Ok(())
}

/// Build the UI
fn app_view() -> impl View {
    // Get the message signal
    let message = MESSAGE.with(|message_cell| message_cell.borrow().as_ref().unwrap().clone());

    let message_label =
        label(move || message.get().clone()).style(|s| s.font_size(20.0).padding(10.0));

    let button = button("Change Message").on_click(move |_| {
        // Get the Python callback
        PY_CALLBACK.with(|callback_cell| {
            if let Some(ref py_callback) = *callback_cell.borrow() {
                // Acquire the GIL and call the Python function
                Python::with_gil(|py| {
                    if let Err(e) = py_callback.call0(py) {
                        eprintln!("Error calling Python callback: {:?}", e);
                    }
                });
            }
        });
        EventPropagation::Stop
    });

    v_stack((message_label, button.style(|s| s.margin_top(10.0))))
        .style(|s| s.size_pct(100.0, 100.0).items_center().justify_center())
}

#[pymodule]
fn my_app_ui(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_app, m)?)?;
    m.add_function(wrap_pyfunction!(update_message, m)?)?;
    Ok(())
}
