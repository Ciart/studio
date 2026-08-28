use std::{rc::Rc, sync::Arc};

use gpui::{
    AnyWindowHandle, App, Bounds, Context, Entity, IntoElement, ParentElement, Pixels, Point,
    Render, SharedString, Styled, Subscription, TitlebarOptions, WeakEntity, Window, WindowBounds,
    WindowOptions, div, point, prelude::*, px, rgb, rgba, size,
};
use gpui_base::dock::{
    AnyDrag, DockArea, DockEvent, DockPlacement, DropTarget, InsertTarget, PanelView,
};

use super::{
    AreaKind, DockSkin, PanelDrag, PanelZone,
    drag::DragState,
    util::{
        center_panels, drop_allowed, group_len, panel_entity, panel_title, placement_of, zone_of,
    },
};
use crate::{
    fullscreen::FullscreenTitlebar,
    theme::{BACKDROP, FONT, FONT_SIZE, TEXT},
};

struct DetachedWindow {
    kind: AreaKind,
    area: Entity<DockArea>,
    fullscreen: FullscreenTitlebar,
    _subscriptions: Vec<Subscription>,
}

impl DetachedWindow {
    fn open(
        state: &DragState,
        panel: Arc<dyn PanelView>,
        zone: PanelZone,
        origin: Option<Point<Pixels>>,
        cx: &mut App,
    ) -> Option<(AnyWindowHandle, WeakEntity<DockArea>)> {
        let title = panel_title(&panel, cx);
        let kind = AreaKind::Detached(zone);
        let home = match zone {
            PanelZone::Canvas => DockPlacement::Center,
            PanelZone::Dock => DockPlacement::Left,
        };
        let id = state.next_window_id.get();
        state.next_window_id.set(id + 1);
        let area_id = SharedString::from(format!("detached-{id}"));

        let window_size = size(px(420.), px(320.));
        let bounds = match origin {
            Some(cursor) => Bounds {
                origin: state.window_origin(cursor),
                size: window_size,
            },
            None => Bounds::centered(None, window_size, cx),
        };

        let state = state.clone();
        let mut opened_area = None;
        let opened = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(title),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(13.), px(13.))),
                }),
                app_owns_titlebar_drag: true,
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| {
                    let area = cx.new(|cx| {
                        let skin = Rc::new(DockSkin::new(
                            kind,
                            window.window_handle(),
                            cx.weak_entity(),
                            state.clone(),
                        ));
                        DockArea::new(area_id.clone(), Some(1), window, cx).with_renderer(skin)
                    });
                    area.update(cx, |area, cx| {
                        area.add_panel_view(panel.clone(), DockPlacement::Center, None, window, cx)
                    });
                    let subscription = cx.subscribe_in(&area, window, Self::on_area_event);
                    DetachedWindow {
                        kind,
                        area,
                        fullscreen: FullscreenTitlebar::default(),
                        _subscriptions: vec![subscription],
                    }
                });
                let area = view.read(cx).area.clone();
                opened_area = Some(area.downgrade());
                window.on_window_should_close(cx, {
                    let state = state.clone();
                    move |_, cx| {
                        let Some(main) = state
                            .main_area
                            .borrow()
                            .clone()
                            .and_then(|main| main.upgrade())
                        else {
                            return true;
                        };
                        let Some(main_window) = *state.main_window.borrow() else {
                            return true;
                        };
                        let panels: Vec<_> = {
                            let area = area.read(cx);
                            center_panels(area)
                                .into_iter()
                                .filter_map(|id| area.panel(id).cloned())
                                .collect()
                        };
                        if panels.is_empty() {
                            return true;
                        }
                        let _ = main_window.update(cx, |_, window, cx| {
                            main.update(cx, |main, cx| {
                                for panel in panels {
                                    main.add_panel_view(panel, home, None, window, cx);
                                }
                            })
                        });
                        true
                    }
                });
                view
            },
        );
        opened.ok().map(Into::into).zip(opened_area)
    }

    fn on_area_event(
        &mut self,
        area: &Entity<DockArea>,
        event: &DockEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            DockEvent::LayoutChanged => {
                if center_panels(area.read(cx)).is_empty() {
                    let handle = window.window_handle();
                    cx.defer(move |cx| {
                        let _ = handle.update(cx, |_, window, _| window.remove_window());
                    });
                }
            }
            DockEvent::DragDrop { item, target } => {
                apply_drop_in(self.kind, area, item, target, window, cx)
            }
        }
    }
}

impl Render for DetachedWindow {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.fullscreen.sync(window);

        div()
            .id("detached-root")
            .size_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(rgb(BACKDROP))
            .font_family(FONT)
            .text_size(px(FONT_SIZE))
            .text_color(rgba(TEXT))
            .child(div().flex_1().min_h(px(0.)).child(self.area.clone()))
    }
}

pub(crate) fn detach_panel(
    state: &DragState,
    source_area: &WeakEntity<DockArea>,
    panel: Arc<dyn PanelView>,
    origin: Option<Point<Pixels>>,
    window: &mut Window,
    cx: &mut App,
) -> Option<(AnyWindowHandle, WeakEntity<DockArea>)> {
    let area = source_area.upgrade()?;
    let entity = panel_entity(&panel)?;
    let zone = zone_of(&panel, cx)?;
    area.update(cx, |area, cx| area.remove_panel(entity, window, cx));
    DetachedWindow::open(state, panel, zone, origin, cx)
}

pub(crate) fn living_source(drag: &PanelDrag) -> WeakEntity<DockArea> {
    drag.detached
        .borrow()
        .clone()
        .map(|(_, area)| area)
        .unwrap_or_else(|| drag.source_area.clone())
}

pub fn apply_drop(
    area: &Entity<DockArea>,
    item: &AnyDrag,
    target: &DropTarget,
    window: &mut Window,
    cx: &mut App,
) {
    apply_drop_in(AreaKind::Main, area, item, target, window, cx);
}

pub(crate) fn apply_drop_in(
    kind: AreaKind,
    area: &Entity<DockArea>,
    item: &AnyDrag,
    target: &DropTarget,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(drag) = item.value().downcast_ref::<PanelDrag>() else {
        return;
    };
    let DropTarget::Group { node, placement } = target else {
        return;
    };

    if drag.source == Some(*node) && living_source(drag).entity_id() == area.entity_id() {
        let len = group_len(area.read(cx), *node).unwrap_or(0);
        if placement.is_none() || len <= 1 {
            return;
        }
    }

    let Some(region) = placement_of(area.read(cx), *node) else {
        return;
    };
    if !drop_allowed(kind, zone_of(&drag.view, cx), region) {
        return;
    }

    let target = match placement {
        Some(placement) => InsertTarget::Split {
            node: *node,
            placement: *placement,
            size: None,
        },
        None => InsertTarget::Tabs {
            node: *node,
            ix: None,
            activate: true,
        },
    };
    transfer_panel(drag, area, region, Some(target), window, cx);
}

pub(crate) fn transfer_panel(
    drag: &PanelDrag,
    target_area: &Entity<DockArea>,
    region: DockPlacement,
    target: Option<InsertTarget>,
    window: &mut Window,
    cx: &mut App,
) {
    if drag.landed.replace(true) {
        return;
    }
    let source_area = living_source(drag);
    let source_window = drag
        .detached
        .borrow()
        .as_ref()
        .map(|(window, _)| *window)
        .unwrap_or(drag.source_window);

    if source_area.entity_id() == target_area.entity_id() {
        if let Some(target) = target {
            target_area.update(cx, |area, cx| {
                area.move_panel(drag.panel, target, window, cx)
            });
        }
        return;
    }

    if let Some(source) = source_area.upgrade()
        && let Some(entity) = panel_entity(&drag.view)
    {
        if source_window.window_id() == window.window_handle().window_id() {
            source.update(cx, |area, cx| area.remove_panel(entity, window, cx));
        } else {
            let _ = source_window.update(cx, |_, source_window, cx| {
                source.update(cx, |area, cx| area.remove_panel(entity, source_window, cx))
            });
        }
    }
    target_area.update(cx, |area, cx| {
        area.add_panel_view(drag.view.clone(), region, None, window, cx);
        if let Some(target) = target {
            area.move_panel(drag.panel, target, window, cx);
        }
    });
}
