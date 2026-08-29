use std::{cell::Cell, rc::Rc, sync::Arc};

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

pub(crate) const SPARE_IDLE: std::time::Duration = std::time::Duration::from_secs(60);

#[cfg(target_os = "windows")]
fn raw_hwnd(window: &Window) -> Option<windows::Win32::Foundation::HWND> {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let handle = HasWindowHandle::window_handle(window).ok()?;
    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        return None;
    };
    Some(windows::Win32::Foundation::HWND(
        handle.hwnd.get() as *mut std::ffi::c_void
    ))
}

#[cfg(target_os = "windows")]
fn set_transitions(hwnd: windows::Win32::Foundation::HWND, enabled: bool) {
    use windows::Win32::Graphics::Dwm::{DWMWA_TRANSITIONS_FORCEDISABLED, DwmSetWindowAttribute};
    use windows::core::BOOL;

    let disabled: BOOL = (!enabled).into();
    unsafe {
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_TRANSITIONS_FORCEDISABLED,
            &disabled as *const _ as _,
            std::mem::size_of::<BOOL>() as u32,
        );
    }
}

#[cfg(target_os = "windows")]
fn conceal(window: &Window) {
    let Some(hwnd) = raw_hwnd(window) else {
        return;
    };
    set_transitions(hwnd, false);
}

#[cfg(target_os = "windows")]
pub(crate) fn set_masked(window: &Window, masked: bool) {
    use windows::Win32::Graphics::Gdi::{CreateRectRgn, SetWindowRgn};

    let Some(hwnd) = raw_hwnd(window) else {
        return;
    };
    unsafe {
        let region = if masked {
            Some(CreateRectRgn(0, 0, 0, 0))
        } else {
            None
        };
        SetWindowRgn(hwnd, region, true);
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn set_masked(window: &Window, masked: bool) {
    crate::mac::set_alpha(window, if masked { 0. } else { 1. });
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub(crate) fn set_masked(_: &Window, _: bool) {}

#[cfg(target_os = "windows")]
fn reveal(window: &Window, origin: Point<Pixels>) {
    use windows::Win32::UI::WindowsAndMessaging::{
        HWND_TOP, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
    };

    let Some(hwnd) = raw_hwnd(window) else {
        return;
    };
    let scale = window.scale_factor();
    unsafe {
        let _ = SetWindowPos(
            hwnd,
            Some(HWND_TOP),
            (origin.x.as_f32() * scale).round() as i32,
            (origin.y.as_f32() * scale).round() as i32,
            0,
            0,
            SWP_NOSIZE | SWP_SHOWWINDOW,
        );
    }
    set_transitions(hwnd, true);
}

#[cfg(not(target_os = "windows"))]
fn conceal(_: &Window) {}

#[cfg(not(target_os = "windows"))]
fn reveal(window: &mut Window, origin: Point<Pixels>) {
    window.set_origin(origin);
}

pub(crate) struct SpareWindow {
    handle: AnyWindowHandle,
    area: WeakEntity<DockArea>,
    adopted: Rc<Cell<bool>>,
    zone: PanelZone,
}

impl SpareWindow {
    pub(crate) fn window_id(&self) -> gpui::WindowId {
        self.handle.window_id()
    }
}

struct DetachedWindow {
    kind: AreaKind,
    drag: DragState,
    area: Entity<DockArea>,
    fullscreen: FullscreenTitlebar,
    adopted: Rc<Cell<bool>>,
    _subscriptions: Vec<Subscription>,
}

impl DetachedWindow {
    fn open(
        state: &DragState,
        panel: Arc<dyn PanelView>,
        zone: PanelZone,
        origin: Option<Point<Pixels>>,
        insert: bool,
        cx: &mut App,
    ) -> Option<SpareWindow> {
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
        let bounds = match origin.filter(|_| insert) {
            Some(cursor) => Bounds {
                origin: state.window_origin(cursor),
                size: window_size,
            },
            None => Bounds::centered(None, window_size, cx),
        };

        let adopted = Rc::new(Cell::new(insert));
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
                focus: insert,
                show: insert,
                ..Default::default()
            },
            |window, cx| {
                let adopted = adopted.clone();
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
                    if insert {
                        area.update(cx, |area, cx| {
                            area.add_panel_view(
                                panel.clone(),
                                DockPlacement::Center,
                                None,
                                window,
                                cx,
                            )
                        });
                    }
                    let subscription = cx.subscribe_in(&area, window, Self::on_area_event);
                    let moved = cx.observe_window_bounds(window, Self::on_bounds_changed);
                    DetachedWindow {
                        kind,
                        drag: state.clone(),
                        area,
                        fullscreen: FullscreenTitlebar::default(),
                        adopted,
                        _subscriptions: vec![subscription, moved],
                    }
                });
                let area = view.read(cx).area.clone();
                opened_area = Some(area.downgrade());
                window.on_move_loop_ended(cx, {
                    let state = state.clone();
                    move |window, cx| {
                        let handle = window.window_handle();
                        state.window_move_ended(handle, window, cx);
                    }
                });
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
        let handle: AnyWindowHandle = opened.ok()?.into();
        Some(SpareWindow {
            handle,
            area: opened_area?,
            adopted,
            zone,
        })
    }

    fn on_bounds_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let handle = window.window_handle();
        self.drag.clone().window_dragged(handle, window, cx);
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
                if self.adopted.get() && center_panels(area.read(cx)).is_empty() {
                    let handle = window.window_handle();
                    cx.defer(move |cx| {
                        let _ = handle.update(cx, |_, window, _| window.remove_window());
                    });
                }
            }
            DockEvent::DragDrop { item, target } => {
                if self.drag.window_drag_active() {
                    return;
                }
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

    let spare = {
        let mut spares = state.spares.borrow_mut();
        spares
            .iter()
            .position(|spare| spare.zone == zone)
            .map(|at| spares.remove(at))
    };
    if let Some(spare) = spare {
        if let Some(adopted) = fill_spare(state, &spare, panel.clone(), origin, cx) {
            return Some(adopted);
        }
        let _ = spare
            .handle
            .update(cx, |_, window, _| window.remove_window());
    }
    DetachedWindow::open(state, panel, zone, origin, true, cx).map(|spare| {
        let _ = spare.handle.update(cx, |_, window, _| {
            #[cfg(target_os = "macos")]
            if let Some(cursor) = crate::mac::cursor_position() {
                let origin = state.window_origin(cursor);
                crate::mac::move_window_ref(
                    crate::mac::window_ref(window),
                    f32::from(origin.x),
                    f32::from(origin.y),
                );
            }
            #[cfg(not(target_os = "macos"))]
            window.start_window_move();
        });
        (spare.handle, spare.area)
    })
}

pub(crate) fn has_spare_for(state: &DragState, panel: &Arc<dyn PanelView>, cx: &App) -> bool {
    let Some(zone) = zone_of(panel, cx) else {
        return true;
    };
    state.spares.borrow().iter().any(|spare| spare.zone == zone)
}

pub(crate) fn prepare_spare(state: &DragState, panel: &Arc<dyn PanelView>, cx: &mut App) {
    if !cfg!(target_os = "windows") {
        return;
    }
    let Some(zone) = zone_of(panel, cx) else {
        return;
    };
    if state.spares.borrow().iter().any(|spare| spare.zone == zone) {
        return;
    }
    if let Some(spare) = DetachedWindow::open(state, panel.clone(), zone, None, false, cx) {
        let _ = spare.handle.update(cx, |_, window, _| conceal(window));
        state.spares.borrow_mut().push(spare);
    }
}

pub(crate) fn close_spares(state: &DragState, cx: &mut App) {
    let spares: Vec<SpareWindow> = state.spares.borrow_mut().drain(..).collect();
    for spare in spares {
        let _ = spare
            .handle
            .update(cx, |_, window, _| window.remove_window());
    }
}

pub(crate) fn close_spare_if_last(state: &DragState, cx: &mut App) {
    let ids: Vec<_> = state
        .spares
        .borrow()
        .iter()
        .map(|spare| spare.window_id())
        .collect();
    if ids.is_empty()
        || !cx
            .windows()
            .iter()
            .all(|handle| ids.contains(&handle.window_id()))
    {
        return;
    }
    for spare in state.spares.borrow_mut().drain(..) {
        let _ = spare
            .handle
            .update(cx, |_, window, _| window.remove_window());
    }
}

fn fill_spare(
    state: &DragState,
    spare: &SpareWindow,
    panel: Arc<dyn PanelView>,
    origin: Option<Point<Pixels>>,
    cx: &mut App,
) -> Option<(AnyWindowHandle, WeakEntity<DockArea>)> {
    let target = spare.area.upgrade()?;
    let placed = spare.handle.update(cx, |_, window, cx| {
        window.set_window_title(&panel_title(&panel, cx));
        target.update(cx, |target, cx| {
            target.add_panel_view(panel, DockPlacement::Center, None, window, cx)
        });
        if let Some(cursor) = origin {
            reveal(window, state.window_origin(cursor));
        }
        window.start_window_move();
    });
    placed.ok()?;
    spare.adopted.set(true);
    Some((spare.handle, spare.area.clone()))
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
