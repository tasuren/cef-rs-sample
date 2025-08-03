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

// FIXME: Rewrite this demo based on cef/tests/cefsimple
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

    let mut app = SampleApp::new_app();

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
    let result = loop {
        do_message_loop_work();
        let timeout = Some(std::time::Duration::ZERO);
        let status = event_loop.pump_app_events(timeout, &mut app);

        if let PumpStatus::Exit(exit_code) = status {
            break std::process::ExitCode::from(exit_code as u8);
        }

        // ↓このマジックナンバーはなんだろう。60FPSに固定するため？
        std::thread::sleep(std::time::Duration::from_millis(1000 / 17));

        // TODO: ここで待機しないようにする。`OnScheduleMessagePumpWork`を使う？
    };

    shutdown();

    result
}
