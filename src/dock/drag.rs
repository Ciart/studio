use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Arc,
};

use gpui::{
    AnyWindowHandle, App, AppContext, Bounds, Entity, Pixels, Point, SharedString, Size,
    WeakEntity, Window, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, point,
    px, size,
};
use gpui_base::{
    Placement,
    dock::{AnyDrag, DockArea, DockPlacement, DropTarget, NodeId, PanelView},
};

use super::{
    AreaKind, DragPreview, HiddenPreview, PanelDrag, PanelZone, TAB_HEIGHT, TRAFFIC_LIGHT_PAD,
    detach::{apply_drop_in, detach_panel, transfer_panel},
    util::{
        EmptyDockCache, GroupBoundsCache, drop_allowed, panel_title, placement_of, split_zone_at,
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

#[derive(Clone, Copy, PartialEq)]
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
    pub(crate) preview_window: Rc<RefCell<Option<AnyWindowHandle>>>,
    pub(crate) attached_window: Rc<RefCell<Option<AnyWindowHandle>>>,
    pub(crate) promote_pending: Rc<Cell<bool>>,
    pub(crate) next_window_id: Rc<Cell<usize>>,
    pub(crate) areas: Rc<RefCell<Vec<AreaEntry>>>,
    pub(crate) dragged: Rc<RefCell<Option<AnyDrag>>>,
    pub(crate) merge_target: Rc<RefCell<Option<MergeTarget>>>,
    pub(crate) last_position: Rc<Cell<Option<Point<Pixels>>>>,
    pub(crate) detached_size: Rc<Cell<Option<Size<Pixels>>>>,
    pub(crate) grab: Rc<Cell<Grab>>,
}

impl DragState {
    pub(crate) fn new() -> Self {
        Self {
            main_area: Rc::new(RefCell::new(None)),
            main_window: Rc::new(RefCell::new(None)),
            dragging_zone: Rc::new(Cell::new(None)),
            hovered_group_accepts: Rc::new(Cell::new(true)),
            preview_window: Rc::new(RefCell::new(None)),
            attached_window: Rc::new(RefCell::new(None)),
            promote_pending: Rc::new(Cell::new(false)),
            next_window_id: Rc::new(Cell::new(0)),
            areas: Rc::new(RefCell::new(Vec::new())),
            dragged: Rc::new(RefCell::new(None)),
            merge_target: Rc::new(RefCell::new(None)),
            last_position: Rc::new(Cell::new(None)),
            detached_size: Rc::new(Cell::new(None)),
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

    pub(crate) fn promote_if_pending(&self, window: &mut Window, cx: &mut App) {
        if self.promote_pending.get() && cx.has_active_drag() {
            self.promote_pending.set(false);
            window.promote_active_drag_to_platform(cx);
        }
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
        self.promote_pending.set(true);
        *self.dragged.borrow_mut() = Some(drag.clone());
        self.grab.set(grab);

        let cursor = window.bounds().origin + window.mouse_position();

        let already_attached = drag
            .value()
            .downcast_ref::<PanelDrag>()
            .and_then(|payload| payload.detached.borrow().clone());

        if let Some((attached, _)) = already_attached {
            *self.attached_window.borrow_mut() = Some(attached);
            if attached.window_id() == window.window_handle().window_id() {
                window.set_accepts_drags(false);
            } else {
                let _ = attached.update(cx, |_, window, _| window.set_accepts_drags(false));
            }
        } else {
            self.open_chip(title.clone(), cursor, cx);
        }

        cx.set_platform_drag_moved_handler(Some(Box::new({
            let state = self.clone();
            move |position, window, cx| state.drag_session_moved(&drag, position, window, cx)
        })));

        cx.new(|_| HiddenPreview)
    }

    pub(crate) fn open_chip(&self, title: SharedString, cursor: Point<Pixels>, cx: &mut App) {
        let bounds = Bounds {
            origin: self.chip_origin(cursor),
            size: self.grab.get().size,
        };
        let preview = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: None,
                focus: false,
                show: true,
                kind: WindowKind::Overlay,
                is_movable: false,
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            move |_, cx| cx.new(|_| DragPreview { title }),
        );
        if let Ok(handle) = preview {
            *self.preview_window.borrow_mut() = Some(handle.into());
        }
    }

    pub(crate) fn with_attached<R>(
        &self,
        handle: AnyWindowHandle,
        source_window: &mut Window,
        cx: &mut App,
        act: impl FnOnce(&mut Window) -> R,
    ) -> Option<R> {
        if handle.window_id() == source_window.window_handle().window_id() {
            Some(act(source_window))
        } else {
            handle.update(cx, |_, window, _| act(window)).ok()
        }
    }

    pub(crate) fn move_attached(
        &self,
        handle: AnyWindowHandle,
        origin: Point<Pixels>,
        source_window: &mut Window,
        cx: &mut App,
    ) {
        self.with_attached(handle, source_window, cx, |window| {
            window.set_origin(origin)
        });
    }

    pub(crate) fn collapse_to_chip(
        &self,
        payload: &PanelDrag,
        handle: AnyWindowHandle,
        position: Point<Pixels>,
        source_window: &mut Window,
        cx: &mut App,
    ) {
        if self.preview_window.borrow().is_none() {
            let restore = self.with_attached(handle, source_window, cx, |window| {
                let previous = window.bounds().size;
                window.resize(size(px(1.), px(1.)));
                previous
            });
            self.detached_size.set(restore);
            self.open_chip(panel_title(&payload.view, cx), position, cx);
        }
        self.move_attached(handle, position, source_window, cx);
        if let Some(chip) = *self.preview_window.borrow() {
            let origin = self.chip_origin(position);
            let _ = chip.update(cx, |_, window, _| window.set_origin(origin));
        }
    }

    pub(crate) fn expand_to_window(
        &self,
        handle: AnyWindowHandle,
        position: Point<Pixels>,
        source_window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(chip) = self.preview_window.borrow_mut().take() {
            let _ = chip.update(cx, |_, window, _| window.remove_window());
            if let Some(size) = self.detached_size.take() {
                self.with_attached(handle, source_window, cx, |window| window.resize(size));
            }
        }
        self.move_attached(handle, self.window_origin(position), source_window, cx);
    }

    pub(crate) fn drag_session_moved(
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
        if payload.detached.borrow().is_some() {
            self.update_merge_target(&payload.view, attached, position, source_window, cx);
        }
        if let Some(handle) = attached {
            if self.merge_target.borrow().is_some() {
                self.collapse_to_chip(payload, handle, position, source_window, cx);
            } else {
                self.expand_to_window(handle, position, source_window, cx);
            }
            return;
        }

        let preview = *self.preview_window.borrow();
        let source_id = source_window.window_handle().window_id();
        let over_own_window = source_window.bounds().contains(&position)
            || cx.windows().iter().any(|handle| {
                handle.window_id() != source_id
                    && preview.is_none_or(|preview| preview.window_id() != handle.window_id())
                    && handle
                        .update(cx, |_, window, _| window.bounds().contains(&position))
                        .unwrap_or(false)
            });

        if over_own_window || zone_of(&payload.view, cx).is_none() {
            if let Some(handle) = preview {
                let origin = self.chip_origin(position);
                let _ = handle.update(cx, |_, window, _| window.set_origin(origin));
            }
            return;
        }

        if let Some(handle) = self.preview_window.borrow_mut().take() {
            let _ = handle.update(cx, |_, window, _| window.remove_window());
        }
        let torn_off = detach_panel(
            self,
            &payload.source_area,
            payload.view.clone(),
            Some(position),
            source_window,
            cx,
        );
        if let Some((handle, area)) = torn_off {
            let _ = handle.update(cx, |_, window, _| window.set_accepts_drags(false));
            *self.attached_window.borrow_mut() = Some(handle);
            *payload.detached.borrow_mut() = Some((handle, area));
        }
    }

    pub(crate) fn finish_drag_preview(&self, cx: &mut App) {
        self.promote_pending.set(false);
        cx.set_platform_drag_moved_handler(None);
        if let Some(handle) = self.preview_window.borrow_mut().take() {
            let _ = handle.update(cx, |_, window, _| window.remove_window());
        }
        let attached = self.attached_window.borrow_mut().take();
        if let Some(handle) = attached {
            let _ = handle.update(cx, |_, window, _| window.set_accepts_drags(true));
        }

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
        let position = self.last_position.take().map(|p| self.window_origin(p));
        let restore = attached.zip(self.detached_size.take());

        cx.defer(move |cx| {
            if let Some(target) = target {
                apply_merge(&item, target, cx);
            }
            let landed = item
                .value()
                .downcast_ref::<PanelDrag>()
                .is_some_and(|drag| drag.landed.get());
            if let Some((handle, previous)) = restore
                && !landed
            {
                let _ = handle.update(cx, |_, window, _| {
                    window.resize(previous);
                    if let Some(origin) = position {
                        window.set_origin(origin);
                    }
                });
            }
        });
    }
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
