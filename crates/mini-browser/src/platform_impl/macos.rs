/// CEFに親として使って欲しいViewをwinitウィンドウから取り出す。
pub mod raw_view {
    use std::ffi::c_void;

    use winit::window::Window;
    use wry::raw_window_handle::{HasWindowHandle, RawWindowHandle};

    pub fn get_view(window: &Window) -> *mut c_void {
        let window_handle = window.window_handle().unwrap();

        if let RawWindowHandle::AppKit(handle) = window_handle.as_raw() {
            unsafe { handle.ns_view.cast().as_mut() }
        } else {
            unreachable!();
        }
    }
}

pub mod ns_application {
    use std::cell::RefCell;

    use cef::application_mac::{CefAppProtocol, CrAppControlProtocol, CrAppProtocol};
    use objc2::{rc::*, runtime::*, *};
    use objc2_app_kit::{NSApp, NSApplication};

    pub struct Ivars {
        handling_send_event: RefCell<Bool>,
    }

    define_class!(
        #[unsafe(super(NSApplication))]
        #[ivars = Ivars]
        pub struct MiniBrowserApp;

        unsafe impl CrAppProtocol for MiniBrowserApp {
            #[unsafe(method(isHandlingSendEvent))]
            unsafe fn is_handling_send_event(&self) -> Bool {
                *self.ivars().handling_send_event.borrow_mut()
            }
        }

        unsafe impl CrAppControlProtocol for MiniBrowserApp {
            #[unsafe(method(setHandlingSendEvent:))]
            unsafe fn set_handling_send_event(&self, handling_send_event: Bool) {
                *self.ivars().handling_send_event.borrow_mut() = handling_send_event;
            }
        }

        unsafe impl CefAppProtocol for MiniBrowserApp {}
    );

    pub fn initialize_ns_app() {
        let mtm = MainThreadMarker::new().unwrap();

        unsafe {
            let _: Retained<AnyObject> =
                objc2::msg_send![MiniBrowserApp::class(), sharedApplication];
        }

        assert!(NSApp(mtm).isKindOfClass(MiniBrowserApp::class()));
    }
}
