use cef::{
    Browser, ImplRenderHandler, Rect, RenderHandler, ScreenInfo, WrapRenderHandler,
    rc::{Rc as _, RcImpl},
    sys::{_cef_base_ref_counted_t, _cef_render_handler_t},
};

use crate::app::SharedWindowState;

/// softbufferを使ってOff-Screen Renderingするよう`RenderHandler`を実装した構造体。
pub struct SampleRenderHandler {
    object: *mut RcImpl<cef::sys::_cef_render_handler_t, Self>,
    window_state: SharedWindowState,
}

impl SampleRenderHandler {
    pub fn new_render_handler(window_state: SharedWindowState) -> RenderHandler {
        RenderHandler::new(Self {
            object: std::ptr::null_mut(),
            window_state,
        })
    }
}

impl Clone for SampleRenderHandler {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            object,
            window_state: self.window_state.clone(),
        }
    }
}

impl cef::rc::Rc for SampleRenderHandler {
    fn as_base(&self) -> &_cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapRenderHandler for SampleRenderHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<_cef_render_handler_t, Self>) {
        self.object = object;
    }
}

impl ImplRenderHandler for SampleRenderHandler {
    fn get_raw(&self) -> *mut _cef_render_handler_t {
        self.object.cast()
    }

    fn view_rect(&self, _browser: Option<&mut Browser>, rect: Option<&mut Rect>) {
        if let Some(rect) = rect {
            let window_state = self.window_state.borrow();
            let size = window_state.get_size();

            if size.width > 0 && size.height > 0 {
                let logical_size = size.to_logical::<i32>(window_state.scale_factor());

                rect.width = logical_size.width as _;
                rect.height = logical_size.height as _;
            }
        }
    }

    fn screen_info(
        &self,
        _browser: Option<&mut Browser>,
        screen_info: Option<&mut ScreenInfo>,
    ) -> ::std::os::raw::c_int {
        // これを実装しない場合、恐らく物理ピクセルで計算される？
        // でも念のため、拡大率を教えておく。

        if let Some(screen_info) = screen_info {
            screen_info.device_scale_factor = self.window_state.borrow().scale_factor() as _;
            return true as _;
        }

        false as _
    }

    fn on_paint(
        &self,
        _browser: Option<&mut Browser>,
        _type: cef::PaintElementType,
        _dirty_rects_count: usize,
        _dirty_rects: Option<&Rect>,
        buffer: *const u8,
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
    ) {
        unsafe {
            self.window_state
                .borrow_mut()
                .paint_raw(buffer, width, height)
        };
    }
}
