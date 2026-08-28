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
    rgb,
};
use gpui_base::dock::{DockArea, NodeId, Panel, PanelEvent, PanelId, PanelView};

use crate::theme::{ACCENT, SURFACE};

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
    name: &'static str,
    pub(crate) title: SharedString,
    pub(crate) zone: PanelZone,
    focus_handle: FocusHandle,
}

impl DockPanel {
    pub fn new(
        name: &'static str,
        title: impl Into<SharedString>,
        zone: PanelZone,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            name,
            title: title.into(),
            zone,
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Panel for DockPanel {
    fn panel_name(&self) -> &'static str {
        self.name
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
        div().size_full().p_3().child(self.title.clone())
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
            .px_2()
            .text_xs()
            .bg(rgb(SURFACE))
            .text_color(rgb(ACCENT))
            .border_1()
            .border_color(rgb(ACCENT))
            .rounded_sm()
            .child(self.title.clone())
    }
}

pub(crate) struct HiddenPreview;

impl Render for HiddenPreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

pub(crate) const TAB_HEIGHT: f32 = 28.;
pub(crate) const TITLEBAR_HEIGHT: f32 = 34.;
pub(crate) const TRAFFIC_LIGHT_PAD: f32 = 80.;
