use std::cell::RefCell;

use cef::{rc::*, *};
use winit::dpi::LogicalSize;

pub type ViewSize = std::rc::Rc<RefCell<LogicalSize<u32>>>;

pub struct SampleRenderHandler {
    object: *mut RcImpl<cef::sys::_cef_render_handler_t, Self>,
    size: ViewSize,
    scale_factor: f32,
}

impl SampleRenderHandler {
    pub fn new_render_handler(size: ViewSize, scale_factor: f32) -> RenderHandler {
        RenderHandler::new(Self {
            object: std::ptr::null_mut(),
            size,
            scale_factor,
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
            size: std::rc::Rc::clone(&self.size),
            scale_factor: self.scale_factor,
        }
    }
}

impl Rc for SampleRenderHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapRenderHandler for SampleRenderHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_render_handler_t, Self>) {
        self.object = object;
    }
}

impl ImplRenderHandler for SampleRenderHandler {
    fn get_raw(&self) -> *mut sys::_cef_render_handler_t {
        self.object.cast()
    }

    fn view_rect(&self, _browser: Option<&mut Browser>, rect: Option<&mut Rect>) {
        if let Some(rect) = rect {
            let size = self.size.borrow();

            if size.width > 0 && size.height > 0 {
                rect.width = size.width as _;
                rect.height = size.height as _;
            }
        }
    }

    fn on_paint(
        &self,
        browser: Option<&mut Browser>,
        type_: PaintElementType,
        dirty_rects_count: usize,
        dirty_rects: Option<&Rect>,
        buffer: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) {
        println!("aaaaa");
        println!("1 {width} {height}");
    }
}
