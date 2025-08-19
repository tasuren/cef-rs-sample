use std::time::Duration;

use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoopProxy},
};

use crate::{
    browser::{set_browser},
    ui::BrowserUI,
};

#[derive(Default)]
pub struct AppState {
    pump_cef_scheduled: bool,
}

pub struct MiniBrowserApp {
    frame_rate: u64,
    browser: Option<cef::Browser>,
    browser_ui: Option<BrowserUI>,
    app_state: AppState,
}

impl MiniBrowserApp {
    pub fn new(frame_rate: u64) -> Self {
        Self {
            frame_rate,
            browser: None,
            browser_ui: None,
            app_state: AppState::default(),
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

impl ApplicationHandler<UserEvent> for MiniBrowserApp {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if matches!(cause, StartCause::ResumeTimeReached { .. }) {
            self.tick(event_loop);
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.browser_ui.is_some() {
            return;
        }

        self.set_normal_state(event_loop);

        // ウィンドウを作る。
        let browser_ui = BrowserUI::new(event_loop);
        set_browser(browser_ui.window(), self.frame_rate as _);

        self.browser_ui = Some(browser_ui);
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
            WindowEvent::Resized(size) => {
                if let Some(browser) = self.browser_ui.as_ref() {
                    browser.was_resized(size);
                }
            }
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
