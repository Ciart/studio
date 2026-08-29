mod assets;
mod caption;
mod dock;
mod fullscreen;
#[cfg(target_os = "macos")]
mod mac;
mod panels;
mod theme;
mod workspace;

use gpui::{
    App, AppContext, Bounds, TitlebarOptions, WindowBounds, WindowOptions, point, px, size,
};
use gpui_platform::application;

use crate::{assets::Assets, workspace::Workspace};

fn main() {
    application().with_assets(Assets).run(|cx: &mut App| {
        let design = size(px(1512.), px(982.));
        let window_size = match cx.primary_display() {
            Some(display) => {
                let screen = display.bounds().size;
                size(
                    design.width.min(screen.width - px(48.)),
                    design.height.min(screen.height - px(96.)),
                )
            }
            None => design,
        };
        let bounds = Bounds::centered(None, window_size, cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Ciart Studio".into()),
                    appears_transparent: cfg!(not(target_os = "macos")),
                    traffic_light_position: cfg!(not(target_os = "macos"))
                        .then(|| point(px(13.0), px(13.0))),
                }),
                ..Default::default()
            },
            |window, cx| {
                #[cfg(target_os = "macos")]
                {
                    mac::force_dark_appearance();
                    mac::attach_toolbar(window, "Ciart Studio");
                    mac::attach_dock_buttons(window);
                }
                cx.new(|cx| Workspace::new(window, cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
