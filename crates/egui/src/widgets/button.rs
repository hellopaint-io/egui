use epaint::Margin;

use crate::{
    Align2, Atom, AtomExt as _, AtomKind, AtomLayout, AtomLayoutResponse, Atoms, Color32,
    CornerRadius,
    Frame, Image, IntoAtoms, NumExt as _, Response, Sense, Stroke, TextStyle, TextWrapMode, Ui,
    Vec2, Widget, WidgetInfo, WidgetText, WidgetType,
    widget_style::{ButtonStyle, Classes, HasClasses, SELECTED_CLASS, WidgetState},
};

/// Clickable button with text.
///
/// See also [`Ui::button`].
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// # fn do_stuff() {}
///
/// if ui.add(egui::Button::new("Click me")).clicked() {
///     do_stuff();
/// }
///
/// // A greyed-out and non-interactive button:
/// if ui.add_enabled(false, egui::Button::new("Can't click this")).clicked() {
///     unreachable!();
/// }
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Button<'a> {
    layout: AtomLayout<'a>,
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    small: bool,
    frame: Option<bool>,
    frame_when_inactive: bool,
    min_size: Vec2,
    corner_radius: Option<CornerRadius>,
    selected: Option<bool>,
    image_tint_follows_text_color: bool,
    limit_image_size: bool,
    classes: Classes,
    accessibility_label: Option<String>,
    accessibility_label_tooltip: bool,
}

impl<'a> Button<'a> {
    pub fn new(atoms: impl IntoAtoms<'a>) -> Self {
        Self {
            layout: AtomLayout::new(atoms.into_atoms())
                .sense(Sense::click())
                .fallback_font(TextStyle::Button),
            fill: None,
            stroke: None,
            small: false,
            frame: None,
            frame_when_inactive: true,
            min_size: Vec2::ZERO,
            corner_radius: None,
            selected: None,
            image_tint_follows_text_color: false,
            limit_image_size: false,
            classes: Classes::default(),
            accessibility_label: None,
            accessibility_label_tooltip: false,
        }
    }

    /// Show a selectable button.
    ///
    /// Equivalent to:
    /// ```rust
    /// # use egui::{Button, IntoAtoms, __run_test_ui};
    /// # __run_test_ui(|ui| {
    /// let selected = true;
    /// ui.add(Button::new("toggle me").selected(selected).frame_when_inactive(!selected).frame(true));
    /// # });
    /// ```
    ///
    /// See also:
    ///   - [`Ui::selectable_value`]
    ///   - [`Ui::selectable_label`]
    pub fn selectable(selected: bool, atoms: impl IntoAtoms<'a>) -> Self {
        Self::new(atoms)
            .selected(selected)
            .frame_when_inactive(selected)
            .frame(true)
    }

    /// Creates a button with an image. The size of the image as displayed is defined by the provided size.
    ///
    /// Note: In contrast to [`Button::new`], this limits the image size to the default font height
    /// (using [`crate::AtomExt::atom_max_height_font_size`]).
    pub fn image(image: impl Into<Image<'a>>) -> Self {
        Self::opt_image_and_text(Some(image.into()), None)
    }

    /// Creates a button with an image to the left of the text.
    ///
    /// Note: In contrast to [`Button::new`], this limits the image size to the default font height
    /// (using [`crate::AtomExt::atom_max_height_font_size`]).
    pub fn image_and_text(image: impl Into<Image<'a>>, text: impl Into<WidgetText>) -> Self {
        Self::opt_image_and_text(Some(image.into()), Some(text.into()))
    }

    /// Create a button with an optional image and optional text.
    ///
    /// Note: In contrast to [`Button::new`], this limits the image size to the default font height
    /// (using [`crate::AtomExt::atom_max_height_font_size`]).
    pub fn opt_image_and_text(image: Option<Image<'a>>, text: Option<WidgetText>) -> Self {
        let mut button = Self::new(());
        if let Some(image) = image {
            button.layout.push_right(image);
        }
        if let Some(text) = text {
            button.layout.push_right(text);
        }
        button.limit_image_size = true;
        button
    }

    /// Set the wrap mode for the text.
    ///
    /// By default, [`crate::Ui::wrap_mode`] will be used, which can be overridden with [`crate::Style::wrap_mode`].
    ///
    /// Note that any `\n` in the text will always produce a new line.
    #[inline]
    pub fn wrap_mode(mut self, wrap_mode: TextWrapMode) -> Self {
        self.layout = self.layout.wrap_mode(wrap_mode);
        self
    }

    /// Set [`Self::wrap_mode`] to [`TextWrapMode::Wrap`].
    #[inline]
    pub fn wrap(self) -> Self {
        self.wrap_mode(TextWrapMode::Wrap)
    }

    /// Set [`Self::wrap_mode`] to [`TextWrapMode::Truncate`].
    #[inline]
    pub fn truncate(self) -> Self {
        self.wrap_mode(TextWrapMode::Truncate)
    }

    /// Override background fill color. Note that this will override any on-hover effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = Some(fill.into());
        self
    }

    /// Override button stroke. Note that this will override any on-hover effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self.frame = Some(true);
        self
    }

    /// Make this a small button, suitable for embedding into text.
    #[inline]
    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }

    /// Turn off the frame
    #[inline]
    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = Some(frame);
        self
    }

    /// If `false`, the button will not have a frame when inactive.
    ///
    /// Default: `true`.
    ///
    /// Note: When [`Self::frame`] (or `ui.visuals().button_frame`) is `false`, this setting
    /// has no effect.
    #[inline]
    pub fn frame_when_inactive(mut self, frame_when_inactive: bool) -> Self {
        self.frame_when_inactive = frame_when_inactive;
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.layout = self.layout.sense(sense);
        self
    }

    /// Set the minimum size of the button.
    #[inline]
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    /// Set the rounding of the button.
    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    /// If true, the tint of the image is multiplied by the widget text color.
    ///
    /// This makes sense for images that are white, that should have the same color as the text color.
    /// This will also make the icon color depend on hover state.
    ///
    /// Default: `false`.
    #[inline]
    pub fn image_tint_follows_text_color(mut self, image_tint_follows_text_color: bool) -> Self {
        self.image_tint_follows_text_color = image_tint_follows_text_color;
        self
    }

    /// Show some text on the right side of the button, in weak color.
    ///
    /// Designed for menu buttons, for setting a keyboard shortcut text (e.g. `Ctrl+S`).
    ///
    /// The text can be created with [`crate::Context::format_shortcut`].
    ///
    /// See also [`Self::right_text`].
    #[inline]
    pub fn shortcut_text(mut self, shortcut_text: impl IntoAtoms<'a>) -> Self {
        self.layout.push_right(Atom::grow());

        for mut atom in shortcut_text.into_atoms() {
            atom.kind = match atom.kind {
                AtomKind::Text(text) => AtomKind::Text(text.weak()),
                other => other,
            };
            self.layout.push_right(atom);
        }

        self
    }

    /// Show some text on the left side of the button.
    #[inline]
    pub fn left_text(mut self, left_text: impl IntoAtoms<'a>) -> Self {
        self.layout.push_left(Atom::grow());

        for atom in left_text.into_atoms() {
            self.layout.push_left(atom);
        }

        self
    }

    /// Show some text on the right side of the button.
    #[inline]
    pub fn right_text(mut self, right_text: impl IntoAtoms<'a>) -> Self {
        self.layout.push_right(Atom::grow());

        for atom in right_text.into_atoms() {
            self.layout.push_right(atom);
        }

        self
    }

    /// If `true`, mark this button as "selected".
    ///
    /// Calling this method opts the button into toggle semantics and the
    /// current pressed/not-pressed state will be reported to assistive
    /// technologies (e.g. screen readers). Plain buttons that never call
    /// `selected` are not announced as toggles.
    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    /// The name assistive technology reads for this button.
    ///
    /// Buttons are normally named by their own text; an icon-only button has
    /// none, so it has to say what it does here.
    #[inline]
    pub fn accessibility_label(mut self, label: impl Into<String>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    /// Also show [`Self::accessibility_label`] as a hover tooltip.
    ///
    /// Sighted users need the same hint an icon-only button gives a screen
    /// reader, and keeping them one string keeps the two from drifting apart.
    #[inline]
    pub fn accessibility_label_tooltip(mut self, tooltip: bool) -> Self {
        self.accessibility_label_tooltip = tooltip;
        self
    }

    /// Set the gap between atoms.
    #[inline]
    pub fn gap(mut self, gap: f32) -> Self {
        self.layout = self.layout.gap(gap);
        self
    }

    /// Output the button's [`Atoms`].
    ///
    /// This includes any images you have on the button.
    pub fn atoms(&self) -> &Atoms<'a> {
        &self.layout.atoms
    }

    /// The button's [`Atoms`], for pushing extra content on either side.
    ///
    /// Unlike [`Self::left_text`] / [`Self::right_text`] this doesn't insert an
    /// [`Atom::grow`], so the added atom sits next to the existing content
    /// instead of being pushed to the far edge.
    pub fn atoms_mut(&mut self) -> &mut Atoms<'a> {
        &mut self.layout.atoms
    }

    /// How to align the button's content inside its frame.
    ///
    /// Defaults to the alignment of the surrounding layout.
    #[inline]
    pub fn align2(mut self, align2: Align2) -> Self {
        self.layout = self.layout.align2(align2);
        self
    }

    /// Show the button and return a [`AtomLayoutResponse`] for painting custom contents.
    pub fn atom_ui(self, ui: &mut Ui) -> AtomLayoutResponse {
        let Button {
            mut layout,
            fill,
            stroke,
            small,
            frame,
            frame_when_inactive,
            mut min_size,
            corner_radius,
            selected,
            image_tint_follows_text_color,
            limit_image_size,
            mut classes,
            accessibility_label,
            accessibility_label_tooltip,
        } = self;

        if limit_image_size {
            layout.map_atoms(|atom| {
                if matches!(&atom.kind, AtomKind::Image(_)) {
                    atom.atom_max_height_font_size(ui)
                } else {
                    atom
                }
            });
        }

        let text = accessibility_label
            .clone()
            .or_else(|| layout.text().map(String::from));

        let has_frame_margin = frame.unwrap_or_else(|| ui.visuals().button_frame);

        let id = ui.next_auto_id();
        let response: Option<Response> = ui.ctx().read_response(id);
        let state = response.map(|r| r.widget_state()).unwrap_or_default();

        classes.add_class_if(SELECTED_CLASS, selected.unwrap_or(false));

        let ButtonStyle {
            frame,
            text_style,
            min_size: style_min_size,
            force_text_style,
        } = ui.widget_style(id, &classes);

        // Min size height always equal or greater than interact size if not
        // small — unless the theme named a height itself, in which case that is
        // the one the design system wants.
        if !small && style_min_size.y <= 0.0 {
            min_size.y = min_size.y.at_least(ui.spacing().interact_size.y);
        }

        // The theme's minimum is a floor, not a cap: an explicit
        // `Button::min_size` still wins when it asks for more.
        min_size = min_size.max(style_min_size);

        let mut button_padding = if has_frame_margin {
            frame.inner_margin
        } else {
            Margin::ZERO
        };

        if small {
            button_padding.bottom = 0;
            button_padding.top = 0;
        }

        // Override global style by local style
        let mut frame = frame;
        if let Some(fill) = fill {
            frame = frame.fill(fill);
        }
        if let Some(corner_radius) = corner_radius {
            frame = frame.corner_radius(corner_radius);
        }
        if let Some(stroke) = stroke {
            frame = frame.stroke(stroke);
        }

        frame = frame.inner_margin(button_padding);

        // Apply the style font and color as fallback
        layout = layout
            .fallback_font(text_style.font_id.clone())
            .fallback_text_color(text_style.color);

        // The fallbacks above only reach the button's own atoms, and a plain
        // `WidgetText::Text` bakes in `Visuals::override_text_color` before it
        // ever consults one. A theme that owns the whole button asks for its
        // text style to be pushed onto the `Ui` instead, so it also reaches
        // nested `AtomLayout`s and can't be beaten by an app-wide override.
        // Anything that names its own size, family or color still wins.
        let pushed_text_style = force_text_style.then(|| {
            let color = ui
                .visuals_mut()
                .override_text_color
                .replace(text_style.color);
            let font_id = ui
                .style_mut()
                .override_font_id
                .replace(text_style.font_id.clone());
            (color, font_id)
        });

        // Retrocompatibility with button settings
        layout = if has_frame_margin && (state != WidgetState::Inactive || frame_when_inactive) {
            layout.frame(frame)
        } else {
            layout.frame(Frame::new().inner_margin(frame.inner_margin))
        };

        let mut prepared = layout.min_size(min_size).allocate(ui);

        // Get AtomLayoutResponse, empty if not visible
        let response = if ui.is_rect_visible(prepared.response.rect) {
            if image_tint_follows_text_color {
                prepared.map_images(|image| image.tint(text_style.color));
            }

            prepared.fallback_text_color = text_style.color;

            prepared.paint(ui)
        } else {
            AtomLayoutResponse::empty(prepared.response)
        };

        if let Some((color, font_id)) = pushed_text_style {
            ui.visuals_mut().override_text_color = color;
            ui.style_mut().override_font_id = font_id;
        }

        if let Some(cursor) = ui.visuals().interact_cursor
            && response.response.hovered()
        {
            ui.ctx().set_cursor_icon(cursor);
        }

        response.response.widget_info(|| match (selected, &text) {
            (Some(selected), Some(text)) => {
                WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), selected, text)
            }
            (Some(selected), None) => {
                let mut info = WidgetInfo::new(WidgetType::Button);
                info.enabled = ui.is_enabled();
                info.selected = Some(selected);
                info
            }
            (None, Some(text)) => WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), text),
            (None, None) => WidgetInfo::new(WidgetType::Button),
        });

        if accessibility_label_tooltip
            && let Some(label) = accessibility_label
        {
            crate::Tooltip::for_enabled(&response.response).show(|ui| ui.label(label));
        }

        response
    }
}

impl Widget for Button<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.atom_ui(ui).response
    }
}

impl HasClasses for Button<'_> {
    fn classes(&self) -> &Classes {
        &self.classes
    }

    fn classes_mut(&mut self) -> &mut Classes {
        &mut self.classes
    }
}
