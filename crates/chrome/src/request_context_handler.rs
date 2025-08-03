use cef::{
    ImplRequestContextHandler, RequestContextHandler, WrapRequestContextHandler,
    rc::{Rc as _, RcImpl},
    sys::{_cef_base_ref_counted_t, _cef_request_context_handler_t},
};

pub struct SampleRequestContextHandler {
    object: *mut RcImpl<_cef_request_context_handler_t, Self>,
}

impl SampleRequestContextHandler {
    pub fn new_request_context_handler() -> RequestContextHandler {
        RequestContextHandler::new(Self {
            object: std::ptr::null_mut(),
        })
    }
}

impl WrapRequestContextHandler for SampleRequestContextHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<_cef_request_context_handler_t, Self>) {
        self.object = object;
    }
}

impl cef::rc::Rc for SampleRequestContextHandler {
    fn as_base(&self) -> &_cef_base_ref_counted_t {
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
    fn get_raw(&self) -> *mut _cef_request_context_handler_t {
        self.object.cast()
    }
}
