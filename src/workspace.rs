use std::rc::Rc;

use gpui::{
    Context, Entity, InteractiveElement, IntoElement, ParentElement, Render, Styled, Subscription,
    Window, WindowControlArea, div, px, rgb,
};
use gpui_base::dock::{DockArea, DockEvent, DockLayout, DockPlacement};

use crate::{
    dock::{self, DockPanel, DockSkin, PanelZone},
    theme::SURFACE,
};

pub struct Workspace {
    area: Entity<DockArea>,
    skin: Rc<DockSkin>,
    _drop_rules: Subscription,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let canvas = DockPanel::new("Canvas", "Canvas", PanelZone::Canvas, cx);
        let palette = DockPanel::new("Palette", "Palette", PanelZone::Dock, cx);
        let brushes = DockPanel::new("Brushes", "Brushes", PanelZone::Dock, cx);
        let layers = DockPanel::new("Layers", "Layers", PanelZone::Dock, cx);

        let (area, skin) = DockSkin::dock_area("workspace", window, cx);

        area.update(cx, |area, cx| {
            area.set_center(DockLayout::tabs().panel(canvas), window, cx);
            area.set_dock(
                DockPlacement::Left,
                DockLayout::tabs().panel(palette).panel(brushes),
                window,
                cx,
            );
            area.set_dock_size(DockPlacement::Left, px(240.), window, cx);
            area.set_dock(DockPlacement::Right, DockLayout::tabs(), window, cx);
            area.set_dock_size(DockPlacement::Right, px(240.), window, cx);
            area.set_dock(
                DockPlacement::Bottom,
                DockLayout::tabs().panel(layers),
                window,
                cx,
            );
            area.set_dock_size(DockPlacement::Bottom, px(140.), window, cx);
        });

        let _drop_rules = cx.subscribe_in(&area, window, Self::on_dock_event);
        skin.sync_empty_regions(area.read(cx), cx);

        Workspace {
            area,
            skin,
            _drop_rules,
        }
    }

    fn on_dock_event(
        &mut self,
        area: &Entity<DockArea>,
        event: &DockEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            DockEvent::LayoutChanged => self.skin.sync_empty_regions(area.read(cx), cx),
            DockEvent::DragDrop { item, target } => {
                dock::apply_drop(area, item, target, window, cx)
            }
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(SURFACE))
            .flex()
            .flex_col()
            .child(
                div()
                    .id("titlebar")
                    .h(px(34.))
                    .flex_none()
                    .flex()
                    .items_center()
                    .pl(px(80.))
                    .window_control_area(WindowControlArea::Drag)
                    .child("Ciart Studio")
                    .text_sm(),
            )
            .child(div().flex_1().min_h(px(0.)).child(self.area.clone()))
    }
}
