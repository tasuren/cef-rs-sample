use std::sync::OnceLock;

use winit::window::Window;

use crate::{cef_impl, ui::BrowserUI};

pub static BROWSER: OnceLock<cef::Browser> = OnceLock::new();

pub fn get_browser() -> &'static cef::Browser {
    BROWSER.get().expect("The browser is not initialized yet")
}

/// CEFのブラウザを用意する。
pub fn set_browser(window: &Window, _frame_rate: i32) {
    let size = window.inner_size().to_logical(window.scale_factor());

    let bounds = cef::Rect {
        x: 0,
        y: BrowserUI::HEADER_HEIGHT as _,
        width: size.width,
        height: size.height - BrowserUI::HEADER_HEIGHT as i32,
    };

    let window_info = cef::WindowInfo {
        bounds,
        parent_view: crate::platform_impl::raw_view::get_view(window),
        ..Default::default()
    };

    // ブラウザの作成を行う。
    let browser_settings = cef::BrowserSettings::default();
    let mut context = cef::request_context_create_context(
        Some(&cef::RequestContextSettings::default()),
        Some(&mut cef_impl::SampleRequestContextHandler::new_request_context_handler()),
    );

    let url = std::env::var("URL")
        .ok()
        .unwrap_or_else(|| "https://www.google.com".to_owned());

    let mut client = cef_impl::SampleClient::new_client();
    let browser = cef::browser_host_create_browser_sync(
        Some(&window_info),
        Some(&mut client),
        Some(&url.as_str().into()),
        Some(&browser_settings),
        None,
        context.as_mut(),
    )
    .expect("Failed to create browser");

    if BROWSER.set(browser).is_err() {
        panic!("Failed to set browser")
    };
}
