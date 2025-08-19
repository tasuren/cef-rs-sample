use cef::{
    ImplLifeSpanHandler, LifeSpanHandler, WrapLifeSpanHandler,
    rc::{Rc as _, RcImpl},
    sys::{_cef_life_span_handler_t, cef_life_span_handler_t},
};

pub struct LifeSpanHandlerService {
    internal: *mut RcImpl<cef_life_span_handler_t, Self>,
}

impl LifeSpanHandlerService {
    pub fn create() -> LifeSpanHandler {
        LifeSpanHandler::new(Self {
            internal: std::ptr::null_mut(),
        })
    }
}

impl WrapLifeSpanHandler for LifeSpanHandlerService {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_life_span_handler_t, Self>) {
        self.internal = object;
    }
}

impl cef::rc::Rc for LifeSpanHandlerService {
    fn as_base(&self) -> &cef::sys::cef_base_ref_counted_t {
        unsafe {
            let object = &*self.internal;
            std::mem::transmute(&object.cef_object)
        }
    }
}

impl Clone for LifeSpanHandlerService {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.internal;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self { internal: object }
    }
}

impl ImplLifeSpanHandler for LifeSpanHandlerService {
    fn get_raw(&self) -> *mut _cef_life_span_handler_t {
        self.internal.cast()
    }
}
