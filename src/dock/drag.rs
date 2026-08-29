use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use gpui::{
    AnyWindowHandle, App, AppContext, Entity, Pixels, Point, SharedString, Size, WeakEntity,
    Window, point, px, size,
};
use gpui_base::{
    Placement,
    dock::{AnyDrag, DockArea, DockPlacement, DropTarget, NodeId, PanelView},
};

use super::{
    AreaKind, HiddenPreview, PanelDrag, PanelZone, TAB_HEIGHT, TRAFFIC_LIGHT_PAD,
    chip::{Chip, ChipContent},
    detach::{
        SPARE_IDLE, SpareWindow, apply_drop_in, close_spares, detach_panel, has_spare_for,
        prepare_spare, set_masked, transfer_panel,
    },
    util::{
        EmptyDockCache, GroupBoundsCache, center_panels, drop_allowed, placement_of, split_zone_at,
        zone_of,
    },
};

#[derive(Clone)]
pub(crate) struct AreaEntry {
    pub(crate) kind: AreaKind,
    pub(crate) window: AnyWindowHandle,
    pub(crate) area: WeakEntity<DockArea>,
    pub(crate) groups: GroupBoundsCache,
    pub(crate) empty_docks: EmptyDockCache,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum MergeSpot {
    Group {
        node: NodeId,
        placement: Option<Placement>,
    },
    EmptyDock(DockPlacement),
}

#[derive(Clone, Copy)]
pub(crate) struct Grab {
    pub(crate) offset: Point<Pixels>,
    pub(crate) size: Size<Pixels>,
}

impl Default for Grab {
    fn default() -> Self {
        let size = size(px(96.), px(TAB_HEIGHT));
        Self {
            offset: point(size.width / 2., size.height / 2.),
            size,
        }
    }
}

#[derive(Clone)]
pub(crate) struct MergeTarget {
    kind: AreaKind,
    window: AnyWindowHandle,
    pub(crate) area: WeakEntity<DockArea>,
    pub(crate) spot: MergeSpot,
}

impl MergeTarget {
    pub(crate) fn same_spot(&self, other: &MergeTarget) -> bool {
        self.area.entity_id() == other.area.entity_id() && self.spot == other.spot
    }
}

#[derive(Clone)]
pub(crate) struct DragState {
    pub(crate) main_area: Rc<RefCell<Option<WeakEntity<DockArea>>>>,
    pub(crate) main_window: Rc<RefCell<Option<AnyWindowHandle>>>,
    pub(crate) dragging_zone: Rc<Cell<Option<PanelZone>>>,
    pub(crate) hovered_group_accepts: Rc<Cell<bool>>,
    pub(crate) chip: Rc<Chip>,
    pub(crate) attached_window: Rc<RefCell<Option<AnyWindowHandle>>>,
    pub(crate) next_window_id: Rc<Cell<usize>>,
    pub(crate) areas: Rc<RefCell<Vec<AreaEntry>>>,
    pub(crate) dragged: Rc<RefCell<Option<AnyDrag>>>,
    pub(crate) merge_target: Rc<RefCell<Option<MergeTarget>>>,
    pub(crate) last_position: Rc<Cell<Option<Point<Pixels>>>>,
    pub(crate) hidden: Rc<Cell<bool>>,
    poll_running: Rc<Cell<bool>>,
    handed_off: Rc<Cell<bool>>,
    still_ticks: Rc<Cell<u32>>,
    last_cursor: Rc<Cell<Option<Point<Pixels>>>>,
    pub(crate) spares: Rc<RefCell<Vec<SpareWindow>>>,
    pub(crate) spares_used: Rc<Cell<u64>>,
    pub(crate) spare_pending: Rc<Cell<bool>>,
    pub(crate) grab: Rc<Cell<Grab>>,
}

impl DragState {
    pub(crate) fn new() -> Self {
        Self {
            main_area: Rc::new(RefCell::new(None)),
            main_window: Rc::new(RefCell::new(None)),
            dragging_zone: Rc::new(Cell::new(None)),
            hovered_group_accepts: Rc::new(Cell::new(true)),
            chip: Rc::new(Chip::new()),
            attached_window: Rc::new(RefCell::new(None)),
            next_window_id: Rc::new(Cell::new(0)),
            areas: Rc::new(RefCell::new(Vec::new())),
            dragged: Rc::new(RefCell::new(None)),
            merge_target: Rc::new(RefCell::new(None)),
            last_position: Rc::new(Cell::new(None)),
            hidden: Rc::new(Cell::new(false)),
            poll_running: Rc::new(Cell::new(false)),
            handed_off: Rc::new(Cell::new(false)),
            still_ticks: Rc::new(Cell::new(0)),
            last_cursor: Rc::new(Cell::new(None)),
            spares: Rc::new(RefCell::new(Vec::new())),
            spares_used: Rc::new(Cell::new(0)),
            spare_pending: Rc::new(Cell::new(false)),
            grab: Rc::new(Cell::new(Grab::default())),
        }
    }

    pub(crate) fn register_area(&self, entry: AreaEntry) {
        let mut areas = self.areas.borrow_mut();
        areas.retain(|entry| entry.area.upgrade().is_some());
        areas.push(entry);
    }

    pub(crate) fn update_merge_target(
        &self,
        view: &Arc<dyn PanelView>,
        dragged: Option<AnyWindowHandle>,
        position: Point<Pixels>,
        source_window: &mut Window,
        cx: &mut App,
    ) {
        let next = self.resolve_merge_target(view, dragged, position, source_window, cx);
        let previous = self.merge_target.borrow().clone();
        let unchanged = match (&previous, &next) {
            (Some(previous), Some(next)) => previous.same_spot(next),
            (None, None) => true,
            _ => false,
        };
        if unchanged {
            return;
        }

        *self.merge_target.borrow_mut() = next.clone();
        let touched = previous
            .map(|target| target.window)
            .into_iter()
            .chain(next.map(|target| target.window));
        let source_id = source_window.window_handle().window_id();
        for handle in touched {
            if handle.window_id() == source_id {
                source_window.refresh();
            } else {
                let _ = handle.update(cx, |_, window, _| window.refresh());
            }
        }
    }

    pub(crate) fn resolve_merge_target(
        &self,
        view: &Arc<dyn PanelView>,
        dragged: Option<AnyWindowHandle>,
        position: Point<Pixels>,
        source_window: &mut Window,
        cx: &mut App,
    ) -> Option<MergeTarget> {
        let source_id = source_window.window_handle().window_id();
        let zone = zone_of(view, cx);
        let entries: Vec<AreaEntry> = self
            .areas
            .borrow()
            .iter()
            .filter(|entry| {
                dragged.is_none_or(|dragged| entry.window.window_id() != dragged.window_id())
            })
            .cloned()
            .collect();

        let mut found: Option<MergeTarget> = None;
        for entry in entries {
            let Some(area) = entry.area.upgrade() else {
                continue;
            };
            let origin = if entry.window.window_id() == source_id {
                Ok(source_window.bounds().origin)
            } else {
                entry
                    .window
                    .update(cx, |_, window, _| window.bounds().origin)
            };
            let Ok(origin) = origin else {
                continue;
            };
            let Some(spot) = hit_spot(&entry, area.read(cx), zone, position - origin) else {
                continue;
            };
            if entry.kind.is_detached() || found.is_none() {
                found = Some(MergeTarget {
                    kind: entry.kind,
                    window: entry.window,
                    area: entry.area.clone(),
                    spot,
                });
            }
        }
        found
    }

    pub(crate) fn drag_accepted(&self, zone: PanelZone) -> bool {
        self.dragging_zone.get() == Some(zone)
    }

    pub(crate) fn is_dragged_window(&self, handle: AnyWindowHandle) -> bool {
        self.attached_window
            .borrow()
            .is_some_and(|dragged| dragged.window_id() == handle.window_id())
    }

    pub(crate) fn window_drag_active(&self) -> bool {
        self.attached_window.borrow().is_some()
    }

    pub(crate) fn pointer_moved(&self, window: &mut Window, cx: &mut App) {
        let Some(drag) = self.dragged.borrow().clone() else {
            return;
        };
        let position = window.bounds().origin + window.mouse_position();
        self.track(&drag, position, window, cx);
    }

    pub(crate) fn chip_origin(&self, cursor: Point<Pixels>) -> Point<Pixels> {
        cursor - self.grab.get().offset
    }

    pub(crate) fn window_origin(&self, cursor: Point<Pixels>) -> Point<Pixels> {
        self.chip_origin(cursor) - point(px(TRAFFIC_LIGHT_PAD), px(0.))
    }

    pub(crate) fn start_drag_preview(
        &self,
        title: SharedString,
        zone: Option<PanelZone>,
        drag: AnyDrag,
        grab: Grab,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<HiddenPreview> {
        self.dragging_zone.set(zone);
        self.last_position.set(None);
        *self.dragged.borrow_mut() = Some(drag.clone());
        self.grab.set(grab);

        let cursor = window.bounds().origin + window.mouse_position();

        let already_attached = drag
            .value()
            .downcast_ref::<PanelDrag>()
            .and_then(|payload| payload.detached.borrow().clone());

        if let Some((attached, _)) = already_attached {
            *self.attached_window.borrow_mut() = Some(attached);
            self.prime_chip(title.clone(), window.scale_factor(), cx);
            self.move_chip(None, cx);
            #[cfg(not(target_os = "macos"))]
            if attached.window_id() == window.window_handle().window_id() {
                window.start_window_move();
            } else {
                let _ = attached.update(cx, |_, window, _| window.start_window_move());
            }
            self.start_drag_poll(cx);
        } else {
            self.prime_chip(title.clone(), window.scale_factor(), cx);
            self.move_chip(Some(self.chip_origin(cursor)), cx);
        }

        cx.new(|_| HiddenPreview)
    }

    pub(crate) fn open_chip(&self, cx: &mut App) {
        self.chip.open(cx);
    }

    pub(crate) fn prime_chip(&self, title: SharedString, scale: f32, cx: &mut App) {
        self.chip.prime(
            ChipContent {
                title,
                size: self.grab.get().size,
                scale,
            },
            cx,
        );
    }

    fn move_chip(&self, origin: Option<Point<Pixels>>, cx: &mut App) {
        match origin {
            Some(origin) => self.chip.show_at(origin, cx),
            None => self.chip.hide(cx),
        }
    }

    fn prepare_spare_off_path(&self, view: Arc<dyn PanelView>, source: &mut Window, cx: &mut App) {
        if self.spare_pending.replace(true) || has_spare_for(self, &view, cx) {
            return;
        }
        let state = self.clone();
        source.on_next_frame(move |_, cx| {
            state.spare_pending.set(false);
            prepare_spare(&state, &view, cx);
        });
    }

    fn release_idle_spares(&self, cx: &mut App) {
        let used = self.spares_used.get() + 1;
        self.spares_used.set(used);
        let state = self.clone();
        cx.spawn(async move |cx| {
            cx.background_executor().timer(SPARE_IDLE).await;
            let _ = cx.update(|cx| {
                if state.spares_used.get() == used {
                    close_spares(&state, cx);
                }
            });
        })
        .detach();
    }

    fn refill_spare(&self, cx: &mut App) {
        let Some(view) = self.dragged.borrow().as_ref().and_then(|drag| {
            drag.value()
                .downcast_ref::<PanelDrag>()
                .map(|p| p.view.clone())
        }) else {
            return;
        };
        let Some(main) = *self.main_window.borrow() else {
            return;
        };
        let state = self.clone();
        let _ = main.update(cx, |_, window, _| {
            window.on_next_frame(move |_, cx| prepare_spare(&state, &view, cx));
        });
    }

    pub(crate) fn window_dragged(
        &self,
        handle: AnyWindowHandle,
        window: &mut Window,
        cx: &mut App,
    ) {
        if cfg!(target_os = "macos") {
            return;
        }
        if !self.is_dragged_window(handle) {
            return;
        }
        let position =
            window.bounds().origin + point(px(TRAFFIC_LIGHT_PAD), px(0.)) + self.grab.get().offset;
        self.window_dragged_at(handle, position, window, cx);
    }

    fn window_dragged_at(
        &self,
        handle: AnyWindowHandle,
        position: Point<Pixels>,
        source_window: &mut Window,
        cx: &mut App,
    ) {
        let Some(drag) = self.dragged.borrow().clone() else {
            return;
        };
        let Some(payload) = drag.value().downcast_ref::<PanelDrag>() else {
            return;
        };
        self.last_position.set(Some(position));
        self.update_merge_target(&payload.view, Some(handle), position, source_window, cx);

        let masked = if self.merge_target.borrow().is_some() {
            self.move_chip(Some(self.chip_origin(position)), cx);
            (!self.hidden.replace(true)).then_some(true)
        } else if self.hidden.replace(false) {
            self.move_chip(None, cx);
            Some(false)
        } else {
            None
        };

        #[cfg(target_os = "macos")]
        let origin = (!self.handed_off.get()).then(|| self.window_origin(position));
        #[cfg(not(target_os = "macos"))]
        let origin: Option<Point<Pixels>> = None;

        if masked.is_none() && origin.is_none() {
            return;
        }
        self.with_dragged(handle, source_window, cx, |window| {
            if let Some(masked) = masked {
                set_masked(window, masked);
            }
            #[cfg(target_os = "macos")]
            if let Some(origin) = origin {
                crate::mac::move_window_ref(
                    crate::mac::window_ref(window),
                    f32::from(origin.x),
                    f32::from(origin.y),
                );
            }
        });
    }

    fn with_dragged(
        &self,
        handle: AnyWindowHandle,
        source_window: &mut Window,
        cx: &mut App,
        act: impl FnOnce(&mut Window),
    ) {
        if handle.window_id() == source_window.window_handle().window_id() {
            act(source_window);
        } else {
            let _ = handle.update(cx, |_, window, _| act(window));
        }
    }

    #[cfg(target_os = "macos")]
    fn consider_handoff(&self, handle: AnyWindowHandle, cursor: Point<Pixels>, cx: &mut App) {
        let still = self.last_cursor.get().is_some_and(|last| {
            f32::from(cursor.x - last.x).abs() < 2. && f32::from(cursor.y - last.y).abs() < 2.
        });
        self.last_cursor.set(Some(cursor));
        if !still || self.hidden.get() || self.merge_target.borrow().is_some() {
            self.still_ticks.set(0);
            return;
        }
        let ticks = self.still_ticks.get() + 1;
        self.still_ticks.set(ticks);
        if ticks < 12 {
            return;
        }
        self.handed_off.set(true);
        let _ = handle.update(cx, |_, window, _| window.start_window_move());
    }

    #[cfg(target_os = "macos")]
    fn start_drag_poll(&self, cx: &mut App) {
        if self.poll_running.replace(true) {
            return;
        }
        let state = self.clone();
        cx.spawn(async move |cx| {
            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(8))
                    .await;
                let done = cx.update(|cx| {
                    let Some(handle) = *state.attached_window.borrow() else {
                        return true;
                    };
                    let Some(cursor) = crate::mac::cursor_position() else {
                        return true;
                    };
                    let lost = handle
                        .update(cx, |_, window, cx| {
                            state.window_dragged_at(handle, cursor, window, cx)
                        })
                        .is_err();
                    if !lost && !state.handed_off.get() {
                        state.consider_handoff(handle, cursor, cx);
                    }
                    lost
                });
                if done {
                    break;
                }
            }
            state.poll_running.set(false);
        })
        .detach();
    }

    #[cfg(not(target_os = "macos"))]
    fn start_drag_poll(&self, _: &mut App) {}

    pub(crate) fn window_move_ended(
        &self,
        handle: AnyWindowHandle,
        window: &mut Window,
        cx: &mut App,
    ) {
        if !self.is_dragged_window(handle) {
            return;
        }
        if self.merge_target.borrow().is_none() && self.hidden.replace(false) {
            set_masked(window, false);
        }
        cx.stop_active_drag(window);
    }

    fn track(
        &self,
        drag: &AnyDrag,
        position: Point<Pixels>,
        source_window: &mut Window,
        cx: &mut App,
    ) {
        let Some(payload) = drag.value().downcast_ref::<PanelDrag>() else {
            return;
        };
        self.last_position.set(Some(position));

        let attached = *self.attached_window.borrow();
        if let Some(handle) = attached {
            let position = live_cursor(position);
            self.window_dragged_at(handle, position, source_window, cx);
            return;
        }
        if payload.detached.borrow().is_some() {
            self.update_merge_target(&payload.view, attached, position, source_window, cx);
        }

        let preview = self.chip.window_id();
        let spares: Vec<_> = self
            .spares
            .borrow()
            .iter()
            .map(|spare| spare.window_id())
            .collect();
        let source_id = source_window.window_handle().window_id();
        let over_own_window = source_window.bounds().contains(&position)
            || cx.windows().iter().any(|handle| {
                handle.window_id() != source_id
                    && preview.is_none_or(|preview| preview != handle.window_id())
                    && !spares.contains(&handle.window_id())
                    && handle
                        .update(cx, |_, window, _| window.bounds().contains(&position))
                        .unwrap_or(false)
            });

        if !source_window.bounds().contains(&position) {
            self.prepare_spare_off_path(payload.view.clone(), source_window, cx);
        }

        if over_own_window || zone_of(&payload.view, cx).is_none() {
            self.move_chip(Some(self.chip_origin(position)), cx);
            return;
        }

        self.move_chip(None, cx);
        let position = live_cursor(position);
        self.last_position.set(Some(position));
        let torn_off = detach_panel(
            self,
            &payload.source_area,
            payload.view.clone(),
            Some(position),
            source_window,
            cx,
        );
        if let Some((handle, area)) = torn_off {
            *self.attached_window.borrow_mut() = Some(handle);
            *payload.detached.borrow_mut() = Some((handle, area));
            self.start_drag_poll(cx);
        }
    }

    pub(crate) fn finish_drag_preview(&self, cx: &mut App) {
        self.handed_off.set(false);
        self.still_ticks.set(0);
        self.last_cursor.set(None);
        self.move_chip(None, cx);
        self.refill_spare(cx);
        self.release_idle_spares(cx);
        let attached = self.attached_window.borrow_mut().take();

        let windows: Vec<AnyWindowHandle> = self
            .areas
            .borrow()
            .iter()
            .map(|entry| entry.window)
            .collect();
        cx.defer(move |cx| {
            for handle in windows {
                let _ = handle.update(cx, |_, window, _| window.refresh());
            }
        });

        let Some(item) = self.dragged.borrow_mut().take() else {
            return;
        };
        let target = self.merge_target.borrow_mut().take();
        self.last_position.take();
        let restore = self.hidden.replace(false).then_some(attached).flatten();

        cx.defer(move |cx| {
            if let Some(target) = target {
                apply_merge(&item, target, cx);
            }
            let payload = item.value().downcast_ref::<PanelDrag>();
            let landed = payload.is_some_and(|drag| drag.landed.get());
            let closing = payload
                .and_then(|drag| drag.detached.borrow().clone())
                .and_then(|(_, area)| area.upgrade())
                .is_some_and(|area| center_panels(area.read(cx)).is_empty());
            if let Some(handle) = restore
                && !closing
            {
                let _ = handle.update(cx, |_, window, _| set_masked(window, false));
            }
            if let Some(handle) = attached
                && !landed
            {
                let _ = handle.update(cx, |_, window, _| window.activate_window());
            }
        });
    }
}

#[cfg(target_os = "macos")]
fn live_cursor(fallback: Point<Pixels>) -> Point<Pixels> {
    crate::mac::cursor_position().unwrap_or(fallback)
}

#[cfg(not(target_os = "macos"))]
fn live_cursor(fallback: Point<Pixels>) -> Point<Pixels> {
    fallback
}

pub(crate) fn hit_spot(
    entry: &AreaEntry,
    area: &DockArea,
    zone: Option<PanelZone>,
    position: Point<Pixels>,
) -> Option<MergeSpot> {
    for (placement, bounds) in entry.empty_docks.borrow().iter() {
        if bounds.contains(&position) && drop_allowed(entry.kind, zone, *placement) {
            return Some(MergeSpot::EmptyDock(*placement));
        }
    }

    for (node, bounds) in entry.groups.borrow().iter() {
        let Some(region) = placement_of(area, *node) else {
            continue;
        };
        if !drop_allowed(entry.kind, zone, region) {
            continue;
        }
        if let Some(content) = bounds.content
            && content.contains(&position)
        {
            return Some(MergeSpot::Group {
                node: *node,
                placement: split_zone_at(content, position),
            });
        }
        if let Some(tab_bar) = bounds.tab_bar
            && tab_bar.contains(&position)
        {
            return Some(MergeSpot::Group {
                node: *node,
                placement: None,
            });
        }
    }
    None
}

pub(crate) fn apply_merge(item: &AnyDrag, target: MergeTarget, cx: &mut App) {
    let already_landed = item
        .value()
        .downcast_ref::<PanelDrag>()
        .is_none_or(|drag| drag.landed.get());
    if already_landed {
        return;
    }
    let Some(area) = target.area.upgrade() else {
        return;
    };
    let _ = target.window.update(cx, |_, window, cx| {
        match target.spot {
            MergeSpot::Group { node, placement } => apply_drop_in(
                target.kind,
                &area,
                item,
                &DropTarget::Group { node, placement },
                window,
                cx,
            ),
            MergeSpot::EmptyDock(placement) => {
                if let Some(drag) = item.value().downcast_ref::<PanelDrag>() {
                    transfer_panel(drag, &area, placement, None, window, cx);
                }
            }
        }
        window.refresh();
    });
}
