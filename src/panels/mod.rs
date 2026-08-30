mod canvas;
mod color_picker;
mod palette;
mod project;
mod timeline;
mod widgets;

pub use canvas::CanvasPanel;
pub use widgets::{empty_hint, icon};

use gpui::{AnyElement, AnyView, App, AppContext as _, IntoElement};

use crate::theme::{CANVAS, PANEL, TRACK};

pub const PROJECT: &str = "Project";
pub const CANVAS_PANEL: &str = "Canvas";
pub const TIMELINE: &str = "Timeline";
pub const COLOR_PICKER: &str = "ColorPicker";
pub const PALETTE: &str = "Palette";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PanelKind {
    Project,
    Canvas,
    Timeline,
    ColorPicker,
    Palette,
}

impl PanelKind {
    pub fn background(self) -> u32 {
        match self {
            PanelKind::Canvas => CANVAS,
            PanelKind::Timeline => TRACK,
            _ => PANEL,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            PanelKind::Project => PROJECT,
            PanelKind::Canvas => CANVAS_PANEL,
            PanelKind::Timeline => TIMELINE,
            PanelKind::ColorPicker => COLOR_PICKER,
            PanelKind::Palette => PALETTE,
        }
    }

    /// Build what this panel renders. Panels that own state get their own view
    /// entity so it survives re-renders and follows the panel between windows.
    pub fn content(self, cx: &mut App) -> PanelContent {
        match self {
            PanelKind::Canvas => PanelContent::View(cx.new(CanvasPanel::new).into()),
            PanelKind::Project => PanelContent::Stateless(project::project),
            PanelKind::Timeline => PanelContent::Stateless(timeline::timeline),
            PanelKind::ColorPicker => PanelContent::Stateless(color_picker::color_picker),
            PanelKind::Palette => PanelContent::Stateless(palette::palette),
        }
    }
}

pub enum PanelContent {
    /// Stateless panels are element trees rebuilt on every render.
    Stateless(fn() -> AnyElement),
    /// Stateful panels live in their own view entity.
    View(AnyView),
}

impl PanelContent {
    pub fn body(&self) -> AnyElement {
        match self {
            PanelContent::Stateless(build) => build(),
            PanelContent::View(view) => view.clone().into_any_element(),
        }
    }
}
