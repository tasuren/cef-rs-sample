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
    pub struct DemoApp;

    unsafe impl CrAppProtocol for DemoApp {
        #[unsafe(method(isHandlingSendEvent))]
        unsafe fn is_handling_send_event(&self) -> Bool {
            *self.ivars().handling_send_event.borrow_mut()
        }
    }

    unsafe impl CrAppControlProtocol for DemoApp {
        #[unsafe(method(setHandlingSendEvent:))]
        unsafe fn set_handling_send_event(&self, handling_send_event: Bool) {
            *self.ivars().handling_send_event.borrow_mut() = handling_send_event;
        }
    }

    unsafe impl CefAppProtocol for DemoApp {}
);

pub fn initialize_ns_app() {
    let mtm = MainThreadMarker::new().unwrap();

    unsafe {
        let _: Retained<AnyObject> = objc2::msg_send![DemoApp::class(), sharedApplication];
    }

    assert!(NSApp(mtm).isKindOfClass(DemoApp::class()));
}
