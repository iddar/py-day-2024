// src/views/mod.rs
use crate::state::{GRID, MESSAGE};
use crate::ui::build_cup;
use floem::event::EventListener;
use floem::prelude::*;
use floem::style::{FlexWrap, JustifyContent};
use std::process;

// Grid layout component
pub fn build_grid() -> impl View {
    let grid = GRID.with(|grid_cell| grid_cell.borrow().as_ref().unwrap().clone());
    let msg = MESSAGE.with(|message_cell| message_cell.borrow().as_ref().unwrap().clone());

    v_stack((
        label(|| msg).style(|s| s.font_size(24.0).margin_bottom(20.0)),
        dyn_stack(
            move || grid.get().chars().enumerate().collect::<Vec<_>>(),
            move |(i, _)| *i,
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

// Main application view
pub fn app_view() -> impl View {
    container(build_grid())
        .style(|s| s.size_full().items_center().justify_center())
        .on_event(EventListener::WindowClosed, move |_| {
            process::exit(0x0100);
        })
}
