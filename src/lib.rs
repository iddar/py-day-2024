use floem::{
    event::EventPropagation,
    prelude::*,
    reactive::RwSignal,
    style::{FlexWrap, JustifyContent},
};
use pyo3::prelude::*;
use std::cell::RefCell;

thread_local! {
    static GRID: RefCell<Option<RwSignal<String>>> = RefCell::new(None);
    static CELL_CALLBACK: RefCell<Option<Py<PyAny>>> = RefCell::new(None);
}

#[pyfunction]
fn update_grid(new_grid: &str) -> PyResult<()> {
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
fn launch_ui(_py: Python<'_>, initial_grid: &str, callback: PyObject) -> PyResult<()> {
    CELL_CALLBACK.with(|callback_cell| {
        *callback_cell.borrow_mut() = Some(callback);
    });

    GRID.with(|grid_cell| {
        *grid_cell.borrow_mut() = Some(RwSignal::new(initial_grid.to_string()));
    });

    floem::launch(app_view);
    Ok(())
}

fn build_cup(index: usize, state: char) -> impl View {
    let cup_content = match state {
        '0' => "d", // Vaso vacío
        'x' => "e", // Vaso con bolita
        _ => "t",   // Estado desconocido
    };

    let button = button(cup_content.to_string())
        .on_click(move |_| {
            CELL_CALLBACK.with(|callback_cell| {
                if let Some(ref callback) = *callback_cell.borrow() {
                    Python::with_gil(|py| {
                        if let Err(e) = callback.call1(py, (index,)) {
                            eprintln!("Error calling Python callback: {:?}", e);
                        }
                    });
                }
            });
            EventPropagation::Stop
        })
        .style(|s| {
            s.width(80)
                .height(80)
                .font_size(32.0)
                .margin(5.0)
                .border(1.0)
                .border_radius(5.0)
                .background(Color::rgb8(240, 240, 240))
                .hover(|s| s.background(Color::rgb8(220, 220, 220)))
        });

    button
}

fn build_grid() -> impl View {
    let grid = GRID.with(|grid_cell| grid_cell.borrow().as_ref().unwrap().clone());

    v_stack((
        label(|| "¿Dónde está la bolita?").style(|s| s.font_size(24.0).margin_bottom(20.0)),
        dyn_stack(
            move || grid.get().chars().enumerate().collect::<Vec<_>>(),
            move |(i, _)| *i, // Cambiado para usar el valor directamente
            |(i, state)| build_cup(i, state),
        )
        .style(|s| {
            s.gap(10.0)
                .flex_wrap(FlexWrap::Wrap)
                .justify_content(JustifyContent::Center)
                .width_pct(100.0)
        }),
    ))
    .style(|s| s.items_center().justify_center().gap(20.0).padding(20.0))
}

fn app_view() -> impl View {
    container(build_grid()).style(|s| s.size_full().items_center().justify_center())
}

#[pymodule]
fn game_floem(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(launch_ui, m)?)?;
    m.add_function(wrap_pyfunction!(update_grid, m)?)?;
    Ok(())
}
