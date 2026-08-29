use gpui::{Pixels, SharedString, Size};

#[cfg(target_os = "windows")]
pub(crate) use native::Chip;

#[cfg(target_os = "macos")]
pub(crate) use overlay::Chip;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub(crate) use fallback::Chip;

#[derive(Clone)]
pub(crate) struct ChipContent {
    pub(crate) title: SharedString,
    pub(crate) size: Size<Pixels>,
    pub(crate) scale: f32,
}

#[cfg(target_os = "windows")]
mod native {
    use std::cell::{Cell, RefCell};
    use std::ffi::c_void;

    use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
    use windows::Win32::Graphics::Dwm::{DWMWA_TRANSITIONS_FORCEDISABLED, DwmSetWindowAttribute};
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::WindowsAndMessaging::*;
    use windows::core::{BOOL, PCWSTR, w};

    use super::ChipContent;
    use crate::theme::{HAIRLINE, PANEL, RADIUS, TEXT};
    use gpui::{App, Pixels, Point};

    const BORDER: f32 = 1.;

    pub(crate) struct Chip {
        hwnd: Cell<isize>,
        drawn: RefCell<Option<ChipContent>>,
        visible: Cell<bool>,
    }

    impl Chip {
        pub(crate) fn new() -> Self {
            Self {
                hwnd: Cell::new(0),
                drawn: RefCell::new(None),
                visible: Cell::new(false),
            }
        }

        pub(crate) fn open(&self, _: &mut App) {
            self.handle();
        }

        pub(crate) fn prime(&self, content: ChipContent, _: &mut App) {
            let Some(hwnd) = self.handle() else {
                return;
            };
            if self
                .drawn
                .borrow()
                .as_ref()
                .is_some_and(|drawn| drawn.matches(&content))
            {
                return;
            }
            paint(hwnd, &content);
            *self.drawn.borrow_mut() = Some(content);
        }

        pub(crate) fn show_at(&self, origin: Point<Pixels>, _: &mut App) {
            let Some(hwnd) = self.handle() else {
                return;
            };
            let scale = self.drawn.borrow().as_ref().map_or(1., |drawn| drawn.scale);
            let x = (origin.x.as_f32() * scale).round() as i32;
            let y = (origin.y.as_f32() * scale).round() as i32;
            let mut flags = SWP_NOSIZE | SWP_NOACTIVATE;
            if !self.visible.replace(true) {
                flags |= SWP_SHOWWINDOW;
            }
            unsafe {
                let _ = SetWindowPos(hwnd, Some(HWND_TOPMOST), x, y, 0, 0, flags);
            }
        }

        pub(crate) fn hide(&self, _: &mut App) {
            if !self.visible.replace(false) {
                return;
            }
            let Some(hwnd) = self.handle() else {
                return;
            };
            unsafe {
                let _ = ShowWindow(hwnd, SW_HIDE);
            }
        }

        pub(crate) fn window_id(&self) -> Option<gpui::WindowId> {
            None
        }

        fn handle(&self) -> Option<HWND> {
            let existing = self.hwnd.get();
            if existing != 0 {
                return Some(HWND(existing as *mut c_void));
            }
            let hwnd = create()?;
            self.hwnd.set(hwnd.0 as isize);
            Some(hwnd)
        }
    }

    impl ChipContent {
        fn matches(&self, other: &Self) -> bool {
            self.title == other.title
                && self.size == other.size
                && (self.scale - other.scale).abs() < f32::EPSILON
        }
    }

    unsafe extern "system" fn procedure(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
    }

    fn create() -> Option<HWND> {
        unsafe {
            let instance = GetModuleHandleW(None).ok()?;
            let class = w!("StudioDragChip");
            let descriptor = WNDCLASSW {
                lpfnWndProc: Some(procedure),
                hInstance: instance.into(),
                lpszClassName: class,
                ..Default::default()
            };
            RegisterClassW(&descriptor);

            let hwnd = CreateWindowExW(
                WS_EX_LAYERED
                    | WS_EX_TRANSPARENT
                    | WS_EX_TOOLWINDOW
                    | WS_EX_NOACTIVATE
                    | WS_EX_TOPMOST,
                class,
                PCWSTR::null(),
                WS_POPUP,
                0,
                0,
                1,
                1,
                None,
                None,
                Some(instance.into()),
                None,
            )
            .ok()?;

            let disabled: BOOL = true.into();
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_TRANSITIONS_FORCEDISABLED,
                &disabled as *const _ as _,
                std::mem::size_of::<BOOL>() as u32,
            );
            Some(hwnd)
        }
    }

    fn paint(hwnd: HWND, content: &ChipContent) {
        let width = ((content.size.width.as_f32() * content.scale).round() as i32).max(1);
        let height = ((content.size.height.as_f32() * content.scale).round() as i32).max(1);

        unsafe {
            let screen = GetDC(None);
            let dc = CreateCompatibleDC(Some(screen));
            let info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut bits: *mut c_void = std::ptr::null_mut();
            let Ok(bitmap) = CreateDIBSection(Some(dc), &info, DIB_RGB_COLORS, &mut bits, None, 0)
            else {
                let _ = DeleteDC(dc);
                ReleaseDC(None, screen);
                return;
            };
            let previous = SelectObject(dc, bitmap.into());

            let bounds = RECT {
                left: 0,
                top: 0,
                right: width,
                bottom: height,
            };
            let fill = CreateSolidBrush(COLORREF(bgr(PANEL)));
            FillRect(dc, &bounds, fill);
            let _ = DeleteObject(fill.into());

            let face: Vec<u16> = crate::theme::FONT
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let font = CreateFontW(
                -((crate::theme::FONT_SIZE * content.scale).round() as i32),
                0,
                0,
                0,
                FW_NORMAL.0 as i32,
                0,
                0,
                0,
                DEFAULT_CHARSET,
                OUT_DEFAULT_PRECIS,
                CLIP_DEFAULT_PRECIS,
                CLEARTYPE_QUALITY,
                (DEFAULT_PITCH.0 | FF_DONTCARE.0) as u32,
                PCWSTR(face.as_ptr()),
            );
            let previous_font = SelectObject(dc, font.into());
            SetBkMode(dc, TRANSPARENT);
            SetTextColor(dc, COLORREF(bgr(over(TEXT, PANEL))));
            let mut text: Vec<u16> = content.title.encode_utf16().collect();
            let mut text_bounds = bounds;
            DrawTextW(
                dc,
                &mut text,
                &mut text_bounds,
                DT_CENTER | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX,
            );
            SelectObject(dc, previous_font);
            let _ = DeleteObject(font.into());

            shape(
                bits as *mut u32,
                width,
                height,
                RADIUS * content.scale,
                BORDER * content.scale,
            );

            let size = windows::Win32::Foundation::SIZE {
                cx: width,
                cy: height,
            };
            let source = windows::Win32::Foundation::POINT { x: 0, y: 0 };
            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };
            let _ = UpdateLayeredWindow(
                hwnd,
                Some(screen),
                None,
                Some(&size),
                Some(dc),
                Some(&source),
                COLORREF(0),
                Some(&blend),
                ULW_ALPHA,
            );

            SelectObject(dc, previous);
            let _ = DeleteObject(bitmap.into());
            let _ = DeleteDC(dc);
            ReleaseDC(None, screen);
        }
    }

    fn shape(bits: *mut u32, width: i32, height: i32, radius: f32, border: f32) {
        let (border_b, border_g, border_r) = channels(over(HAIRLINE, PANEL));
        for y in 0..height {
            for x in 0..width {
                let distance = rounded_rect(
                    x as f32 + 0.5,
                    y as f32 + 0.5,
                    width as f32,
                    height as f32,
                    radius,
                );
                let coverage = (0.5 - distance).clamp(0., 1.);
                let inside = (0.5 - (distance + border)).clamp(0., 1.);

                let pixel = unsafe { bits.add((y * width + x) as usize) };
                let packed = unsafe { *pixel };
                let b = (packed & 0xff) as f32;
                let g = ((packed >> 8) & 0xff) as f32;
                let r = ((packed >> 16) & 0xff) as f32;

                let b = border_b + (b - border_b) * inside;
                let g = border_g + (g - border_g) * inside;
                let r = border_r + (r - border_r) * inside;

                unsafe {
                    *pixel = ((coverage * 255.).round() as u32) << 24
                        | ((r * coverage).round() as u32) << 16
                        | ((g * coverage).round() as u32) << 8
                        | (b * coverage).round() as u32;
                }
            }
        }
    }

    fn rounded_rect(x: f32, y: f32, width: f32, height: f32, radius: f32) -> f32 {
        let half_width = width * 0.5;
        let half_height = height * 0.5;
        let radius = radius.min(half_width).min(half_height);
        let qx = (x - half_width).abs() - (half_width - radius);
        let qy = (y - half_height).abs() - (half_height - radius);
        let outside = (qx.max(0.).powi(2) + qy.max(0.).powi(2)).sqrt();
        outside + qx.max(qy).min(0.) - radius
    }

    fn over(color: u32, backdrop: u32) -> u32 {
        let alpha = (color & 0xff) as f32 / 255.;
        let (br, bg, bb) = (
            (backdrop >> 16) & 0xff,
            (backdrop >> 8) & 0xff,
            backdrop & 0xff,
        );
        let (cr, cg, cb) = (
            (color >> 24) & 0xff,
            (color >> 16) & 0xff,
            (color >> 8) & 0xff,
        );
        let mix =
            |c: u32, b: u32| ((c as f32 * alpha + b as f32 * (1. - alpha)).round() as u32) & 0xff;
        mix(cr, br) << 16 | mix(cg, bg) << 8 | mix(cb, bb)
    }

    fn channels(rgb: u32) -> (f32, f32, f32) {
        (
            (rgb & 0xff) as f32,
            ((rgb >> 8) & 0xff) as f32,
            ((rgb >> 16) & 0xff) as f32,
        )
    }

    fn bgr(rgb: u32) -> u32 {
        (rgb & 0xff) << 16 | (rgb & 0xff00) | (rgb >> 16) & 0xff
    }
}

#[cfg(target_os = "macos")]
mod overlay {
    use std::cell::{Cell, RefCell};

    use gpui::{
        AnyWindowHandle, App, AppContext, Bounds, Entity, Pixels, Point, Size,
        WindowBackgroundAppearance, WindowBounds, WindowId, WindowKind, WindowOptions, point, px,
        size,
    };

    use super::super::DragPreview;
    use super::ChipContent;

    pub(crate) struct Chip {
        window: RefCell<Option<AnyWindowHandle>>,
        view: RefCell<Option<Entity<DragPreview>>>,
        native: Cell<usize>,
        visible: Cell<bool>,
        size: Cell<Size<Pixels>>,
    }

    impl Chip {
        pub(crate) fn new() -> Self {
            Self {
                window: RefCell::new(None),
                view: RefCell::new(None),
                native: Cell::new(0),
                visible: Cell::new(false),
                size: Cell::new(size(px(96.), px(28.))),
            }
        }

        pub(crate) fn open(&self, _: &mut App) {}

        pub(crate) fn prime(&self, content: ChipContent, cx: &mut App) {
            self.build(cx);
            if content.size != self.size.get() {
                self.size.set(content.size);
                if let Some(window) = *self.window.borrow() {
                    let _ = window.update(cx, |_, window, _| window.resize(content.size));
                }
            }
            if let Some(view) = self.view.borrow().clone() {
                view.update(cx, |preview, cx| {
                    if preview.title != content.title {
                        preview.title = content.title;
                        cx.notify();
                    }
                });
            }
        }

        pub(crate) fn show_at(&self, origin: Point<Pixels>, cx: &mut App) {
            let (x, y) = (f32::from(origin.x), f32::from(origin.y));
            if self.visible.get() {
                crate::mac::move_window_ref(self.native.get(), x, y);
                return;
            }
            let Some(window) = *self.window.borrow() else {
                return;
            };
            self.visible.set(true);
            crate::mac::move_window_ref(self.native.get(), x, y);
            let _ = window.update(cx, |_, window, _| {
                crate::mac::order_front(window);
                window.refresh();
            });
        }

        pub(crate) fn hide(&self, cx: &mut App) {
            if !self.visible.replace(false) {
                return;
            }
            let Some(window) = *self.window.borrow() else {
                return;
            };
            let _ = window.update(cx, |_, window, _| crate::mac::order_out(window));
        }

        pub(crate) fn window_id(&self) -> Option<WindowId> {
            self.window.borrow().map(|window| window.window_id())
        }

        fn build(&self, cx: &mut App) {
            if self.window.borrow().is_some() {
                return;
            }
            let bounds = Bounds {
                origin: point(px(0.), px(0.)),
                size: self.size.get(),
            };
            let mut opened_view = None;
            let opened = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    focus: false,
                    show: false,
                    kind: WindowKind::Overlay,
                    is_movable: false,
                    window_background: WindowBackgroundAppearance::Transparent,
                    ..Default::default()
                },
                |_, cx| {
                    let preview = cx.new(|_| DragPreview {
                        title: Default::default(),
                    });
                    opened_view = Some(preview.clone());
                    preview
                },
            );
            if let (Ok(handle), Some(view)) = (opened, opened_view) {
                let handle: AnyWindowHandle = handle.into();
                if let Ok(native) = handle.update(cx, |_, window, _| crate::mac::window_ref(window))
                {
                    self.native.set(native);
                }
                *self.window.borrow_mut() = Some(handle);
                *self.view.borrow_mut() = Some(view);
            }
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod fallback {
    use std::cell::RefCell;

    use gpui::{
        App, AppContext, Bounds, Entity, Pixels, Point, WindowBackgroundAppearance, WindowBounds,
        WindowId, WindowKind, WindowOptions,
    };

    use super::super::DragPreview;
    use super::ChipContent;

    pub(crate) struct Chip {
        window: RefCell<Option<gpui::AnyWindowHandle>>,
        view: RefCell<Option<Entity<DragPreview>>>,
        content: RefCell<Option<ChipContent>>,
    }

    impl Chip {
        pub(crate) fn new() -> Self {
            Self {
                window: RefCell::new(None),
                view: RefCell::new(None),
                content: RefCell::new(None),
            }
        }

        pub(crate) fn open(&self, _: &mut App) {}

        pub(crate) fn prime(&self, content: ChipContent, cx: &mut App) {
            if let Some(view) = self.view.borrow().clone() {
                let title = content.title.clone();
                view.update(cx, |preview, cx| {
                    preview.title = title;
                    cx.notify();
                });
            }
            *self.content.borrow_mut() = Some(content);
        }

        pub(crate) fn show_at(&self, origin: Point<Pixels>, cx: &mut App) {
            if let Some(window) = *self.window.borrow() {
                let _ = window.update(cx, |_, window, _| window.set_origin(origin));
                return;
            }
            let Some(content) = self.content.borrow().clone() else {
                return;
            };
            let bounds = Bounds {
                origin,
                size: content.size,
            };
            let title = content.title;
            let mut opened_view = None;
            let opened = cx.open_window(
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
                |_, cx| {
                    let preview = cx.new(|_| DragPreview { title });
                    opened_view = Some(preview.clone());
                    preview
                },
            );
            if let (Ok(handle), Some(view)) = (opened, opened_view) {
                *self.window.borrow_mut() = Some(handle.into());
                *self.view.borrow_mut() = Some(view);
            }
        }

        pub(crate) fn hide(&self, cx: &mut App) {
            self.view.borrow_mut().take();
            if let Some(window) = self.window.borrow_mut().take() {
                let _ = window.update(cx, |_, window, _| window.remove_window());
            }
        }

        pub(crate) fn window_id(&self) -> Option<WindowId> {
            self.window.borrow().map(|window| window.window_id())
        }
    }
}
