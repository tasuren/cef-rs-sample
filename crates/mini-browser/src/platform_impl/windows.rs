/// CEFに親として使って欲しいウィンドウをwinitウィンドウから取り出す。
pub mod raw_view {
    use cef::sys::HWND;
    use winit::window::Window;
    use wry::raw_window_handle::{HasWindowHandle, RawWindowHandle};

    pub fn get_hwnd(window: &Window) -> HWND {
        let window_handle = window.window_handle().unwrap();

        if let RawWindowHandle::Win32(raw_handle) = window_handle.as_raw() {
            HWND(raw_handle.hwnd.get() as _)
        } else {
            unreachable!();
        }
    }
}
