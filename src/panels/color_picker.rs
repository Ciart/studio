use gpui::{
    AnyElement, IntoElement, ParentElement, Styled, black, div, img, linear_color_stop,
    linear_gradient, px, rgb, transparent_black, white,
};

pub fn color_picker() -> AnyElement {
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
