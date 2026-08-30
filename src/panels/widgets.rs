use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px, rgba, svg};

use crate::theme::MUTED;

pub fn icon(path: &'static str, width: f32, height: f32, color: u32) -> impl IntoElement {
    svg()
        .path(path)
        .w(px(width))
        .h(px(height))
        .flex_none()
        .text_color(rgba(color))
}

pub fn empty_hint(text: impl Into<SharedString>) -> impl IntoElement {
    div().text_color(rgba(MUTED)).child(text.into())
}
