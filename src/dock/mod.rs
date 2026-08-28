mod detach;
mod drag;
mod skin;
mod util;

pub use detach::apply_drop;
pub use skin::DockSkin;

use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use gpui::{
    AnyWindowHandle, App, Context, Empty, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, ParentElement, Render, SharedString, Styled, WeakEntity, Window, div, prelude::*,
    px, rgb, rgba,
};
use gpui_base::dock::{DockArea, NodeId, Panel, PanelEvent, PanelId, PanelView};

use crate::{
    panels::PanelKind,
    theme::{FONT, FONT_SIZE, HAIRLINE, PANEL, RADIUS, TEXT},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PanelZone {
    Canvas,
    Dock,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum AreaKind {
    Main,
    Detached(PanelZone),
}

impl AreaKind {
    pub(crate) fn is_detached(self) -> bool {
        matches!(self, AreaKind::Detached(_))
    }
}

pub struct DockPanel {
    pub(crate) kind: PanelKind,
    pub(crate) title: SharedString,
    pub(crate) zone: PanelZone,
    focus_handle: FocusHandle,
}

impl DockPanel {
    pub fn new(
        kind: PanelKind,
        title: impl Into<SharedString>,
        zone: PanelZone,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            kind,
            title: title.into(),
            zone,
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Panel for DockPanel {
    fn panel_name(&self) -> &'static str {
        self.kind.name()
    }
}

impl EventEmitter<PanelEvent> for DockPanel {}

impl Focusable for DockPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DockPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.kind.body()
    }
}

pub(crate) struct PanelDrag {
    pub(crate) panel: PanelId,
    pub(crate) source: Option<NodeId>,
    pub(crate) source_area: WeakEntity<DockArea>,
    pub(crate) source_window: AnyWindowHandle,
    pub(crate) view: Arc<dyn PanelView>,
    pub(crate) detached: RefCell<Option<(AnyWindowHandle, WeakEntity<DockArea>)>>,
    pub(crate) landed: Cell<bool>,
}

pub(crate) struct DragPreview {
    pub(crate) title: SharedString,
}

impl Render for DragPreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(PANEL))
            .font_family(FONT)
            .text_size(px(FONT_SIZE))
            .text_color(rgba(TEXT))
            .border_1()
            .border_color(rgba(HAIRLINE))
            .rounded(px(RADIUS))
            .child(self.title.clone())
    }
}

pub(crate) struct HiddenPreview;

impl Render for HiddenPreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

pub(crate) const TAB_HEIGHT: f32 = 30.;
pub(crate) const TITLEBAR_HEIGHT: f32 = 38.;
pub(crate) const TRAFFIC_LIGHT_PAD: f32 = 78.;
