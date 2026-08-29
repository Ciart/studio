use std::rc::Rc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    Context, Entity, InteractiveElement, IntoElement, ParentElement, Render, Styled, Subscription,
    Window, WindowControlArea, div, px, rgb, rgba, white,
};
use gpui_base::dock::{DockArea, DockEvent, DockLayout, DockPlacement};

use crate::{
    caption,
    dock::{self, DockPanel, DockSkin, PanelZone},
    fullscreen::FullscreenTitlebar,
    panels::PanelKind,
    theme::{BACKDROP, FONT, FONT_SIZE, TEXT},
};

pub struct Workspace {
    area: Entity<DockArea>,
    skin: Rc<DockSkin>,
    fullscreen: FullscreenTitlebar,
    _drop_rules: Subscription,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let canvas = DockPanel::new(PanelKind::Canvas, "Hello", PanelZone::Canvas, cx);
        let project = DockPanel::new(PanelKind::Project, "Project", PanelZone::Dock, cx);
        let timeline = DockPanel::new(PanelKind::Timeline, "Timeline", PanelZone::Dock, cx);
        let color_picker =
            DockPanel::new(PanelKind::ColorPicker, "Color Picker", PanelZone::Dock, cx);
        let palette = DockPanel::new(PanelKind::Palette, "Palette", PanelZone::Dock, cx);

        let (area, skin) = DockSkin::dock_area("workspace", window, cx);

        area.update(cx, |area, cx| {
            area.set_center(DockLayout::tabs().panel(canvas), window, cx);
            area.set_dock(
                DockPlacement::Left,
                DockLayout::tabs().panel(project),
                window,
                cx,
            );
            area.set_dock_size(DockPlacement::Left, px(300.), window, cx);
            area.set_dock(
                DockPlacement::Right,
                DockLayout::v_split()
                    .child(DockLayout::tabs().panel(color_picker), Some(px(320.)))
                    .child(DockLayout::tabs().panel(palette), None),
                window,
                cx,
            );
            area.set_dock_size(DockPlacement::Right, px(300.), window, cx);
            area.set_dock(
                DockPlacement::Bottom,
                DockLayout::tabs().panel(timeline),
                window,
                cx,
            );
            area.set_dock_size(DockPlacement::Bottom, px(245.), window, cx);
        });

        #[cfg(target_os = "macos")]
        {
            let dock = area.downgrade();
            let handle = window.window_handle();
            let async_app = cx.to_async();
            crate::mac::set_toolbar_action(move |button| {
                let placement = match button {
                    crate::mac::TitlebarButton::LeftDock => DockPlacement::Left,
                    crate::mac::TitlebarButton::RightDock => DockPlacement::Right,
                    crate::mac::TitlebarButton::BottomDock => DockPlacement::Bottom,
                };
                let dock = dock.clone();
                let mut async_app = async_app.clone();
                let _ = handle.update(&mut async_app, move |_, window, cx| {
                    let Some(dock) = dock.upgrade() else {
                        return;
                    };
                    dock.update(cx, |dock, cx| dock.toggle_dock(placement, window, cx));
                });
            });
        }

        let _drop_rules = cx.subscribe_in(&area, window, Self::on_dock_event);
        skin.sync_empty_regions(area.read(cx), cx);

        Workspace {
            area,
            skin,
            fullscreen: FullscreenTitlebar::default(),
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
                if self.skin.window_drag_active() {
                    return;
                }
                dock::apply_drop(area, item, target, window, cx)
            }
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.fullscreen.sync(window);
        #[cfg(target_os = "macos")]
        crate::mac::attach_dock_buttons(window);
        let native_titlebar = cfg!(target_os = "macos");

        div()
            .id("workspace-root")
            .size_full()
            .bg(rgb(BACKDROP))
            .font_family(FONT)
            .text_size(px(FONT_SIZE))
            .text_color(rgba(TEXT))
            .flex()
            .flex_col()
            .when(!native_titlebar, |el| {
                el.child(
                    div()
                        .id("titlebar")
                        .h(px(38.))
                        .flex_none()
                        .flex()
                        .items_center()
                        .window_control_area(WindowControlArea::Drag)
                        .child(div().w(px(caption::caption_buttons_width())).flex_none())
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child("Ciart Studio"),
                        )
                        .child(caption::caption_buttons(window)),
                )
            })
            .child(div().flex_1().min_h(px(0.)).child(self.area.clone()))
            .child(
                div()
                    .h(px(30.))
                    .flex_none()
                    .flex()
                    .items_center()
                    .px(px(12.))
                    .text_color(white())
                    .child("Size: 128, 128 - Cursor: 13, 29"),
            )
    }
}
