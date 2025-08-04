use std::{cell::RefCell, num::NonZero};

use cef::{
    Browser, ImplRenderHandler, Rect, RenderHandler, ScreenInfo, WrapRenderHandler,
    rc::{Rc as _, RcImpl},
    sys::{_cef_base_ref_counted_t, _cef_render_handler_t},
};
use softbuffer::{Context, Surface};
use winit::dpi::PhysicalSize;

pub type ViewWindow = std::rc::Rc<winit::window::Window>;
pub type ViewSize = std::rc::Rc<RefCell<PhysicalSize<u32>>>;

/// softbufferを使ってOff-Screen Renderingするよう`RenderHandler`を実装した構造体。
pub struct SampleRenderHandler {
    object: *mut RcImpl<cef::sys::_cef_render_handler_t, Self>,
    window: ViewWindow,
    surface: std::rc::Rc<RefCell<Surface<ViewWindow, ViewWindow>>>,
    size: ViewSize,
}

impl SampleRenderHandler {
    pub fn new_render_handler(window: ViewWindow, size: ViewSize) -> RenderHandler {
        let context = Context::new(std::rc::Rc::clone(&window)).unwrap();

        RenderHandler::new(Self {
            object: std::ptr::null_mut(),
            window: std::rc::Rc::clone(&window),
            surface: std::rc::Rc::new(RefCell::new(Surface::new(&context, window).unwrap())),
            size,
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
            let size = self.size.borrow();

            if size.width > 0 && size.height > 0 {
                let scale_factor = self.window.scale_factor();
                let logical_size = size.to_logical::<i32>(scale_factor);

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
        // これを実装しない場合、恐らく物理ピクセルで計算される？
        // でも念のため、拡大率を教えておく。

        if let Some(screen_info) = screen_info {
            screen_info.device_scale_factor = self.window.scale_factor() as _;
            return true as _;
        }

        false as _
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn on_paint(
        &self,
        _browser: Option<&mut Browser>,
        _type: cef::PaintElementType,
        _dirty_rects_count: usize,
        _dirty_rects: Option<&Rect>,
        buffer: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) {
        let mut surface = self.surface.borrow_mut();
        let mut dest_buffer = surface
            .buffer_mut()
            .expect("Failed to get buffer of surface");

        // CEFから提供される`buffer`は、1ピクセルにつきu8が4つ分のBGRAの塊(u32)の列の先頭のポインタ。
        // このため、ここから全ピクセル数分までは画像データであり、そこまでを配列として参照する。
        let pixel_count = (width * height) as usize;
        let len = pixel_count * size_of::<u32>();
        let source_slice = unsafe { std::slice::from_raw_parts(buffer, len) };

        // そのままだとデータはBGRAのチ列なので、softbufferの要求する0RGBの形に変換する必要がある。
        // そのため、変換のためにBGRAを分解してRGBを取り出したい。そこで、u8のBGRAのチャンクで分割。
        let source_pixels = source_slice.chunks_exact(size_of::<u32>());

        // １ピクセル毎、バッファに書き込んでいく。
        for (dest_pixel, src_pixel) in dest_buffer.iter_mut().zip(source_pixels) {
            let b = src_pixel[0] as u32;
            let g = src_pixel[1] as u32;
            let r = src_pixel[2] as u32;
            // let a = src_pixel[3] as u32; // softbufferはAlphaに対応していない。

            // 0RGBに再配置。
            *dest_pixel = (r << 16) | (g << 8) | b;
        }

        // ウィンドウに描画依頼。
        dest_buffer.present().unwrap();
    }
}
