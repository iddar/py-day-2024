// src/ui/mod.rs
use crate::state::{CELL_CALLBACK, HIDDEN_BALL};
use floem::event::EventPropagation;
use floem::prelude::*;
use floem::{unit::UnitExt, views::img};
use pyo3::prelude::*;

// Component for individual cup buttons
pub fn build_cup(index: usize, state: char) -> impl View {
    let vaso_png = include_bytes!("./../../assets/vaso.png");
    let ball_png = include_bytes!("./../../assets/ball.png");

    let cup_content = match state {
        '0' => img(move || vaso_png.to_vec()), // Empty cup
        'x' => img(move || ball_png.to_vec()), // Ball in cup
        _ => img(move || vaso_png.to_vec()),   // Hidden ball
    };

    button(cup_content.style(|s| s.width(60.px()).height(90.px())))
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
        })
}
