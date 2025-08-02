use std::{cell::RefCell, num::NonZero, ptr::NonNull};

use cef::{rc::*, *};
use softbuffer::{Context, Surface};
use winit::dpi::PhysicalSize;

pub type ViewWindow = std::rc::Rc<winit::window::Window>;
pub type ViewSize = std::rc::Rc<RefCell<PhysicalSize<u32>>>;

/// softbufferを使ってOff-Screen Renderingするよう`RenderHandler`を実装した、構造体。
pub struct SampleRenderHandler {
    object: *mut RcImpl<cef::sys::_cef_render_handler_t, Self>,
    window: ViewWindow,
    surface: std::rc::Rc<RefCell<Surface<ViewWindow, ViewWindow>>>,
    size: ViewSize,
    scale_factor: f64,
}

impl SampleRenderHandler {
    pub fn new_render_handler(
        window: ViewWindow,
        size: ViewSize,
        scale_factor: f64,
    ) -> RenderHandler {
        let context = Context::new(std::rc::Rc::clone(&window)).unwrap();

        RenderHandler::new(Self {
            object: std::ptr::null_mut(),
            window: std::rc::Rc::clone(&window),
            surface: std::rc::Rc::new(RefCell::new(Surface::new(&context, window).unwrap())),
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
            window: std::rc::Rc::clone(&self.window),
            surface: std::rc::Rc::clone(&self.surface),
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
                let logical_size = size.to_logical::<i32>(self.scale_factor);
                rect.width = logical_size.width as _;
                rect.height = logical_size.height as _;

                self.surface
                    .borrow_mut()
                    .resize(
                        NonZero::new(size.width).unwrap(),
                        NonZero::new(size.height).unwrap(),
                    )
                    .unwrap();
            }
        }
    }

    fn screen_info(
        &self,
        _browser: Option<&mut Browser>,
        screen_info: Option<&mut ScreenInfo>,
    ) -> ::std::os::raw::c_int {
        if let Some(screen_info) = screen_info {
            screen_info.device_scale_factor = self.scale_factor as _;
            return true as _;
        }

        false as _
    }

    fn on_paint(
        &self,
        _browser: Option<&mut Browser>,
        _type: PaintElementType,
        _dirty_rects_count: usize,
        _dirty_rects: Option<&Rect>,
        source_buffer: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) {
        // bufferはBGRAの順でu8が並んだ、１ピクセルにつきu32のデータが並ぶ列の、
        // 最初のアドレスを示す生ポインタ。

        let mut surface = self.surface.borrow_mut();
        let mut dest_buffer = surface
            .buffer_mut()
            .expect("Failed to get buffer of surface");

        #[inline]
        unsafe fn unpack_bgra(mut buffer: NonNull<u8>) -> (NonNull<u8>, (u32, u32, u32, u32)) {
            unsafe {
                // Blue of BGRA
                let blue = buffer.read() as u32;

                // Green of BGRA
                buffer = buffer.add(1);
                let green = buffer.read() as u32;

                // Red of BGRA
                buffer = buffer.add(1);
                let red = buffer.read() as u32;

                // Alpha of BGRA
                buffer = buffer.add(1);
                let alpha = buffer.read() as u32;

                // 次は青なので、一回インクリメントする。
                buffer = buffer.add(1);

                (buffer, (blue, green, red, alpha))
            }
        }

        let mut source_buffer = unsafe { NonNull::new_unchecked(source_buffer as _) };
        let (mut b, mut g, mut r);

        for i in 0..(width * height) {
            (source_buffer, (b, g, r, _)) = unsafe { unpack_bgra(source_buffer) };

            dest_buffer[i as usize] = b | (g << 8) | (r << 16);
        }

        dest_buffer.present().unwrap();
    }
}
