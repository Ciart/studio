use gpui::{
    AnyElement, IntoElement, ObjectFit, ParentElement, SharedString, Styled, StyledImage, black,
    div, img, linear_color_stop, linear_gradient, px, rgb, rgba, svg, transparent_black, white,
};

use crate::theme::{ACCENT, CANVAS, CHIP, MUTED, OUTLINE, PANEL, RADIUS, TEXT, TRACK};

pub const PROJECT: &str = "Project";
pub const CANVAS_PANEL: &str = "Canvas";
pub const TIMELINE: &str = "Timeline";
pub const COLOR_PICKER: &str = "ColorPicker";
pub const PALETTE: &str = "Palette";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PanelKind {
    Project,
    Canvas,
    Timeline,
    ColorPicker,
    Palette,
}

impl PanelKind {
    pub fn background(self) -> u32 {
        match self {
            PanelKind::Canvas => CANVAS,
            PanelKind::Timeline => TRACK,
            _ => PANEL,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            PanelKind::Project => PROJECT,
            PanelKind::Canvas => CANVAS_PANEL,
            PanelKind::Timeline => TIMELINE,
            PanelKind::ColorPicker => COLOR_PICKER,
            PanelKind::Palette => PALETTE,
        }
    }

    pub fn body(self) -> AnyElement {
        match self {
            PanelKind::Project => project(),
            PanelKind::Canvas => canvas(),
            PanelKind::Timeline => timeline(),
            PanelKind::ColorPicker => color_picker(),
            PanelKind::Palette => div().size_full().into_any_element(),
        }
    }
}

pub fn icon(path: &'static str, width: f32, height: f32, color: u32) -> impl IntoElement {
    svg()
        .path(path)
        .w(px(width))
        .h(px(height))
        .flex_none()
        .text_color(rgba(color))
}

fn project() -> AnyElement {
    div()
        .size_full()
        .relative()
        .child(
            img("images/sprite.png")
                .absolute()
                .left(px(17.))
                .top(px(11.))
                .size(px(64.))
                .rounded(px(RADIUS))
                .border_1()
                .border_color(rgba(OUTLINE)),
        )
        .child(
            div()
                .absolute()
                .left(px(35.))
                .top(px(80.))
                .text_color(rgba(TEXT))
                .child("Hello"),
        )
        .into_any_element()
}

fn canvas() -> AnyElement {
    div()
        .size_full()
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .size(px(256.))
                .bg(white())
                .child(img("images/sprite.png").size(px(256.))),
        )
        .child(
            div()
                .absolute()
                .left(px(11.))
                .bottom(px(10.))
                .size(px(24.))
                .rounded(px(RADIUS))
                .border_1()
                .border_color(rgba(OUTLINE))
                .flex()
                .items_center()
                .justify_center()
                .child(icon("icons/play_arrow.svg", 16., 16., TEXT)),
        )
        .child(
            div()
                .absolute()
                .right(px(9.))
                .bottom(px(10.))
                .w(px(56.))
                .h(px(24.))
                .rounded(px(RADIUS))
                .border_1()
                .border_color(rgba(OUTLINE))
                .flex()
                .items_center()
                .justify_center()
                .text_color(rgba(TEXT))
                .child("100%"),
        )
        .into_any_element()
}

fn frame_chip(label: &'static str, left: f32, active: bool) -> impl IntoElement {
    div()
        .absolute()
        .left(px(left))
        .top(px(7.))
        .w(px(21.))
        .h(px(16.))
        .rounded(px(4.))
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(10.))
        .text_color(white())
        .bg(rgb(if active { ACCENT } else { CHIP }))
        .child(label)
}

fn cel(left: f32, top: f32, width: f32) -> impl IntoElement {
    img("images/sprite.png")
        .object_fit(ObjectFit::Cover)
        .absolute()
        .left(px(left))
        .top(px(top))
        .w(px(width))
        .h(px(25.))
        .rounded(px(5.))
        .border_1()
        .border_color(rgba(OUTLINE))
}

fn layer_row(top: f32, label: &'static str, visible: bool, unlocked: bool) -> impl IntoElement {
    div()
        .absolute()
        .left_0()
        .top(px(top))
        .w(px(160.))
        .h(px(30.))
        .flex()
        .items_center()
        .p(px(4.))
        .child(icon(
            if visible {
                "icons/eye.svg"
            } else {
                "icons/eye_off.svg"
            },
            20.,
            20.,
            TEXT,
        ))
        .child(icon(
            if unlocked {
                "icons/unlock.svg"
            } else {
                "icons/lock.svg"
            },
            20.,
            20.,
            TEXT,
        ))
        .child(
            div()
                .absolute()
                .left(px(52.))
                .text_color(white())
                .child(label),
        )
}

fn timeline() -> AnyElement {
    let track = div()
        .absolute()
        .left(px(160.))
        .top_0()
        .right_0()
        .bottom_0()
        .border_l_1()
        .border_color(rgb(PANEL))
        .child(frame_chip("1", 11., false))
        .child(frame_chip("2", 42., true))
        .child(frame_chip("3", 73., false))
        .child(cel(9., 32., 25.))
        .child(cel(40., 32., 25.))
        .child(cel(71., 32., 25.))
        .child(cel(9., 63., 56.))
        .child(
            div()
                .absolute()
                .left(px(52.))
                .top(px(20.))
                .w(px(1.))
                .bottom_0()
                .bg(rgb(ACCENT)),
        );

    let layers = div()
        .absolute()
        .left_0()
        .top_0()
        .w(px(160.))
        .h_full()
        .bg(rgb(PANEL))
        .child(
            div()
                .absolute()
                .left_0()
                .top_0()
                .w(px(160.))
                .h(px(30.))
                .bg(rgb(TRACK))
                .child(
                    div()
                        .absolute()
                        .left(px(4.))
                        .top(px(5.))
                        .child(icon("icons/plus.svg", 20., 20., TEXT)),
                ),
        )
        .child(layer_row(30., "Layer 2", true, true))
        .child(layer_row(61., "Layer 1", false, false));

    div()
        .size_full()
        .relative()
        .overflow_hidden()
        .child(track)
        .child(layers)
        .into_any_element()
}

fn color_picker() -> AnyElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .justify_center()
        .p(px(12.))
        .child(
            div()
                .relative()
                .w_full()
                .h(px(268.))
                .flex_none()
                .child(
                    img("images/hue_ring.png")
                        .absolute()
                        .left_0()
                        .top_0()
                        .size(px(268.)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(62.))
                        .top(px(62.))
                        .size(px(143.))
                        .bg(linear_gradient(
                            90.,
                            linear_color_stop(white(), 0.),
                            linear_color_stop(rgb(0xff0000), 1.),
                        ))
                        .child(div().size_full().bg(linear_gradient(
                            180.,
                            linear_color_stop(transparent_black(), 0.),
                            linear_color_stop(black(), 1.),
                        ))),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(193.))
                        .top(px(50.))
                        .size(px(24.))
                        .rounded_full()
                        .bg(rgb(0xff0000))
                        .border_2()
                        .border_color(white()),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(128.))
                        .top(px(-5.))
                        .w(px(13.))
                        .h(px(24.))
                        .bg(rgb(0xff0000))
                        .border_2()
                        .border_color(white()),
                ),
        )
        .into_any_element()
}

pub fn empty_hint(text: impl Into<SharedString>) -> impl IntoElement {
    div().text_color(rgba(MUTED)).child(text.into())
}
