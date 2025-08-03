use cef::{
    ImplApp, WrapApp,
    rc::{Rc as _, RcImpl},
};

use crate::*;

pub struct SampleApp {
    object: *mut RcImpl<cef::sys::_cef_app_t, Self>,
    tx_pump: TxPump,
}

impl SampleApp {
    pub fn new_app(tx_pump: TxPump) -> App {
        App::new(Self {
            object: std::ptr::null_mut(),
            tx_pump,
        })
    }
}

impl WrapApp for SampleApp {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef::sys::_cef_app_t, Self>) {
        self.object = object;
    }
}

impl Clone for SampleApp {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            object,
            tx_pump: self.tx_pump.clone(),
        }
    }
}

impl cef::rc::Rc for SampleApp {
    fn as_base(&self) -> &cef::sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplApp for SampleApp {
    fn get_raw(&self) -> *mut cef::sys::_cef_app_t {
        self.object.cast()
    }

    fn on_before_command_line_processing(
        &self,
        _process_type: Option<&CefString>,
        command_line: Option<&mut CommandLine>,
    ) {
        if let Some(command_line) = command_line {
            // 毎回デバッグビルドを起動する度にキーチェーンのパスワードを求められるのを防止。
            #[cfg(debug_assertions)]
            command_line.append_switch(Some(&"use-mock-keychain".into()));

            command_line.append_switch(Some(&"enable-logging=stderr".into()));

            // 開発者ツールを`chrome://inspect`から開けるようにする。
            command_line.append_switch_with_value(
                Some(&"remote-debugging-port".into()),
                Some(&"9229".into()),
            );
        }
    }

    fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
        Some(SampleBrowserProcessHandler::new_browser_process_handler(
            self.tx_pump.clone(),
        ))
    }
}
