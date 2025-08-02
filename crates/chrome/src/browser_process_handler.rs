use std::sync::{Arc, Mutex};

use cef::{rc::*, *};

pub struct SampleBrowserProcessHandler {
    object: *mut RcImpl<cef::sys::cef_browser_process_handler_t, Self>,
    window: Arc<Mutex<Option<Window>>>,
}

impl SampleBrowserProcessHandler {
    pub fn new_browser_process_handler(
        window: Arc<Mutex<Option<Window>>>,
    ) -> BrowserProcessHandler {
        BrowserProcessHandler::new(Self {
            object: std::ptr::null_mut(),
            window,
        })
    }
}

impl Rc for SampleBrowserProcessHandler {
    fn as_base(&self) -> &cef::sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapBrowserProcessHandler for SampleBrowserProcessHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef::sys::_cef_browser_process_handler_t, Self>) {
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

        let window = self.window.clone();

        Self { object, window }
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

    // The real lifespan of cef starts from `on_context_initialized`, so all the cef objects should be manipulated after that.
    fn on_context_initialized(&self) {
        println!("cef context intiialized");
    }
}
