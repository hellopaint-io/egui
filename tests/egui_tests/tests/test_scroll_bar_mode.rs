//! Global [`egui::style::ScrollStyle::bar_mode`]: interactive / visual-only / hidden.

use egui::epaint::Shape;
use egui::style::{ScrollBarMode, ScrollStyle};
use egui::{Event, PointerButton, Pos2, RawInput, Rect, ScrollArea, Vec2};

const SCREEN: Vec2 = Vec2::new(200.0, 200.0);

/// Inside the scroll bar column, since the area below fills the screen width.
const BAR_X: f32 = 199.0;

struct Probe {
    /// Scroll offset after pressing on the bar and dragging down.
    offset: f32,
    /// Widths of the rects painted in the bar column on the last frame.
    bar_widths: Vec<f32>,
}

/// Press on the vertical scroll bar and drag down the height of the area.
fn drag_bar(mode: ScrollBarMode) -> Probe {
    let ctx = egui::Context::default();
    ctx.all_styles_mut(|style| style.spacing.scroll.bar_mode = mode);

    let frame = |events: Vec<Event>| RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
        events,
        ..Default::default()
    };
    let mut frames = vec![
        frame(vec![Event::PointerMoved(Pos2::new(BAR_X, 20.0))]),
        frame(vec![Event::PointerButton {
            pos: Pos2::new(BAR_X, 20.0),
            button: PointerButton::Primary,
            pressed: true,
            modifiers: Default::default(),
        }]),
    ];
    frames.extend((1..=4).map(|i| {
        frame(vec![Event::PointerMoved(Pos2::new(
            BAR_X,
            20.0 + 40.0 * i as f32,
        ))])
    }));
    // Let the hover/show animations settle before sampling the painted width.
    frames.extend((0..30).map(|_| frame(vec![])));

    let mut probe = Probe {
        offset: 0.0,
        bar_widths: vec![],
    };
    for input in frames {
        let output = ctx.run_ui(input, |ui| {
            let out = ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                for i in 0..100 {
                    ui.label(format!("row {i}"));
                }
            });
            probe.offset = out.state.offset.y;
        });

        let mut rects = vec![];
        for clipped in &output.shapes {
            collect_rects(&clipped.shape, &mut rects);
        }
        probe.bar_widths = rects
            .iter()
            .filter(|rect| rect.max.x >= SCREEN.x - 1.0 && rect.height() > 5.0)
            .map(|rect| rect.width())
            .collect();
    }
    probe
}

fn collect_rects(shape: &Shape, out: &mut Vec<Rect>) {
    match shape {
        Shape::Rect(rect) => out.push(rect.rect),
        Shape::Vec(shapes) => shapes.iter().for_each(|s| collect_rects(s, out)),
        _ => {}
    }
}

#[test]
fn interactive_bar_can_be_dragged() {
    let probe = drag_bar(ScrollBarMode::Interactive);
    assert!(
        probe.offset > 0.0,
        "dragging the bar should scroll, but offset stayed {}",
        probe.offset
    );
    assert!(!probe.bar_widths.is_empty(), "the bar should be painted");
}

#[test]
fn visual_only_bar_is_painted_but_ignores_drags() {
    let probe = drag_bar(ScrollBarMode::VisualOnly);
    assert_eq!(
        probe.offset, 0.0,
        "a visual-only bar must not react to drags"
    );
    assert!(
        !probe.bar_widths.is_empty(),
        "a visual-only bar is still painted"
    );
}

#[test]
fn visual_only_bar_does_not_expand_on_hover() {
    let interactive = drag_bar(ScrollBarMode::Interactive);
    let visual_only = drag_bar(ScrollBarMode::VisualOnly);
    let widest = |widths: &[f32]| widths.iter().copied().fold(0.0, f32::max);
    assert!(
        widest(&visual_only.bar_widths) < widest(&interactive.bar_widths),
        "a bar you can't grab shouldn't grow to invite you to grab it: \
         visual-only {:?} vs interactive {:?}",
        visual_only.bar_widths,
        interactive.bar_widths
    );
}

#[test]
fn hidden_bar_is_not_painted_and_ignores_drags() {
    let probe = drag_bar(ScrollBarMode::Hidden);
    assert_eq!(probe.offset, 0.0, "a hidden bar must not react to drags");
    assert!(
        probe.bar_widths.is_empty(),
        "a hidden bar must not be painted, got widths {:?}",
        probe.bar_widths
    );
}

#[test]
fn hidden_bar_allocates_no_space() {
    let mut style = ScrollStyle::solid();
    assert!(style.allocated_width() > 0.0);

    style.bar_mode = ScrollBarMode::Hidden;
    assert_eq!(style.allocated_width(), 0.0);
}
