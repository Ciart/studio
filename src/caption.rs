use gpui::prelude::FluentBuilder as _;
use gpui::{
    InteractiveElement as _, IntoElement, ParentElement, StatefulInteractiveElement as _, Styled,
    Window, WindowControlArea, div, px, rgb, rgba, white,
};

use crate::theme::{HAIRLINE, OUTLINE, TEXT};

const BUTTON_WIDTH: f32 = 46.;
const GLYPH_SIZE: f32 = 10.;
const CLOSE_HOVER: u32 = 0xe81123;
const CLOSE_ACTIVE: u32 = 0xf1707a;

const MINIMIZE: &str = "\u{e921}";
const MAXIMIZE: &str = "\u{e922}";
const RESTORE: &str = "\u{e923}";
const CLOSE: &str = "\u{e8bb}";

// Windows only: macOS draws its own traffic lights, and on Linux gpui never
// reports the control areas back (`on_hit_test_window_control` is a no-op
// there), so the buttons would render dead.
fn app_draws_caption_buttons() -> bool {
    cfg!(target_os = "windows")
}

pub fn caption_buttons_width() -> f32 {
    if app_draws_caption_buttons() {
        BUTTON_WIDTH * 3.
    } else {
        0.
    }
}

pub fn caption_buttons(window: &Window) -> impl IntoElement {
    let maximized = window.is_maximized();
    let row = div().h_full().flex().flex_none();
    if !app_draws_caption_buttons() {
        return row;
    }

    row.child(button(
        "caption-min",
        WindowControlArea::Min,
        MINIMIZE,
        false,
    ))
    .child(button(
        "caption-max",
        WindowControlArea::Max,
        if maximized { RESTORE } else { MAXIMIZE },
        false,
    ))
    .child(button(
        "caption-close",
        WindowControlArea::Close,
        CLOSE,
        true,
    ))
}

// Hover styling only repaints on stateful elements, so a caption button inside
// a non-client hit-test region has to carry an id.
fn button(
    id: &'static str,
    area: WindowControlArea,
    glyph: &'static str,
    is_close: bool,
) -> impl IntoElement {
    div()
        .id(id)
        .w(px(BUTTON_WIDTH))
        .h_full()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .font_family(caption_font())
        .text_size(px(GLYPH_SIZE))
        .text_color(rgba(TEXT))
        .occlude()
        .window_control_area(area)
        .when(is_close, |el| {
            el.hover(|s| s.bg(rgb(CLOSE_HOVER)).text_color(white()))
                .active(|s| s.bg(rgb(CLOSE_ACTIVE)).text_color(white()))
        })
        .when(!is_close, |el| {
            el.hover(|s| s.bg(rgba(HAIRLINE)))
                .active(|s| s.bg(rgba(OUTLINE)))
        })
        .child(glyph)
}

// Windows 11 (build 22000+) ships Segoe Fluent Icons; Windows 10 only has
// Segoe MDL2 Assets. The glyph codepoints are the same in both.
#[cfg(target_os = "windows")]
fn caption_font() -> &'static str {
    static FONT: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| {
        #[repr(C)]
        struct OsVersionInfo {
            size: u32,
            major: u32,
            minor: u32,
            build: u32,
            platform: u32,
            csd: [u16; 128],
        }
        #[link(name = "ntdll")]
        unsafe extern "system" {
            fn RtlGetVersion(info: *mut OsVersionInfo) -> i32;
        }
        let mut info = OsVersionInfo {
            size: std::mem::size_of::<OsVersionInfo>() as u32,
            major: 0,
            minor: 0,
            build: 0,
            platform: 0,
            csd: [0; 128],
        };
        if unsafe { RtlGetVersion(&mut info) } == 0 && info.build >= 22000 {
            "Segoe Fluent Icons"
        } else {
            "Segoe MDL2 Assets"
        }
    });
    &FONT
}

#[cfg(not(target_os = "windows"))]
fn caption_font() -> &'static str {
    "Segoe Fluent Icons"
}
