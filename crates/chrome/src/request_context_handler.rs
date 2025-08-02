use cef::{rc::*, *};

pub struct SampleRequestContextHandler {
    object: *mut RcImpl<sys::cef_request_context_handler_t, Self>,
}

impl SampleRequestContextHandler {
    pub fn new_request_context_handler() -> RequestContextHandler {
        RequestContextHandler::new(Self {
            object: std::ptr::null_mut(),
        })
    }
}

impl WrapRequestContextHandler for SampleRequestContextHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_request_context_handler_t, Self>) {
        self.object = object;
    }
}

impl Rc for SampleRequestContextHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let rc_impl = &*self.object;
            std::mem::transmute(&rc_impl.cef_object)
        }
    }
}

impl Clone for SampleRequestContextHandler {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self { object }
    }
}

impl ImplRequestContextHandler for SampleRequestContextHandler {
    fn get_raw(&self) -> *mut sys::_cef_request_context_handler_t {
        self.object.cast()
    }
}
