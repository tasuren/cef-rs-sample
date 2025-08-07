use cef::{
    BrowserProcessHandler, CommandLine, ImplBrowserProcessHandler, ImplCommandLine,
    WrapBrowserProcessHandler, rc::Rc as _,
};

use crate::app::PumpCefHandle;

pub struct SampleBrowserProcessHandler {
    object: *mut cef::rc::RcImpl<cef::sys::cef_browser_process_handler_t, Self>,
    pump_cef_handle: PumpCefHandle,
}

impl SampleBrowserProcessHandler {
    pub fn new_browser_process_handler(pump_cef_handle: PumpCefHandle) -> BrowserProcessHandler {
        BrowserProcessHandler::new(Self {
            object: std::ptr::null_mut(),
            pump_cef_handle,
        })
    }
}

impl cef::rc::Rc for SampleBrowserProcessHandler {
    fn as_base(&self) -> &cef::sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapBrowserProcessHandler for SampleBrowserProcessHandler {
    fn wrap_rc(
        &mut self,
        object: *mut cef::rc::RcImpl<cef::sys::_cef_browser_process_handler_t, Self>,
    ) {
        self.object = object;
    }
}

impl Clone for SampleBrowserProcessHandler {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            object,
            pump_cef_handle: self.pump_cef_handle.clone(),
        }
    }
}

impl ImplBrowserProcessHandler for SampleBrowserProcessHandler {
    fn get_raw(&self) -> *mut cef::sys::_cef_browser_process_handler_t {
        self.object.cast()
    }

    fn on_before_child_process_launch(&self, command_line: Option<&mut CommandLine>) {
        if let Some(command_line) = command_line {
            command_line.append_switch(Some(&"enable-logging=stderr".into()));
        }
    }

    fn on_context_initialized(&self) {
        println!("cef context intiialized");
    }

    fn on_schedule_message_pump_work(&self, delay_ms: i64) {
        println!("on_schedule_message_pump_work: delay_ms = {delay_ms}ms");
        self.pump_cef_handle.send_pump_cef_event(delay_ms);
    }
}
