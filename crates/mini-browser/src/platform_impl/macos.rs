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

/// 右クリックしてもクラッシュしないようにする。
/// 参考: https://github.com/tauri-apps/cef-rs/issues/96
pub mod handling_send_event {
    use objc2::{runtime::*, *};

    extern "C" fn set_handling_send_event(
        _this: *mut AnyObject,
        _cmd: Sel,
        _handling_send_event: Bool,
    ) {
    }

    extern "C" fn is_handling_send_event(_this: *mut AnyObject, _cmd: Sel) -> Bool {
        Bool::YES
    }

    pub unsafe fn extend_nswindow_class() {
        let ns_window = class!(NSApplication);

        let encoding_get = c"B@:";
        let encoding_set = c"v@:B";

        let _ = unsafe {
            objc2::ffi::class_addMethod(
                ns_window as *const _ as *mut _,
                sel!(isHandlingSendEvent),
                std::mem::transmute::<*const (), unsafe extern "C-unwind" fn()>(
                    is_handling_send_event as *const (),
                ),
                encoding_get.as_ptr(),
            )
        };

        let _ = unsafe {
            objc2::ffi::class_addMethod(
                ns_window as *const _ as *mut _,
                sel!(setHandlingSendEvent:),
                std::mem::transmute::<*const (), unsafe extern "C-unwind" fn()>(
                    set_handling_send_event as *const (),
                ),
                encoding_set.as_ptr(),
            )
        };
    }
}
