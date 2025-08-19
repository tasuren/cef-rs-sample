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

/// This modules include bindings of `include/cef_application_mac.h`.
pub(crate) mod cef_application_mac {
    use objc2::{extern_protocol, runtime::Bool};

    extern_protocol!(
        #[allow(clippy::missing_safety_doc)]
        pub unsafe trait CrAppProtocol {
            #[unsafe(method(isHandlingSendEvent))]
            unsafe fn is_handling_send_event(&self) -> Bool;
        }
    );

    extern_protocol!(
        #[allow(clippy::missing_safety_doc)]
        pub unsafe trait CrAppControlProtocol: CrAppProtocol {
            #[unsafe(method(setHandlingSendEvent:))]
            unsafe fn set_handling_send_event(&self, handling_send_event: Bool);
        }
    );

    extern_protocol!(
        #[allow(clippy::missing_safety_doc)]
        pub unsafe trait CefAppProtocol: CrAppControlProtocol {}
    );
}

pub mod ns_application {
    use std::cell::RefCell;

    pub use super::cef_application_mac::*;
    use objc2::{ClassType, DefinedClass, define_class, rc::Retained, runtime::Bool};
    use objc2_app_kit::NSApplication;

    pub struct Ivars {
        handling_send_event: RefCell<Bool>,
    }

    define_class!(
        #[unsafe(super(NSApplication))]
        #[ivars = Ivars]
        pub struct SimpleApplication;

        unsafe impl CrAppProtocol for SimpleApplication {
            #[unsafe(method(isHandlingSendEvent))]
            unsafe fn is_handling_send_event(&self) -> Bool {
                *self.ivars().handling_send_event.borrow_mut()
            }
        }

        unsafe impl CrAppControlProtocol for SimpleApplication {
            #[unsafe(method(setHandlingSendEvent:))]
            unsafe fn set_handling_send_event(&self, handling_send_event: Bool) {
                *self.ivars().handling_send_event.borrow_mut() = handling_send_event;
            }
        }

        unsafe impl CefAppProtocol for SimpleApplication {}
    );

    pub fn initialize_simple_application() -> Retained<SimpleApplication> {
        unsafe { objc2::msg_send![SimpleApplication::class(), sharedApplication] }
    }
}
