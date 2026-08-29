use gpui::Window;
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

const NIL: *mut Object = std::ptr::null_mut();

pub(crate) fn native_window(window: &Window) -> Option<*mut Object> {
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

pub(crate) fn set_alpha(window: &Window, alpha: f64) {
    let Some(native) = native_window(window) else {
        return;
    };
    unsafe {
        let _: () = msg_send![native, setAlphaValue: alpha];
    }
}

pub(crate) fn order_front(window: &Window) {
    let Some(native) = native_window(window) else {
        return;
    };
    unsafe {
        let _: () = msg_send![native, orderFrontRegardless];
    }
}

pub(crate) fn order_out(window: &Window) {
    let Some(native) = native_window(window) else {
        return;
    };
    unsafe {
        let _: () = msg_send![native, orderOut: NIL];
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NativePoint {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NativeRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub(crate) fn cursor_position() -> Option<gpui::Point<gpui::Pixels>> {
    unsafe {
        let location: NativePoint = msg_send![class!(NSEvent), mouseLocation];
        let screens: *mut Object = msg_send![class!(NSScreen), screens];
        let primary: *mut Object = msg_send![screens, firstObject];
        if primary.is_null() {
            return None;
        }
        let screen: NativeRect = msg_send![primary, frame];
        Some(gpui::point(
            gpui::px(location.x as f32),
            gpui::px((screen.height - location.y) as f32),
        ))
    }
}

pub(crate) fn window_ref(window: &Window) -> usize {
    native_window(window).map_or(0, |native| native as usize)
}

pub(crate) fn move_window_ref(window: usize, x: f32, y: f32) {
    if window == 0 {
        return;
    }
    let native = window as *mut Object;
    unsafe {
        let screens: *mut Object = msg_send![class!(NSScreen), screens];
        let primary: *mut Object = msg_send![screens, firstObject];
        if primary.is_null() {
            return;
        }
        let screen: NativeRect = msg_send![primary, frame];
        let frame: NativeRect = msg_send![native, frame];
        let origin = NativePoint {
            x: x as f64,
            y: screen.height - y as f64 - frame.height,
        };
        let _: () = msg_send![native, setFrameOrigin: origin];
    }
}

const TITLE_KEY: u8 = 0;
const TOOLBAR_ID: &std::ffi::CStr = c"studio.toolbar";
const TITLE_ITEM_ID: &std::ffi::CStr = c"studio.toolbar.title";
const DOCK_SYMBOLS: [&std::ffi::CStr; 3] = [
    c"sidebar.left",
    c"rectangle.bottomthird.inset.filled",
    c"sidebar.right",
];
const DOCK_BUTTON_SIZE: f64 = 22.;
const DOCK_BUTTON_GAP: f64 = 2.;
const MIN_MAX_Y_MARGIN: usize = 8 | 32;
const MIN_X_MARGIN: usize = 1;
const DOCK_ROW_MARGIN: f64 = 12.;
const DOCK_ROW_ID: &std::ffi::CStr = c"studio.titlebar.docks";
const UNIFIED_COMPACT: usize = 4;
const SEPARATOR_NONE: usize = 1;
const TITLE_HIDDEN: usize = 1;
const RETAIN_NONATOMIC: usize = 1;

unsafe extern "C" {
    fn objc_getAssociatedObject(object: *mut Object, key: *const std::ffi::c_void) -> *mut Object;
    fn objc_setAssociatedObject(
        object: *mut Object,
        key: *const std::ffi::c_void,
        value: *mut Object,
        policy: usize,
    );
}

fn title_key() -> *const std::ffi::c_void {
    &TITLE_KEY as *const u8 as *const std::ffi::c_void
}

unsafe fn ns_string(text: &std::ffi::CStr) -> *mut Object {
    unsafe { msg_send![class!(NSString), stringWithUTF8String: text.as_ptr()] }
}

unsafe fn symbol_image(name: &std::ffi::CStr) -> *mut Object {
    unsafe {
        let symbols: bool = msg_send![
            class!(NSImage),
            respondsToSelector: sel!(imageWithSystemSymbolName:accessibilityDescription:)
        ];
        if !symbols {
            return NIL;
        }
        msg_send![
            class!(NSImage),
            imageWithSystemSymbolName: ns_string(name)
            accessibilityDescription: NIL
        ]
    }
}

#[derive(Copy, Clone)]
pub(crate) enum TitlebarButton {
    LeftDock,
    RightDock,
    BottomDock,
}

thread_local! {
    static TOOLBAR_ACTION: std::cell::RefCell<Option<std::rc::Rc<dyn Fn(TitlebarButton)>>> =
        const { std::cell::RefCell::new(None) };
}

pub(crate) fn set_toolbar_action(action: impl Fn(TitlebarButton) + 'static) {
    TOOLBAR_ACTION.with(|slot| *slot.borrow_mut() = Some(std::rc::Rc::new(action)));
}

fn run_toolbar_action(button: TitlebarButton) {
    let action = TOOLBAR_ACTION.with(|slot| slot.borrow().clone());
    if let Some(action) = action {
        action(button);
    }
}

extern "C" fn toolbar_item(
    _this: &Object,
    _sel: objc::runtime::Sel,
    toolbar: *mut Object,
    identifier: *mut Object,
    _inserted: bool,
) -> *mut Object {
    unsafe {
        let item: *mut Object = msg_send![class!(NSToolbarItem), alloc];
        let item: *mut Object = msg_send![item, initWithItemIdentifier: identifier];
        let title: *mut Object = objc_getAssociatedObject(toolbar, title_key());
        let title = if title.is_null() {
            ns_string(c"")
        } else {
            title
        };
        let label: *mut Object = msg_send![class!(NSTextField), labelWithString: title];
        let font: *mut Object = msg_send![class!(NSFont), systemFontOfSize: 13.0f64];
        let color: *mut Object = msg_send![class!(NSColor), whiteColor];
        let _: () = msg_send![label, setFont: font];
        let _: () = msg_send![label, setTextColor: color];
        let _: () = msg_send![item, setView: label];
        msg_send![item, autorelease]
    }
}

unsafe fn dock_buttons() -> *mut Object {
    unsafe {
        let count = DOCK_SYMBOLS.len() as f64;
        let bounds = NativeRect {
            x: 0.,
            y: 0.,
            width: count * DOCK_BUTTON_SIZE + (count - 1.) * DOCK_BUTTON_GAP,
            height: DOCK_BUTTON_SIZE,
        };
        let container: *mut Object = msg_send![class!(NSView), alloc];
        let container: *mut Object = msg_send![container, initWithFrame: bounds];
        for (index, symbol) in DOCK_SYMBOLS.iter().enumerate() {
            let image = symbol_image(symbol);
            let target = toolbar_delegate();
            let button: *mut Object = if image.is_null() {
                msg_send![
                    class!(NSButton),
                    buttonWithTitle: ns_string(c"\u{25a1}")
                    target: target
                    action: sel!(studioToggleDock:)
                ]
            } else {
                msg_send![
                    class!(NSButton),
                    buttonWithImage: image
                    target: target
                    action: sel!(studioToggleDock:)
                ]
            };
            let _: () = msg_send![button, setBordered: false];
            let _: () = msg_send![button, setTag: index as isize];
            let _: () = msg_send![button, setFrame: NativeRect {
                x: index as f64 * (DOCK_BUTTON_SIZE + DOCK_BUTTON_GAP),
                y: 0.,
                width: DOCK_BUTTON_SIZE,
                height: DOCK_BUTTON_SIZE,
            }];
            let _: () = msg_send![button, setAutoresizingMask: MIN_MAX_Y_MARGIN];
            let _: () = msg_send![container, addSubview: button];
        }
        msg_send![container, autorelease]
    }
}

extern "C" fn toolbar_identifiers(
    _this: &Object,
    _sel: objc::runtime::Sel,
    _toolbar: *mut Object,
) -> *mut Object {
    unsafe {
        let identifiers = [ns_string(TITLE_ITEM_ID)];
        msg_send![
            class!(NSArray),
            arrayWithObjects: identifiers.as_ptr()
            count: identifiers.len()
        ]
    }
}

extern "C" fn studio_toggle_dock(_this: &Object, _sel: objc::runtime::Sel, sender: *mut Object) {
    unsafe {
        let tag: isize = msg_send![sender, tag];
        let button = match tag {
            0 => TitlebarButton::LeftDock,
            1 => TitlebarButton::BottomDock,
            2 => TitlebarButton::RightDock,
            _ => return,
        };
        run_toolbar_action(button);
    }
}

fn toolbar_delegate() -> *mut Object {
    static DELEGATE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *DELEGATE.get_or_init(|| unsafe {
        let class = match objc::declare::ClassDecl::new("StudioToolbarDelegate", class!(NSObject)) {
            Some(mut decl) => {
                let item: extern "C" fn(
                    &Object,
                    objc::runtime::Sel,
                    *mut Object,
                    *mut Object,
                    bool,
                ) -> *mut Object = toolbar_item;
                let identifiers: extern "C" fn(
                    &Object,
                    objc::runtime::Sel,
                    *mut Object,
                ) -> *mut Object = toolbar_identifiers;
                decl.add_method(
                    sel!(toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:),
                    item,
                );
                let toggle: extern "C" fn(&Object, objc::runtime::Sel, *mut Object) =
                    studio_toggle_dock;
                decl.add_method(sel!(toolbarAllowedItemIdentifiers:), identifiers);
                decl.add_method(sel!(toolbarDefaultItemIdentifiers:), identifiers);
                decl.add_method(sel!(studioToggleDock:), toggle);
                decl.register()
            }
            None => match objc::runtime::Class::get("StudioToolbarDelegate") {
                Some(class) => class,
                None => return 0,
            },
        };
        let instance: *mut Object = msg_send![class, new];
        instance as usize
    }) as *mut Object
}

pub(crate) fn force_dark_appearance() {
    unsafe {
        let name = ns_string(c"NSAppearanceNameDarkAqua");
        let appearance: *mut Object = msg_send![class!(NSAppearance), appearanceNamed: name];
        let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
        let _: () = msg_send![app, setAppearance: appearance];
    }
}

pub(crate) fn attach_dock_buttons(window: &Window) -> bool {
    let Some(native) = native_window(window) else {
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
        let mut row = dock_row(titlebar);
        if row.is_null() {
            row = dock_buttons();
            let _: () = msg_send![row, setAutoresizingMask: MIN_X_MARGIN | MIN_MAX_Y_MARGIN];
            let _: () = msg_send![row, setIdentifier: ns_string(DOCK_ROW_ID)];
            let _: () = msg_send![titlebar, addSubview: row];
        }
        let bounds: NativeRect = msg_send![titlebar, bounds];
        if bounds.height <= 0. {
            return false;
        }
        let frame: NativeRect = msg_send![row, frame];
        let placed = NativeRect {
            x: (bounds.width - frame.width - DOCK_ROW_MARGIN).round(),
            y: ((bounds.height - frame.height) / 2.).round(),
            width: frame.width,
            height: frame.height,
        };
        if placed.x != frame.x || placed.y != frame.y {
            let _: () = msg_send![row, setFrame: placed];
        }
        true
    }
}

unsafe fn dock_row(titlebar: *mut Object) -> *mut Object {
    unsafe {
        let wanted = ns_string(DOCK_ROW_ID);
        let subviews: *mut Object = msg_send![titlebar, subviews];
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

pub(crate) fn attach_toolbar(window: &Window, title: &str) {
    let Some(native) = native_window(window) else {
        return;
    };
    let delegate = toolbar_delegate();
    if delegate.is_null() {
        return;
    }
    let title = std::ffi::CString::new(title).unwrap_or_default();
    unsafe {
        let toolbar: *mut Object = msg_send![class!(NSToolbar), alloc];
        let toolbar: *mut Object = msg_send![toolbar, initWithIdentifier: ns_string(TOOLBAR_ID)];
        objc_setAssociatedObject(toolbar, title_key(), ns_string(&title), RETAIN_NONATOMIC);
        let _: () = msg_send![toolbar, setDelegate: delegate];
        let centered: *mut Object =
            msg_send![class!(NSSet), setWithObject: ns_string(TITLE_ITEM_ID)];
        let centers: bool =
            msg_send![toolbar, respondsToSelector: sel!(setCenteredItemIdentifiers:)];
        if centers {
            let _: () = msg_send![toolbar, setCenteredItemIdentifiers: centered];
        }
        let _: () = msg_send![native, setToolbar: toolbar];
        let _: () = msg_send![native, setToolbarStyle: UNIFIED_COMPACT];
        let _: () = msg_send![toolbar, release];
    }
    apply_titlebar_style(window);
}

pub(crate) fn apply_titlebar_style(window: &Window) -> bool {
    let Some(native) = native_window(window) else {
        return false;
    };
    unsafe {
        let black: *mut Object = msg_send![class!(NSColor), blackColor];
        let _: () = msg_send![native, setTitlebarSeparatorStyle: SEPARATOR_NONE];
        let _: () = msg_send![native, setTitleVisibility: TITLE_HIDDEN];
        let _: () = msg_send![native, setTitlebarAppearsTransparent: true];
        let _: () = msg_send![native, setBackgroundColor: black];
    }
    true
}
