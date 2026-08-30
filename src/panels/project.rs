use gpui::{AnyElement, IntoElement, ParentElement, Styled, div, img, px, rgba};

use crate::theme::{OUTLINE, RADIUS, TEXT};

pub fn project() -> AnyElement {
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
