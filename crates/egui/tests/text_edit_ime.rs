use egui::{Context, Id, RawInput, Rect, TextEdit, pos2, vec2};

/// Runs a pass with a focused single-line text edit, clipped out when `clipped`, and returns the
/// IME output.
fn ime_output(ctx: &Context, text: &mut String, clipped: bool) -> Option<egui::output::IMEOutput> {
    let input = RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(400.0, 300.0))),
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| {
        if clipped {
            ui.set_clip_rect(Rect::from_min_size(pos2(0.0, 200.0), vec2(400.0, 100.0)));
        }
        let response = ui.add(TextEdit::singleline(text).id(Id::unique("field")));
        response.request_focus();
    });
    // Nothing is painted here, so nothing consumes these.
    output.textures_delta.clear();
    output.platform_output.ime
}

/// A focused text edit keeps reporting IME output while it is clipped out, as for the frame the
/// layout moves up for an on-screen keyboard. The integration would otherwise end the text input,
/// and close that keyboard.
#[test]
fn a_clipped_out_focused_text_edit_keeps_its_ime_output() {
    let ctx = Context::default();
    let mut text = String::from("hello");
    for _ in 0..2 {
        ime_output(&ctx, &mut text, false);
    }
    assert!(ime_output(&ctx, &mut text, false).is_some());
    assert!(ime_output(&ctx, &mut text, true).is_some());
}
