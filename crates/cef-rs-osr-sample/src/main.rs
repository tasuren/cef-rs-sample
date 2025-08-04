use std::time::Duration;

use cef::{CefString, ImplCommandLine};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::PumpCefHandle;

mod app;
mod browser;
mod cef_impl;
mod platform_impl;

fn main() {
    let event_loop = EventLoop::with_user_event().build().unwrap();

    // CEFのライブラリの読み込みとプロセスの起動。
    #[cfg(target_os = "macos")]
    let _loader = {
        let loader =
            cef::library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), false);
        assert!(loader.load());
        loader
    };

    let _ = cef::api_hash(cef::sys::CEF_API_VERSION_LAST, 0);

    let args = cef::args::Args::new();
    let cmd = args.as_cmd_line().unwrap();

    let switch = CefString::from("type");
    let is_browser_process = cmd.has_switch(Some(&switch)) != 1;

    let mut app = cef_impl::SampleApp::new_app(PumpCefHandle::new(event_loop.create_proxy()));

    let ret = cef::execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        std::ptr::null_mut(),
    );

    if is_browser_process {
        println!("launch browser process");
        assert!(ret == -1, "cannot execute browser process");
    } else {
        let process_type = CefString::from(&cmd.switch_value(Some(&switch)));
        println!("launch process {process_type}");
        assert!(ret >= 0, "cannot execute non-browser process");
        // ブラウザプロセス以外のヘルパーのプロセスの処理は、ここまで。
        return;
    }

    // CEFの初期化を行う。
    let settings = cef::Settings {
        windowless_rendering_enabled: true as _,
        external_message_pump: true as _,
        ..Default::default()
    };
    assert_eq!(
        cef::initialize(
            Some(args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            std::ptr::null_mut(),
        ),
        1
    );

    // macOSで右クリックができないのを修正する。
    #[cfg(target_os = "macos")]
    unsafe {
        platform_impl::macos::extend_nswindow_class()
    };

    // イベントループを動かす。
    const FPS: u64 = 60;
    let mut app = app::CefWithOsrApp::new(FPS);

    event_loop.run_app(&mut app).unwrap();

    cef::shutdown();
}
