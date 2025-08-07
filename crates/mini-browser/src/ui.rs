use winit::window::Window;

use winit::{
    dpi::{LogicalPosition, PhysicalSize, Position, Size},
    event_loop::ActiveEventLoop,
    window::WindowAttributes,
};
use wry::WebView;

pub struct BrowserUI {
    window: Window,
    webview: WebView,
}

impl BrowserUI {
    /// タイトルやURLの操作、タブを配置する場所の高さ
    pub const HEADER_HEIGHT: u32 = 50;

    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Failed to create window");

        let webview = create_webview(&window);

        Self { window, webview }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn was_resized(&self, size: PhysicalSize<u32>) {
        self.webview
            .set_bounds(wry::Rect {
                position: Position::Logical(LogicalPosition { x: 0., y: 0. }),
                size: Size::Physical(size),
            })
            .unwrap();
    }
}

fn create_webview(window: &Window) -> WebView {
    wry::WebViewBuilder::new()
        .with_asynchronous_custom_protocol("wry".to_string(), wry_cmd::use_wry_cmd_protocol!("wry"))
        .with_html(include_str!("index.html"))
        .with_initialization_script(include_str!("main.js"))
        .build_as_child(&window)
        .expect("Failed to create webview")
}
