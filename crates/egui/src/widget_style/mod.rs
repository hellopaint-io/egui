// This module is only public with the `experimental_theme` feature,
// so without it a lot of it looks unused:
#![cfg_attr(not(feature = "experimental"), allow(dead_code, unused_imports))]

mod classes;

pub use self::classes::{ClassName, Classes, HasClasses, ROOT_CLASS, SELECTED_CLASS};

use core::fmt::Debug;

use emath::Vec2;
use epaint::{Color32, FontId, Stroke, text::TextWrapMode};

use crate::{
    Context, Frame, Response, Style, UiStack,
    style::{WidgetVisuals, Widgets},
};

/// Each dedicated style must implement this trait to be used in the theme plugin system
pub trait WidgetStyle: Debug + Clone + Send + Sync + core::any::Any + 'static {}

/// General text style
#[derive(Debug, Clone)]
pub struct TextVisuals {
    /// Font used
    pub font_id: FontId,

    /// Font color
    pub color: Color32,

    /// Text decoration
    pub underline: Stroke,
    pub strikethrough: Stroke,
}

/// General widget style
#[derive(Debug, Clone)]
pub struct BaseStyle {
    pub frame: Frame,

    pub text: TextVisuals,

    pub stroke: Stroke,
}

impl WidgetStyle for BaseStyle {}

/// Dedicated button style
#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub frame: Frame,
    pub text_style: TextVisuals,

    /// Smallest size the button may take, including its frame margins.
    ///
    /// Design systems where the control height is part of the theme (a "small"
    /// / "medium" / "large" button) need to set this here, since the call site
    /// doesn't know what the theme picked.
    pub min_size: Vec2,

    /// Is [`Self::text_style`] a fallback, or the answer?
    ///
    /// By default it only fills in what the button's own atoms leave open, and
    /// an app-wide [`Visuals::override_text_color`](crate::Visuals::override_text_color)
    /// or a nested [`AtomLayout`](crate::AtomLayout) never sees it. A theme that
    /// owns the look of the whole button — one where "primary" means white text
    /// on blue, whatever the surrounding style says — sets this, and the style
    /// is pushed onto the `Ui` for the duration of the button instead.
    pub force_text_style: bool,
}

impl WidgetStyle for ButtonStyle {}

/// Dedicated checkbox style
#[derive(Debug, Clone)]
pub struct CheckboxStyle {
    /// Frame around
    pub frame: Frame,

    /// Text next to it
    pub text_style: TextVisuals,

    /// Checkbox size
    pub checkbox_size: f32,

    /// Checkmark size
    pub check_size: f32,

    /// Frame of the checkbox itself
    pub checkbox_frame: Frame,

    /// Checkmark stroke
    pub check_stroke: Stroke,
}

impl WidgetStyle for CheckboxStyle {}

/// Dedicated label style
#[derive(Debug, Clone)]
pub struct LabelStyle {
    /// Frame around
    pub frame: Frame,

    /// Text style
    pub text: TextVisuals,

    /// Wrap mode used
    pub wrap_mode: TextWrapMode,
}

impl WidgetStyle for LabelStyle {}

/// Dedicated separator style
#[derive(Debug, Clone)]
pub struct SeparatorStyle {
    /// How much space is allocated in the layout direction
    pub spacing: f32,

    /// How to paint it
    pub stroke: Stroke,
}

impl WidgetStyle for SeparatorStyle {}

/// The different state of a widget can be
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WidgetState {
    Noninteractive,
    #[default]
    Inactive,
    Hovered,
    Active,
}

impl Widgets {
    /// The widget visuals according to the state
    pub fn state(&self, state: WidgetState) -> &WidgetVisuals {
        match state {
            WidgetState::Noninteractive => &self.noninteractive,
            WidgetState::Inactive => &self.inactive,
            WidgetState::Hovered => &self.hovered,
            WidgetState::Active => &self.active,
        }
    }
}

impl Response {
    pub fn widget_state(&self) -> WidgetState {
        if !self.sense.interactive() {
            WidgetState::Noninteractive
        } else if self.is_pointer_button_down_on() || self.has_focus() || self.clicked() {
            WidgetState::Active
        } else if self.hovered() || self.highlighted() {
            WidgetState::Hovered
        } else {
            WidgetState::Inactive
        }
    }
}

pub struct StyleArgs<'a> {
    pub classes: &'a Classes,
    pub state: WidgetState,

    /// Was the pointer over the widget (or held down on it) in the previous pass?
    ///
    /// [`WidgetState`] ranks its four states, so a hovered widget that also has
    /// focus reports [`WidgetState::Active`] and the hover is lost. A theme that
    /// tints on hover independently of focus has to look here.
    pub hovered: bool,

    /// Did the widget have keyboard focus in the previous pass?
    ///
    /// [`WidgetState::Active`] also covers press and click, so a theme that
    /// wants a focus ring — and only a focus ring — has to look here.
    pub focused: bool,
    pub stack: &'a UiStack,
    pub style: &'a Style,
    pub ctx: &'a Context,
}
