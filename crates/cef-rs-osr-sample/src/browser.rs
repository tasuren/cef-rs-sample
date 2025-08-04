use std::{cell::RefCell, rc::Rc};

use crate::cef_impl::{self, ViewSize, ViewWindow};

pub fn create_browser(window: &ViewWindow, frame_rate: i32) -> (ViewSize, cef::Browser) {
    // 作ったウィンドウをCEFで使う準備として、レンダリング関連の設定を用意。
    let window_info = cef::WindowInfo {
        // CEFにウィンドウを作らせないために必要。
        windowless_rendering_enabled: true as _,
        ..Default::default()
    };

    // 描画関連の実装を用意する。
    let size = Rc::new(RefCell::new(window.inner_size()));
    let render_handler =
        cef_impl::SampleRenderHandler::new_render_handler(Rc::clone(window), Rc::clone(&size));

    // ブラウザの作成を行う。
    let browser_settings = cef::BrowserSettings {
        windowless_frame_rate: frame_rate,
        ..Default::default()
    };
    let mut context = cef::request_context_create_context(
        Some(&cef::RequestContextSettings::default()),
        Some(&mut cef_impl::SampleRequestContextHandler::new_request_context_handler()),
    );

    let url = std::env::var("URL")
        .ok()
        .unwrap_or_else(|| "https://bevy.org/examples/3d-rendering/motion-blur/".to_owned());

    let browser = cef::browser_host_create_browser_sync(
        Some(&window_info),
        Some(&mut cef_impl::SampleClient::new_client(render_handler)),
        Some(&url.as_str().into()),
        Some(&browser_settings),
        None,
        context.as_mut(),
    );

    (size, browser.expect("Failed to create browser"))
}
