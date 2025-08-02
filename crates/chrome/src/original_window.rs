use std::{cell::RefCell, rc::Rc};

use cef::*;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, window::WindowAttributes,
};

use crate::*;

#[derive(Default)]
pub struct SampleWindowApp {
    browser: Option<Browser>,
    window: Option<winit::window::Window>,
    size: ViewSize,
}

impl ApplicationHandler for SampleWindowApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // ウィンドウを作る。
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();

        // 作ったウィンドウをCEFで使う準備として、レンダリング関連の設定を用意。
        let window_info = WindowInfo {
            // CEFにウィンドウを作らせないために必要。
            windowless_rendering_enabled: true as _,
            // 描画後の描画データを`on_paint`で受け取るのに必要。
            shared_texture_enabled: true as _,
            // こちら側から、再描画を指示できるよう設定。
            external_begin_frame_enabled: true as _,
            ..Default::default()
        };

        let scale_factor = window.scale_factor();
        let size = Rc::new(RefCell::new(window.inner_size().to_logical(scale_factor)));
        self.size = Rc::clone(&size);
        let render_handler = SampleRenderHandler::new_render_handler(size, scale_factor as _);

        let browser_settings = BrowserSettings {
            windowless_frame_rate: 30,
            ..Default::default()
        };
        let mut context = cef::request_context_create_context(
            Some(&RequestContextSettings::default()),
            Some(&mut SampleRequestContextHandler::new_request_context_handler()),
        );

        let browser = cef::browser_host_create_browser_sync(
            Some(&window_info),
            Some(&mut SampleClient::new_client(render_handler)),
            Some(&"https://www.google.com/".into()),
            Some(&browser_settings),
            None,
            context.as_mut(),
        );
        if browser.is_none() {
            panic!("ブラウザの起動に失敗しました。");
        }

        window.request_redraw();

        self.browser = browser;
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.redraw(),
            WindowEvent::Resized(size) => {
                self.resize(size.to_logical(self.window().scale_factor()))
            }
            _ => (),
        }
    }
}

impl SampleWindowApp {
    fn browser(&self) -> &Browser {
        self.browser
            .as_ref()
            .expect("Browser is not initialized yet")
    }

    fn window(&self) -> &winit::window::Window {
        self.window.as_ref().expect("Window is not initialized yet")
    }

    fn redraw(&self) {
        // 再描画が要求されてるので、新しいフレームの描画をCEFに要求する。
        if let Some(host) = self.browser().host() {
            println!("external");
            host.send_external_begin_frame();
        }
    }

    fn resize(&self, size: LogicalSize<u32>) {
        *self.size.borrow_mut() = size;

        // 可能であれば、CEFにウィンドウサイズが変更されたことを通知する。
        if let Some(host) = self.browser().host() {
            host.was_resized()
        }
    }
}
