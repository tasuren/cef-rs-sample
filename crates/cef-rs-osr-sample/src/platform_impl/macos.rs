use objc2::{runtime::*, *};

// ==== 右クリックしてもクラッシュしないようにする。 ====
// 参考: https://github.com/tauri-apps/cef-rs/issues/96

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
