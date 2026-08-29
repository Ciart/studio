use gpui::Window;

#[derive(Default)]
pub struct FullscreenTitlebar {
    fullscreen: bool,
    backdrop_cleared: bool,
}

impl FullscreenTitlebar {
    pub fn sync(&mut self, window: &Window) {
        install_presentation_options(window);

        let fullscreen = window.is_fullscreen();
        if fullscreen != self.fullscreen {
            self.fullscreen = fullscreen;
            self.backdrop_cleared = false;
            if fullscreen {
                install_reveal_fade();
                enter_fullscreen(window);
            } else {
                exit_fullscreen(window);
            }
        }
        if fullscreen && !self.backdrop_cleared {
            self.backdrop_cleared = set_backdrop_alpha(window, 0.);
            if !self.backdrop_cleared {
                window.request_animation_frame();
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod mac {
    use gpui::Window;
    use objc::{
        class, msg_send,
        runtime::{Class, Object, Sel, class_addMethod, object_getClass},
        sel, sel_impl,
    };

    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use std::sync::Once;

    const NIL: *mut Object = std::ptr::null_mut();
    const AUTO_HIDE_TOOLBAR: usize = 1 << 11;

    fn native_window(window: &Window) -> Option<*mut Object> {
        let handle = HasWindowHandle::window_handle(window).ok()?;
        let RawWindowHandle::AppKit(handle) = handle.as_raw() else {
            return None;
        };
        unsafe {
            let view = handle.ns_view.as_ptr() as *mut Object;
            let native: *mut Object = msg_send![view, window];
            (!native.is_null()).then_some(native)
        }
    }

    extern "C" fn will_use_fullscreen_presentation_options(
        _this: &Object,
        _sel: Sel,
        _window: *mut Object,
        proposed: usize,
    ) -> usize {
        proposed | AUTO_HIDE_TOOLBAR
    }

    pub(super) fn install_presentation_options(window: &Window) {
        static INSTALL: Once = Once::new();
        let Some(native) = native_window(window) else {
            return;
        };
        INSTALL.call_once(|| unsafe {
            let class = object_getClass(native as *const Object);
            let imp: extern "C" fn(&Object, Sel, *mut Object, usize) -> usize =
                will_use_fullscreen_presentation_options;
            class_addMethod(
                class as *mut Class,
                sel!(window:willUseFullScreenPresentationOptions:),
                std::mem::transmute(imp),
                c"Q@:@Q".as_ptr(),
            );
        });
    }

    pub(super) fn enter_fullscreen(window: &Window) {
        let Some(native) = native_window(window) else {
            return;
        };
        unsafe {
            let name: *mut Object = msg_send![class!(NSString), stringWithUTF8String: c"studio-fullscreen-lights".as_ptr()];
            let toolbar: *mut Object = msg_send![class!(NSToolbar), alloc];
            let toolbar: *mut Object = msg_send![toolbar, initWithIdentifier: name];
            let _: () = msg_send![native, setToolbar: toolbar];
            let _: () = msg_send![native, setToolbarStyle: 4usize];
            let _: () = msg_send![native, setTitlebarSeparatorStyle: 1usize];
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    static ORIG_SET_FRAME: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static HIDDEN_Y_KEY: u8 = 0;

    unsafe fn titlebar_container(parent: *mut Object) -> *mut Object {
        unsafe {
            let close: *mut Object = msg_send![parent, standardWindowButton: 0usize];
            if close.is_null() {
                return std::ptr::null_mut();
            }
            let titlebar: *mut Object = msg_send![close, superview];
            if titlebar.is_null() {
                return std::ptr::null_mut();
            }
            msg_send![titlebar, superview]
        }
    }

    extern "C" fn reveal_set_frame(this: &Object, sel: Sel, frame: Rect, display: bool) {
        unsafe {
            let orig: extern "C" fn(&Object, Sel, Rect, bool) =
                std::mem::transmute(ORIG_SET_FRAME.load(std::sync::atomic::Ordering::Relaxed));
            let this_ptr = this as *const Object as *mut Object;

            let parent: *mut Object = msg_send![this_ptr, parentWindow];
            if parent.is_null() {
                return orig(this, sel, frame, display);
            }
            let mask: usize = msg_send![parent, styleMask];
            if mask & (1 << 14) == 0 {
                return orig(this, sel, frame, display);
            }

            let parent_frame: Rect = msg_send![parent, frame];
            let shown_y = parent_frame.y + parent_frame.height - frame.height;

            let key = &HIDDEN_Y_KEY as *const u8 as *const std::ffi::c_void;
            let stored: *mut Object = objc_getAssociatedObject(this_ptr, key);
            let mut hidden_y: f64 = if stored.is_null() {
                shown_y + frame.height
            } else {
                msg_send![stored, doubleValue]
            };
            if frame.y > hidden_y {
                hidden_y = frame.y;
                let number: *mut Object = msg_send![class!(NSNumber), numberWithDouble: hidden_y];
                objc_setAssociatedObject(this_ptr, key, number, 1);
            }

            let travel = hidden_y - shown_y;
            if travel <= 1. {
                return orig(this, sel, frame, display);
            }
            let progress = ((hidden_y - frame.y) / travel).clamp(0., 1.);

            let container = titlebar_container(parent);
            if !container.is_null() {
                let _: () = msg_send![class!(CATransaction), begin];
                let _: () = msg_send![class!(CATransaction), setDisableActions: true];
                let origin: Rect = msg_send![container, frame];
                if origin.y != 0. {
                    let _: () = msg_send![container, setFrameOrigin: PointXY {
                        x: origin.x,
                        y: 0.,
                    }];
                }
                let _: () = msg_send![container, setAlphaValue: progress];
                let _: () = msg_send![class!(CATransaction), commit];
            }

            if frame.y <= shown_y + 0.5 || frame.y >= hidden_y - 0.5 {
                return orig(this, sel, frame, display);
            }
            let pinned = Rect {
                x: frame.x,
                y: shown_y,
                width: frame.width,
                height: frame.height,
            };
            orig(this, sel, pinned, display)
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub(super) struct PointXY {
        pub x: f64,
        pub y: f64,
    }

    unsafe extern "C" {
        fn objc_getAssociatedObject(
            object: *mut Object,
            key: *const std::ffi::c_void,
        ) -> *mut Object;
        fn objc_setAssociatedObject(
            object: *mut Object,
            key: *const std::ffi::c_void,
            value: *mut Object,
            policy: usize,
        );
        fn method_setImplementation(
            method: *mut std::ffi::c_void,
            imp: *const std::ffi::c_void,
        ) -> *const std::ffi::c_void;
        fn class_getInstanceMethod(class: *const Class, sel: Sel) -> *mut std::ffi::c_void;
    }

    pub(super) fn install_reveal_fade() {
        use std::sync::Once;
        static INSTALL: Once = Once::new();
        INSTALL.call_once(|| unsafe {
            let Some(class) = Class::get("NSToolbarFullScreenWindow") else {
                return;
            };
            let method = class_getInstanceMethod(class as *const Class, sel!(setFrame:display:));
            if method.is_null() {
                return;
            }
            let imp: extern "C" fn(&Object, Sel, Rect, bool) = reveal_set_frame;
            let orig = method_setImplementation(method, imp as *const std::ffi::c_void);
            ORIG_SET_FRAME.store(orig as usize, std::sync::atomic::Ordering::Relaxed);
        });
    }

    pub(super) fn exit_fullscreen(window: &Window) {
        let Some(native) = native_window(window) else {
            return;
        };
        unsafe {
            let _: () = msg_send![native, setToolbar: NIL];
        }
        set_backdrop_alpha(window, 1.);
    }

    pub(super) fn set_backdrop_alpha(window: &Window, alpha: f64) -> bool {
        let Some(native) = native_window(window) else {
            return false;
        };
        let Some(backdrop) = Class::get("NSTitlebarBackgroundView") else {
            return false;
        };
        unsafe {
            let close: *mut Object = msg_send![native, standardWindowButton: 0usize];
            if close.is_null() {
                return false;
            }
            let titlebar: *mut Object = msg_send![close, superview];
            if titlebar.is_null() {
                return false;
            }
            let container: *mut Object = msg_send![titlebar, superview];
            if container.is_null() {
                return false;
            }

            let mut found = false;
            let subviews: *mut Object = msg_send![container, subviews];
            let count: usize = msg_send![subviews, count];
            for index in 0..count {
                let subview: *mut Object = msg_send![subviews, objectAtIndex: index];
                let matches: bool = msg_send![subview, isKindOfClass: backdrop];
                if !matches {
                    continue;
                }
                found = true;
                let current: f64 = msg_send![subview, alphaValue];
                if current != alpha {
                    let _: () = msg_send![subview, setAlphaValue: alpha];
                }
            }
            found
        }
    }
}

#[cfg(target_os = "macos")]
use mac::{
    enter_fullscreen, exit_fullscreen, install_presentation_options, install_reveal_fade,
    set_backdrop_alpha,
};

#[cfg(not(target_os = "macos"))]
mod stub {
    use gpui::Window;
    pub(super) fn install_presentation_options(_: &Window) {}
    pub(super) fn enter_fullscreen(_: &Window) {}
    pub(super) fn exit_fullscreen(_: &Window) {}
    pub(super) fn set_backdrop_alpha(_: &Window, _: f64) -> bool {
        true
    }
    pub(super) fn install_reveal_fade() {}
}

#[cfg(not(target_os = "macos"))]
use stub::{
    enter_fullscreen, exit_fullscreen, install_presentation_options, install_reveal_fade,
    set_backdrop_alpha,
};
