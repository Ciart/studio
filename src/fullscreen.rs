use gpui::Window;

#[derive(Default)]
pub struct FullscreenTitlebar {
    fullscreen: bool,
    styled: bool,
    restored: bool,
}

impl FullscreenTitlebar {
    pub fn sync(&mut self, window: &Window) {
        let fullscreen = window.is_fullscreen();
        if fullscreen != self.fullscreen {
            self.fullscreen = fullscreen;
            self.styled = false;
            self.restored = false;
        }
        if fullscreen {
            if !self.styled {
                self.styled = style_titlebar(window);
                if !self.styled {
                    window.request_animation_frame();
                }
            }
        } else if !self.restored {
            self.restored = restore_titlebar(window);
            if !self.restored {
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
        runtime::{Class, Object},
        sel, sel_impl,
    };

    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    const NIL: *mut Object = std::ptr::null_mut();
    const BACKING_ID: &std::ffi::CStr = c"studio.titlebar.backing";
    const WIDTH_AND_HEIGHT_SIZABLE: usize = 2 | 16;
    const ORDER_BELOW: isize = -1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

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

    unsafe fn titlebar_container(native: *mut Object) -> *mut Object {
        unsafe {
            let close: *mut Object = msg_send![native, standardWindowButton: 0usize];
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

    pub(super) fn restore_titlebar(window: &Window) -> bool {
        let Some(native) = native_window(window) else {
            return false;
        };
        unsafe {
            let container = titlebar_container(native);
            if !container.is_null() {
                let backing = backing_view(container);
                if !backing.is_null() {
                    let _: () = msg_send![backing, removeFromSuperview];
                }
            }
        }
        set_backdrop_alpha(window, 1.);
        crate::mac::apply_titlebar_style(window)
    }

    pub(super) fn style_titlebar(window: &Window) -> bool {
        if !set_backdrop_alpha(window, 0.) {
            return false;
        }
        let Some(native) = native_window(window) else {
            return false;
        };
        unsafe {
            let container = titlebar_container(native);
            if container.is_null() {
                return false;
            }
            if backing_view(container).is_null() {
                install_backing(container);
            }
        }
        true
    }

    unsafe fn backing_view(container: *mut Object) -> *mut Object {
        unsafe {
            let wanted: *mut Object =
                msg_send![class!(NSString), stringWithUTF8String: BACKING_ID.as_ptr()];
            let subviews: *mut Object = msg_send![container, subviews];
            let count: usize = msg_send![subviews, count];
            for index in 0..count {
                let subview: *mut Object = msg_send![subviews, objectAtIndex: index];
                let identifier: *mut Object = msg_send![subview, identifier];
                if identifier.is_null() {
                    continue;
                }
                let matches: bool = msg_send![identifier, isEqualToString: wanted];
                if matches {
                    return subview;
                }
            }
            std::ptr::null_mut()
        }
    }

    unsafe fn install_backing(container: *mut Object) {
        unsafe {
            let bounds: Rect = msg_send![container, bounds];
            let view: *mut Object = msg_send![class!(NSView), alloc];
            let view: *mut Object = msg_send![view, initWithFrame: bounds];
            let identifier: *mut Object =
                msg_send![class!(NSString), stringWithUTF8String: BACKING_ID.as_ptr()];
            let _: () = msg_send![view, setIdentifier: identifier];
            let _: () = msg_send![view, setWantsLayer: true];
            let color: *mut Object = msg_send![class!(NSColor), blackColor];
            let cgcolor: *mut Object = msg_send![color, CGColor];
            let layer: *mut Object = msg_send![view, layer];
            let _: () = msg_send![layer, setBackgroundColor: cgcolor];
            let _: () = msg_send![view, setAutoresizingMask: WIDTH_AND_HEIGHT_SIZABLE];
            let _: () =
                msg_send![container, addSubview: view positioned: ORDER_BELOW relativeTo: NIL];
            let _: () = msg_send![view, release];
        }
    }

    fn set_backdrop_alpha(window: &Window, alpha: f64) -> bool {
        let Some(native) = native_window(window) else {
            return false;
        };
        let Some(backdrop) = Class::get("NSTitlebarBackgroundView") else {
            return false;
        };
        unsafe {
            let container = titlebar_container(native);
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
use mac::{restore_titlebar, style_titlebar};

#[cfg(not(target_os = "macos"))]
mod stub {
    use gpui::Window;
    pub(super) fn restore_titlebar(_: &Window) -> bool {
        true
    }
    pub(super) fn style_titlebar(_: &Window) -> bool {
        true
    }
}

#[cfg(not(target_os = "macos"))]
use stub::{restore_titlebar, style_titlebar};
