use std::{num::NonZero, rc::Rc};

use softbuffer::Surface;
use winit::{dpi::PhysicalSize, window::Window};

pub struct WindowState {
    window: Rc<Window>,
    size: PhysicalSize<u32>,
    surface: Surface<Rc<Window>, Rc<Window>>,
}

impl WindowState {
    pub fn new(window: Window) -> Self {
        let window = Rc::new(window);

        let context = softbuffer::Context::new(Rc::clone(&window)).unwrap();
        let surface = Surface::new(&context, Rc::clone(&window)).unwrap();

        Self {
            size: window.inner_size(),
            surface,
            window,
        }
    }

    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;

        self.surface
            .resize(
                NonZero::new(size.width).unwrap(),
                NonZero::new(size.height).unwrap(),
            )
            .unwrap();
    }

    pub unsafe fn paint_raw(
        &mut self,
        buffer: *const u8,
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
    ) {
        let mut dest_buffer = self.surface.buffer_mut().unwrap();

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
