use std::rc::Rc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoopProxy},
    window::WindowAttributes,
};

use crate::{
    cef_impl::{ViewSize, ViewWindow},
    *,
};

#[derive(Default)]
pub struct AppState {
    pump_cef_scheduled: bool,
}

pub struct CefState {
    browser: cef::Browser,
    window: ViewWindow,
    size: ViewSize,
}

pub struct CefWithOsrApp {
    frame_rate: u64,
    app_state: AppState,
    cef_state: Option<CefState>,
}

impl CefWithOsrApp {
    pub fn new(frame_rate: u64) -> Self {
        Self {
            frame_rate,
            cef_state: None,
            app_state: AppState::default(),
        }
    }

    fn cef_state(&self) -> &CefState {
        self.cef_state
            .as_ref()
            .expect("Browser is not initialized yet")
    }

    fn browser(&self) -> &Browser {
        &self.cef_state().browser
    }

    fn resize(&self, size: PhysicalSize<u32>) {
        *self.cef_state().size.borrow_mut() = size;

        if let Some(host) = self.browser().host() {
            host.was_resized()
        }
    }

    fn set_normal_state(&mut self, event_loop: &ActiveEventLoop) {
        let duration = Duration::from_millis(1000 / self.frame_rate);
        event_loop.set_control_flow(ControlFlow::wait_duration(duration));
        self.app_state.pump_cef_scheduled = false;
    }

    fn schedule_pump_cef(&mut self, event_loop: &ActiveEventLoop, delay_ms: u64) {
        self.app_state.pump_cef_scheduled = true;
        event_loop.set_control_flow(ControlFlow::wait_duration(Duration::from_millis(delay_ms)))
    }

    fn tick(&mut self, event_loop: &ActiveEventLoop) {
        cef::do_message_loop_work();

        if self.app_state.pump_cef_scheduled {
            // もし、スケジュールされた仕事が終わったのなら、スケジュールの`delay_ms`の予定を削除し元に戻す。
            self.set_normal_state(event_loop);
        }
    }
}

impl ApplicationHandler<UserEvent> for CefWithOsrApp {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if matches!(cause, StartCause::ResumeTimeReached { .. }) {
            self.tick(event_loop);
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.cef_state.is_some() {
            return;
        }

        self.set_normal_state(event_loop);

        // ウィンドウを作る。
        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();
        let window = Rc::new(window);

        // ブラウザの用意。
        let (size, browser) = crate::browser::create_browser(&window, self.frame_rate as _);

        self.cef_state = Some(CefState {
            browser,
            window,
            size,
        });

        self.cef_state().window.request_redraw();
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::PumpCef { delay_ms } => {
                if delay_ms > 0 {
                    self.schedule_pump_cef(event_loop, delay_ms as _);
                } else {
                    self.tick(event_loop);
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => self.resize(size),
            _ => (),
        }
    }
}

#[non_exhaustive]
pub enum UserEvent {
    PumpCef { delay_ms: i64 },
}

#[derive(Clone)]
pub struct PumpCefHandle {
    event_loop: EventLoopProxy<UserEvent>,
}

impl PumpCefHandle {
    pub fn new(event_loop: EventLoopProxy<UserEvent>) -> Self {
        Self { event_loop }
    }

    pub fn send_pump_cef_event(&self, delay_ms: i64) {
        if self
            .event_loop
            .send_event(UserEvent::PumpCef { delay_ms })
            .is_err()
        {
            eprintln!("[WARN] Failed to send pump cef event to winit event loop")
        };
    }
}
