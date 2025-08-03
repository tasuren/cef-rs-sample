use std::{sync::mpsc::RecvTimeoutError, time::Duration};

use cef::{args::Args, *};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};

mod app;
mod browser_process_handler;
mod client;
mod original_window;
mod platform_impl;
mod render_handler;
mod request_context_handler;

pub use app::*;
pub use browser_process_handler::*;
pub use client::*;
pub use render_handler::*;
pub use request_context_handler::*;

fn main() -> std::process::ExitCode {
    #[cfg(target_os = "macos")]
    let _loader = {
        let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), false);
        assert!(loader.load());
        loader
    };

    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);

    let args = Args::new();
    let cmd = args.as_cmd_line().unwrap();

    let switch = CefString::from("type");
    let is_browser_process = cmd.has_switch(Some(&switch)) != 1;

    let (tx_pump, rx_pump) = std::sync::mpsc::channel();
    let mut app = SampleApp::new_app(tx_pump);

    let ret = execute_process(
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
        return std::process::ExitCode::SUCCESS;
    }

    // CEFの初期化を行う。
    let settings = Settings {
        windowless_rendering_enabled: true as _,
        external_message_pump: true as _,
        ..Default::default()
    };
    assert_eq!(
        initialize(
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
    let mut event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = original_window::SampleWindowApp::default();
    let mut delay = 0i64;

    let result = 'message_loop: loop {
        do_message_loop_work();

        let timeout = Some(std::time::Duration::ZERO);
        let status = event_loop.pump_app_events(timeout, &mut app);

        if let PumpStatus::Exit(exit_code) = status {
            break exit_code;
        }

        loop {
            delay = match rx_pump.recv_timeout(Duration::from_millis(delay as _)) {
                Ok(delay) => delay,
                Err(e) => match e {
                    RecvTimeoutError::Disconnected => break 'message_loop 0,
                    RecvTimeoutError::Timeout => {
                        delay = 0;
                        break;
                    }
                },
            };

            if delay <= 0 {
                break;
            }
        }
    };

    shutdown();

    std::process::ExitCode::from(result as u8)
}
