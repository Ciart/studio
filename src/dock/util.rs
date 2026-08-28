use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use gpui::{App, Bounds, Entity, Pixels, Point, SharedString, point};
use gpui_base::{
    Placement,
    dock::{DockArea, DockPlacement, NodeId, PaneRef, PanelId, PanelView},
};

use super::{AreaKind, DockPanel, PanelZone};

pub(crate) fn panel_title(panel: &Arc<dyn PanelView>, cx: &App) -> SharedString {
    panel
        .as_any()
        .downcast_ref::<Entity<DockPanel>>()
        .map(|panel| panel.read(cx).title.clone())
        .unwrap_or_else(|| panel.panel_name(cx).into())
}

pub(crate) fn zone_of(panel: &Arc<dyn PanelView>, cx: &App) -> Option<PanelZone> {
    panel
        .as_any()
        .downcast_ref::<Entity<DockPanel>>()
        .map(|panel| panel.read(cx).zone)
}

pub(crate) fn grab_within(press: Point<Pixels>, bounds: Bounds<Pixels>) -> Point<Pixels> {
    point(
        press.x.max(bounds.left()).min(bounds.right()) - bounds.left(),
        press.y.max(bounds.top()).min(bounds.bottom()) - bounds.top(),
    )
}

pub(crate) fn panel_entity(panel: &Arc<dyn PanelView>) -> Option<Entity<DockPanel>> {
    panel.as_any().downcast_ref::<Entity<DockPanel>>().cloned()
}

pub(crate) fn group_len(area: &DockArea, node: NodeId) -> Option<usize> {
    let placement = placement_of(area, node)?;
    let group = area.layout(placement)?.find_node(node)?;
    match group.kind() {
        PaneRef::Tabs { panels, .. } => Some(panels.len()),
        _ => None,
    }
}

pub(crate) fn placement_of(area: &DockArea, node: NodeId) -> Option<DockPlacement> {
    [
        DockPlacement::Center,
        DockPlacement::Left,
        DockPlacement::Right,
        DockPlacement::Bottom,
    ]
    .into_iter()
    .find(|&placement| {
        area.layout(placement)
            .is_some_and(|tree| tree.find_node(node).is_some())
    })
}

pub(crate) fn drop_allowed(kind: AreaKind, zone: Option<PanelZone>, region: DockPlacement) -> bool {
    match kind {
        AreaKind::Main => match zone {
            Some(PanelZone::Canvas) => region == DockPlacement::Center,
            Some(PanelZone::Dock) => region != DockPlacement::Center,
            None => false,
        },
        AreaKind::Detached(host) => zone == Some(host),
    }
}

pub(crate) fn split_zone_at(bounds: Bounds<Pixels>, position: Point<Pixels>) -> Option<Placement> {
    if position.x < bounds.left() + bounds.size.width * 0.35 {
        Some(Placement::Left)
    } else if position.x > bounds.left() + bounds.size.width * 0.65 {
        Some(Placement::Right)
    } else if position.y < bounds.top() + bounds.size.height * 0.35 {
        Some(Placement::Top)
    } else if position.y > bounds.top() + bounds.size.height * 0.65 {
        Some(Placement::Bottom)
    } else {
        None
    }
}

pub(crate) fn center_panels(area: &DockArea) -> Vec<PanelId> {
    area.layout(DockPlacement::Center)
        .map(|tree| tree.panels().collect())
        .unwrap_or_default()
}

#[derive(Clone, Copy, Default)]
pub(crate) struct GroupBounds {
    pub(crate) tab_bar: Option<Bounds<Pixels>>,
    pub(crate) content: Option<Bounds<Pixels>>,
}

pub(crate) type GroupBoundsCache = Rc<RefCell<HashMap<NodeId, GroupBounds>>>;
pub(crate) type EmptyDockCache = Rc<RefCell<Vec<(DockPlacement, Bounds<Pixels>)>>>;
