//! Drag-to-scroll across nested [`ScrollArea`]s: motion the inner area can't use goes on to the
//! enclosing one, which rubber-bands past its own edge.

use std::sync::Arc;

use egui::mutex::Mutex;
use egui::scroll_area::{DragScroll, ScrollSource, State};
use egui::{Pos2, ScrollArea, Vec2, pos2};
use egui_kittest::Harness;

#[derive(Default)]
struct States {
    outer: Option<State>,
    inner: Option<State>,
}

/// A vertical page with a horizontal row at the top that fills the first 100 points.
fn harness() -> (Harness<'static>, Arc<Mutex<States>>) {
    let states = Arc::new(Mutex::new(States::default()));
    let states_clone = Arc::clone(&states);

    let drag_only = ScrollSource {
        drag: DragScroll::Always,
        ..ScrollSource::NONE
    };
    let mut harness = Harness::builder()
        .with_size(Vec2::new(200.0, 200.0))
        .with_step_dt(1.0 / 60.0)
        .build_ui(move |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.style_mut().interaction.selectable_labels = false;
            let outer = ScrollArea::vertical()
                .auto_shrink(false)
                .scroll_source(drag_only)
                .show(ui, |ui| {
                    let inner = ScrollArea::horizontal()
                        .id_salt("row")
                        .auto_shrink(false)
                        .max_height(100.0)
                        .scroll_source(drag_only)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for i in 0..50 {
                                    ui.add_sized([60.0, 100.0], egui::Label::new(format!("{i}")));
                                }
                            });
                        });
                    states_clone.lock().inner = Some(inner.state);
                    for i in 0..100 {
                        ui.label(format!("Row {i}"));
                    }
                });
            states_clone.lock().outer = Some(outer.state);
        });

    harness.run_steps(2); // Let the scroll areas measure their content.
    (harness, states)
}

/// Press at `from`, move the pointer in steps to `to`, stepping the harness between each move.
fn drag(harness: &mut Harness<'_>, from: Pos2, to: Pos2, steps: usize) {
    harness.drag_at(from);
    harness.step();
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        harness.hover_at(from.lerp(to, t));
        harness.step();
    }
}

#[test]
fn vertical_drag_in_a_horizontal_row_scrolls_the_page() {
    let (mut harness, states) = harness();

    // Up, inside the row: the row can't scroll vertically, so the page does.
    drag(&mut harness, pos2(100.0, 80.0), pos2(100.0, 20.0), 10);
    harness.drop_at(pos2(100.0, 20.0));
    harness.run_steps(60);

    let states = states.lock();
    let outer = states.outer.unwrap().clamped_offset();
    let inner = states.inner.unwrap().clamped_offset();
    assert!(outer.y > 30.0, "the page should have scrolled, got {outer:?}");
    assert_eq!(inner.x, 0.0, "the row should not have moved");
}

#[test]
fn horizontal_drag_in_the_row_scrolls_only_the_row() {
    let (mut harness, states) = harness();

    drag(&mut harness, pos2(150.0, 50.0), pos2(50.0, 50.0), 10);
    harness.drop_at(pos2(50.0, 50.0));
    harness.run_steps(60);

    let states = states.lock();
    let outer = states.outer.unwrap().clamped_offset();
    let inner = states.inner.unwrap().clamped_offset();
    assert!(inner.x > 50.0, "the row should have scrolled, got {inner:?}");
    assert_eq!(outer.y, 0.0, "the page should not have moved");
}

#[test]
fn page_rubber_bands_when_dragged_down_from_inside_the_row() {
    let (mut harness, states) = harness();

    // Down, inside the row, with the page at its top: the page is pulled past its start.
    drag(&mut harness, pos2(100.0, 20.0), pos2(100.0, 120.0), 10);
    let pulled = states.lock().outer.unwrap().unclamped_offset().y;
    assert!(pulled < 0.0, "the page should be past its top, got {pulled}");
    assert!(-100.0 < pulled, "with resistance, got {pulled}");

    harness.drop_at(pos2(100.0, 120.0));
    harness.run_steps(120);
    let settled = states.lock().outer.unwrap().unclamped_offset().y;
    assert_eq!(settled, 0.0, "the page should spring back to its top");
}
