use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use gpui::{
    AnyElement, AnyView, AnyWindowHandle, App, Axis, Bounds, DispatchPhase, Div, Entity,
    InteractiveElement, IntoElement, MouseButton, MouseMoveEvent, MouseUpEvent, ParentElement,
    Pixels, Point, Stateful, StatefulInteractiveElement, Styled, WeakEntity, Window, canvas, div,
    prelude::*, px, rgb, rgba,
};
use gpui_base::{
    ElementExt, Placement,
    dock::{
        AnyDrag, DockArea, DockAreaRenderer, DockContext, DockPlacement, DropIndicator,
        InsertTarget, NodeId, PanelId, TabGroupContext, TabGroupRenderer, TileContext,
        TilesRenderer,
    },
    h_flex,
};

use super::{
    AreaKind, PanelDrag, PanelZone, TAB_HEIGHT, TITLEBAR_HEIGHT, TRAFFIC_LIGHT_PAD,
    detach::{living_source, transfer_panel},
    drag::{AreaEntry, DragState, Grab, MergeSpot},
    util::{
        EmptyDockCache, GroupBoundsCache, background_of, center_panels, drop_allowed, grab_within,
        panel_title, placement_of, zone_of,
    },
};
use crate::{
    caption,
    panels::{empty_hint, icon},
    theme::{BACKDROP, CANVAS, DROP_TARGET, FONT, FONT_SIZE, HAIRLINE, MUTED, PANEL, RADIUS, TEXT},
};

#[derive(Clone)]
pub struct DockSkin {
    kind: AreaKind,
    window: AnyWindowHandle,
    area: WeakEntity<DockArea>,
    drag: DragState,
    resizing: Rc<RefCell<Option<DockContext>>>,
    empty_docks: Rc<RefCell<Vec<DockPlacement>>>,
    empty_center_root: Rc<Cell<Option<NodeId>>>,
    group_bounds: GroupBoundsCache,
    empty_dock_bounds: EmptyDockCache,
    tab_bounds: Rc<RefCell<HashMap<PanelId, Bounds<Pixels>>>>,
    press: Rc<Cell<Point<Pixels>>>,
}

impl DockSkin {
    pub(crate) fn new(
        kind: AreaKind,
        window: AnyWindowHandle,
        area: WeakEntity<DockArea>,
        drag: DragState,
    ) -> Self {
        let group_bounds: GroupBoundsCache = Rc::new(RefCell::new(HashMap::new()));
        let empty_dock_bounds: EmptyDockCache = Rc::new(RefCell::new(Vec::new()));
        drag.register_area(AreaEntry {
            kind,
            window,
            area: area.clone(),
            groups: group_bounds.clone(),
            empty_docks: empty_dock_bounds.clone(),
        });
        Self {
            kind,
            window,
            area,
            drag,
            resizing: Rc::new(RefCell::new(None)),
            empty_docks: Rc::new(RefCell::new(Vec::new())),
            empty_center_root: Rc::new(Cell::new(None)),
            group_bounds,
            empty_dock_bounds,
            tab_bounds: Rc::new(RefCell::new(HashMap::new())),
            press: Rc::new(Cell::new(Point::default())),
        }
    }

    pub fn window_drag_active(&self) -> bool {
        self.drag.window_drag_active()
    }

    fn merge_spot(&self) -> Option<MergeSpot> {
        let target = self.drag.merge_target.borrow();
        let target = target.as_ref()?;
        (target.area.entity_id() == self.area.entity_id()).then_some(target.spot)
    }

    pub fn dock_area(
        id: &'static str,
        window: &mut Window,
        cx: &mut App,
    ) -> (Entity<DockArea>, Rc<Self>) {
        let drag = DragState::new();
        let handle = window.window_handle();
        let mut skin = None;
        let area = cx.new(|cx| {
            let this = Rc::new(DockSkin::new(
                AreaKind::Main,
                handle,
                cx.weak_entity(),
                drag.clone(),
            ));
            skin = Some(this.clone());
            DockArea::new(id, Some(1), window, cx).with_renderer(this)
        });
        *drag.main_area.borrow_mut() = Some(area.downgrade());
        *drag.main_window.borrow_mut() = Some(window.window_handle());
        drag.open_chip(cx);
        cx.on_window_closed({
            let drag = drag.clone();
            move |cx, _| crate::dock::detach::close_spare_if_last(&drag, cx)
        })
        .detach();
        (
            area,
            skin.expect("DockSkin built in the DockArea constructor"),
        )
    }

    pub fn sync_empty_regions(&self, area: &DockArea, cx: &App) {
        *self.empty_docks.borrow_mut() = [
            DockPlacement::Left,
            DockPlacement::Right,
            DockPlacement::Bottom,
        ]
        .into_iter()
        .filter(|&placement| area.is_dock_open(placement) && area.is_empty(placement, cx))
        .collect();

        self.empty_center_root.set(
            area.is_empty(DockPlacement::Center, cx)
                .then(|| {
                    area.layout(DockPlacement::Center)
                        .map(|tree| tree.root().id())
                })
                .flatten(),
        );

        let docks = self.empty_docks.borrow();
        self.empty_dock_bounds
            .borrow_mut()
            .retain(|(placement, _)| match placement {
                DockPlacement::Center => self.empty_center_root.get().is_some(),
                placement => docks.contains(placement),
            });
    }

    fn group_zone(&self, node: NodeId, cx: &App) -> PanelZone {
        match self.kind {
            AreaKind::Detached(zone) => zone,
            AreaKind::Main => self
                .area
                .upgrade()
                .and_then(|area| placement_of(area.read(cx), node))
                .map_or(PanelZone::Canvas, |placement| match placement {
                    DockPlacement::Center => PanelZone::Canvas,
                    _ => PanelZone::Dock,
                }),
        }
    }

    fn render_resize_strip(&self, dock: &DockContext) -> impl IntoElement {
        let placement = dock.placement();
        let dock = dock.clone();
        let resizing = self.resizing.clone();

        div()
            .absolute()
            .flex()
            .items_center()
            .justify_center()
            .map(|this| match placement {
                DockPlacement::Bottom => {
                    this.top_0().left_0().w_full().h(px(4.)).cursor_row_resize()
                }
                DockPlacement::Right => {
                    this.top_0().left_0().h_full().w(px(4.)).cursor_col_resize()
                }
                _ => this
                    .top_0()
                    .right_0()
                    .h_full()
                    .w(px(4.))
                    .cursor_col_resize(),
            })
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                cx.stop_propagation();
                *resizing.borrow_mut() = Some(dock.clone());
            })
    }

    fn render_empty_dock(&self, placement: DockPlacement) -> impl IntoElement {
        let area = self.area.clone();
        let kind = self.kind;
        let merging = self.merge_spot() == Some(MergeSpot::EmptyDock(placement));
        div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .when(merging, |this| this.bg(rgba(DROP_TARGET)))
            .on_prepaint({
                let bounds_cache = self.empty_dock_bounds.clone();
                move |bounds, _, _| {
                    let mut cache = bounds_cache.borrow_mut();
                    cache.retain(|(cached, _)| *cached != placement);
                    cache.push((placement, bounds));
                }
            })
            .child(empty_hint("Drop panels here"))
            .drag_over::<AnyDrag>({
                let drag_state = self.drag.clone();
                move |style, _, _, _| match !drag_state.window_drag_active()
                    && drop_allowed(kind, drag_state.dragging_zone.get(), placement)
                {
                    true => style.bg(rgba(DROP_TARGET)),
                    false => style,
                }
            })
            .on_drop({
                let drag_state = self.drag.clone();
                move |item: &AnyDrag, window, cx| {
                    if drag_state.window_drag_active() {
                        return;
                    }
                    let Some(drag) = item.value().downcast_ref::<PanelDrag>() else {
                        return;
                    };
                    if !drop_allowed(kind, zone_of(&drag.view, cx), placement) {
                        return;
                    }
                    let Some(area) = area.upgrade() else {
                        return;
                    };

                    let same_area = living_source(drag).entity_id() == area.entity_id();
                    if same_area {
                        let root = area.read(cx).layout(placement).map(|tree| tree.root().id());
                        let Some(root) = root else {
                            return;
                        };
                        if drag.landed.replace(true) {
                            return;
                        }
                        area.update(cx, |area, cx| {
                            area.move_panel(
                                drag.panel,
                                InsertTarget::Split {
                                    node: root,
                                    placement: Placement::Right,
                                    size: None,
                                },
                                window,
                                cx,
                            )
                        });
                    } else {
                        transfer_panel(drag, &area, placement, None, window, cx);
                    }
                }
            })
    }
}

impl DockAreaRenderer for DockSkin {
    fn frame(&self, _: &mut Window, _: &mut App) -> Stateful<Div> {
        let dragging = self.resizing.clone();
        let finished = self.resizing.clone();
        let drag = self.drag.clone();

        div()
            .id("workspace-dock")
            .size_full()
            .flex()
            .flex_row()
            .overflow_hidden()
            .bg(rgb(BACKDROP))
            .font_family(FONT)
            .text_size(px(FONT_SIZE))
            .text_color(rgba(TEXT))
            .on_mouse_move(move |event: &MouseMoveEvent, window, cx| {
                let dock = dragging.borrow().clone();
                let Some(dock) = dock else {
                    return;
                };
                dock.resize_to(event.position, window, cx);
            })
            .on_mouse_up(MouseButton::Left, move |_: &MouseUpEvent, _, _| {
                finished.borrow_mut().take();
            })
            .child(
                canvas(
                    |_, _, _| (),
                    move |_, _, window, _| {
                        let drag = drag.clone();
                        window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, cx| {
                            if phase == DispatchPhase::Bubble {
                                drag.pointer_moved(window, cx);
                            }
                        });
                    },
                )
                .absolute()
                .w_0()
                .h_0(),
            )
    }

    fn center_frame(&self, _: &mut Window, _: &mut App) -> Stateful<Div> {
        div()
            .id("workspace-dock-center")
            .flex()
            .flex_1()
            .flex_col()
            .overflow_hidden()
    }

    fn split_frame(&self, node: NodeId, _: Axis, _: &mut Window, _: &mut App) -> Stateful<Div> {
        div()
            .id(("workspace-dock-split", node.as_u64()))
            .size_full()
            .flex_1()
            .min_h(px(0.))
            .overflow_hidden()
            .when(self.empty_center_root.get() == Some(node), |this| {
                this.relative().child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .flex()
                        .child(self.render_empty_dock(DockPlacement::Center)),
                )
            })
    }

    fn render_dock(
        &self,
        dock: &DockContext,
        content: AnyElement,
        _: &mut Window,
        _: &mut App,
    ) -> AnyElement {
        if !dock.is_open() {
            return div().into_any_element();
        }

        let placement = dock.placement();
        let empty = self.empty_docks.borrow().contains(&placement);

        div()
            .flex()
            .flex_none()
            .relative()
            .overflow_hidden()
            .map(|this| match placement {
                DockPlacement::Bottom => this.w_full().h(dock.size()).flex_col(),
                DockPlacement::Right => this.h_full().w(dock.size()).flex_row().ml(px(1.)),
                _ => this.h_full().w(dock.size()).flex_row().mr(px(1.)),
            })
            .map(|this| match empty {
                false => this.child(content),
                true => this.child(self.render_empty_dock(placement)),
            })
            .child(self.render_resize_strip(dock))
            .into_any_element()
    }

    fn tab_group_renderer(&self) -> Rc<dyn TabGroupRenderer> {
        Rc::new(self.clone())
    }

    fn tiles_renderer(&self) -> Rc<dyn TilesRenderer> {
        Rc::new(self.clone())
    }
}

impl TabGroupRenderer for DockSkin {
    fn frame(&self, group: &TabGroupContext, _: &mut Window, cx: &mut App) -> Stateful<Div> {
        let zone = self.group_zone(group.node(), cx);
        let in_dock = zone == PanelZone::Dock;
        self.drag.hovered_group_accepts.set(
            !cx.has_active_drag()
                || (self.drag.drag_accepted(zone) && !self.drag.window_drag_active()),
        );
        div()
            .id("tab-group")
            .size_full()
            .flex()
            .flex_col()
            .min_h(px(0.))
            .overflow_hidden()
            .rounded(px(RADIUS))
            .bg(rgb(if in_dock { PANEL } else { BACKDROP }))
            .when(!self.kind.is_detached(), |this| this.mb(px(1.)))
    }

    fn content_frame(
        &self,
        group: &TabGroupContext,
        _: &mut Window,
        cx: &mut App,
    ) -> Stateful<Div> {
        let node = group.node();
        let in_dock = self.group_zone(node, cx) == PanelZone::Dock;
        let background = group
            .panels()
            .get(group.active_ix())
            .and_then(|panel| background_of(panel, cx))
            .unwrap_or(if in_dock { PANEL } else { CANVAS });
        let bounds_cache = self.group_bounds.clone();

        div()
            .id("tab-group-content")
            .relative()
            .flex_1()
            .min_h(px(0.))
            .overflow_hidden()
            .bg(rgb(background))
            .rounded_b(px(RADIUS))
            .when(!in_dock, |this| this.rounded_tr(px(RADIUS)))
            .on_prepaint(move |bounds, _, _| {
                bounds_cache.borrow_mut().entry(node).or_default().content = Some(bounds);
            })
    }

    fn render_active_panel(
        &self,
        panel: AnyView,
        group: &TabGroupContext,
        _: &mut Window,
        _: &mut App,
    ) -> AnyElement {
        let node = group.node();
        let merging = match self.merge_spot() {
            Some(MergeSpot::Group {
                node: target,
                placement,
            }) if target == node => Some(placement),
            _ => None,
        };

        div()
            .relative()
            .size_full()
            .child(panel)
            .when_some(merging, |this, placement| {
                this.child(
                    div()
                        .absolute()
                        .bg(rgba(DROP_TARGET))
                        .map(|this| match placement {
                            Some(Placement::Left) => this.top_0().left_0().w_1_2().h_full(),
                            Some(Placement::Right) => this.top_0().right_0().w_1_2().h_full(),
                            Some(Placement::Top) => this.top_0().left_0().w_full().h_1_2(),
                            Some(Placement::Bottom) => this.bottom_0().left_0().w_full().h_1_2(),
                            None => this.top_0().left_0().size_full(),
                        }),
                )
            })
            .into_any_element()
    }

    fn render_tab_bar(
        &self,
        group: &TabGroupContext,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let group_zone = self.group_zone(group.node(), cx);
        let in_dock = group_zone == PanelZone::Dock;
        let window_handle = window.window_handle();
        let single_window_panel = self.kind.is_detached()
            && self
                .area
                .upgrade()
                .map(|area| center_panels(area.read(cx)).len() == 1)
                .unwrap_or(false);

        let in_titlebar = self.kind.is_detached();
        let merging = self.merge_spot()
            == Some(MergeSpot::Group {
                node: group.node(),
                placement: None,
            });

        h_flex()
            .h(px(if in_titlebar {
                TITLEBAR_HEIGHT
            } else {
                TAB_HEIGHT
            }))
            .flex_none()
            .items_center()
            .overflow_hidden()
            .when(in_dock, |this| {
                this.bg(rgb(PANEL))
                    .pr(px(9.))
                    .justify_between()
                    .when(!in_titlebar, |this| {
                        this.rounded_t(px(RADIUS))
                            .border_t_1()
                            .border_color(rgba(HAIRLINE))
                    })
            })
            .map(|this| match (in_titlebar, in_dock) {
                (true, _) => this.pl(px(TRAFFIC_LIGHT_PAD)),
                (false, true) => this.pl(px(12.)),
                (false, false) => this,
            })
            .when(merging, |this| this.bg(rgba(DROP_TARGET)))
            .on_prepaint({
                let node = group.node();
                let bounds_cache = self.group_bounds.clone();
                move |bounds, _, _| {
                    bounds_cache.borrow_mut().entry(node).or_default().tab_bar = Some(bounds);
                }
            })
            .drag_over::<AnyDrag>({
                let skin = self.clone();
                move |style, _, _, _| match !skin.drag.window_drag_active()
                    && skin.drag.drag_accepted(group_zone)
                {
                    true => style.bg(rgba(DROP_TARGET)),
                    false => style,
                }
            })
            .on_drop({
                let group = group.clone();
                let skin = self.clone();
                move |item: &AnyDrag, window, cx| {
                    if skin.drag.window_drag_active() {
                        return;
                    }
                    group.drop_item(item.clone(), None, window, cx)
                }
            })
            .child(h_flex().h_full().items_center().children(
                group.panels().iter().enumerate().map(|(ix, panel)| {
                    let selected = ix == group.active_ix();
                    let title = panel_title(panel, cx);
                    let zone = zone_of(panel, cx);
                    let panel_id = panel.panel_id(cx);
                    let drag = AnyDrag::new(PanelDrag {
                        panel: panel_id,
                        source: Some(group.node()),
                        source_area: self.area.clone(),
                        source_window: window_handle,
                        view: panel.clone(),
                        detached: RefCell::new(
                            single_window_panel.then(|| (window_handle, self.area.clone())),
                        ),
                        landed: Cell::new(false),
                    });
                    div()
                        .id(("tab", ix))
                        .on_mouse_down(MouseButton::Left, {
                            let press = self.press.clone();
                            let drag_state = self.drag.clone();
                            let title = title.clone();
                            move |event, window, cx| {
                                press.set(event.position);
                                drag_state.prime_chip(title.clone(), window.scale_factor(), cx);
                            }
                        })
                        .on_prepaint({
                            let cache = self.tab_bounds.clone();
                            move |bounds, _, _| {
                                cache.borrow_mut().insert(panel_id, bounds);
                            }
                        })
                        .h_full()
                        .flex()
                        .items_center()
                        .cursor_pointer()
                        .map(|this| match in_dock {
                            true => this,
                            false => this
                                .rounded_t(px(RADIUS))
                                .border_t_1()
                                .border_color(rgba(if selected { HAIRLINE } else { 0 }))
                                .when(selected, |this| this.bg(rgb(CANVAS))),
                        })
                        .text_color(rgba(if selected { TEXT } else { MUTED }))
                        .child(
                            div()
                                .h_full()
                                .flex()
                                .items_center()
                                .map(|this| match in_dock {
                                    true => this.pr(px(12.)),
                                    false => this.pl(px(12.)).pr(px(4.)),
                                })
                                .child(title.clone())
                                .when(!in_dock, |this| {
                                    this.child(icon("icons/close_small.svg", 20., 20., TEXT))
                                }),
                        )
                        .on_click({
                            let group = group.clone();
                            move |_, window, cx| group.select_tab(ix, window, cx)
                        })
                        .on_drag(drag.clone(), {
                            let state = self.drag.clone();
                            let cache = self.tab_bounds.clone();
                            let press = self.press.clone();
                            move |_, _, window, cx| {
                                let grab = cache
                                    .borrow()
                                    .get(&panel_id)
                                    .map(|bounds| Grab {
                                        offset: grab_within(press.get(), *bounds),
                                        size: bounds.size,
                                    })
                                    .unwrap_or_default();
                                let preview = state.start_drag_preview(
                                    title.clone(),
                                    zone,
                                    drag.clone(),
                                    grab,
                                    window,
                                    cx,
                                );
                                cx.observe_release(&preview, {
                                    let state = state.clone();
                                    move |_, cx| state.finish_drag_preview(cx)
                                })
                                .detach();
                                preview
                            }
                        })
                }),
            ))
            .when(in_titlebar, |this| {
                this.child(div().id("titlebar-drag").flex_1().h_full().on_mouse_down(
                    MouseButton::Left,
                    |event, window, _| {
                        if event.click_count >= 2 {
                            window.titlebar_double_click();
                        } else {
                            window.start_window_move();
                        }
                    },
                ))
                .child(caption::caption_buttons(window))
            })
            .when(in_dock, |this| {
                this.child(icon("icons/more_horiz.svg", 20., 8., MUTED))
            })
            .into_any_element()
    }

    fn render_drop_indicator(
        &self,
        indicator: DropIndicator,
        _: &mut Window,
        _: &mut App,
    ) -> Option<AnyElement> {
        if !self.drag.hovered_group_accepts.get() {
            return None;
        }

        let to = indicator.to();
        Some(
            div()
                .absolute()
                .left(to.origin().x)
                .top(to.origin().y)
                .w(to.size().width)
                .h(to.size().height)
                .bg(rgba(DROP_TARGET))
                .into_any_element(),
        )
    }
}

impl TilesRenderer for DockSkin {
    fn render_drag_bar(&self, tile: &TileContext, _: &mut Window, _: &mut App) -> AnyElement {
        div()
            .h(px(28.))
            .on_mouse_down(MouseButton::Left, {
                let tile = tile.clone();
                move |event, window, cx| tile.begin_move(event.position, window, cx)
            })
            .into_any_element()
    }

    fn grid_size(&self, _: &App) -> Pixels {
        px(8.)
    }
}
