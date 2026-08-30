use gpui::{
    AnyElement, IntoElement, ObjectFit, ParentElement, Styled, StyledImage, div, img, px, rgb,
    rgba, white,
};

use crate::{
    panels::widgets::icon,
    theme::{ACCENT, CHIP, OUTLINE, PANEL, TEXT, TRACK},
};

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

pub fn timeline() -> AnyElement {
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
                .child(div().absolute().left(px(4.)).top(px(5.)).child(icon(
                    "icons/plus.svg",
                    20.,
                    20.,
                    TEXT,
                ))),
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
