use std::{cell::RefCell, rc::Rc};

use cef::{
    Client, ContextMenuHandler, ImplClient, LifeSpanHandler, WrapClient,
    rc::{Rc as _, RcImpl},
};

use crate::cef_impl::{ContextMenuHandlerService, LifeSpanHandlerService};

pub struct SampleClient {
    object: *mut RcImpl<cef::sys::_cef_client_t, Self>,
    life_span_handler: LifeSpanHandler,
    context_menu_handler: ContextMenuHandler,
}

impl SampleClient {
    pub fn new_client() -> Client {
        let shared_client = Rc::new(RefCell::new(None));

        let client = Client::new(Self {
            object: std::ptr::null_mut(),
            life_span_handler: LifeSpanHandlerService::create(),
            context_menu_handler: ContextMenuHandlerService::create(Rc::clone(&shared_client)),
        });

        *shared_client.borrow_mut() = Some(client.clone());

        client
    }
}

impl WrapClient for SampleClient {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef::sys::_cef_client_t, Self>) {
        self.object = object;
    }
}

impl Clone for SampleClient {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            object,
            life_span_handler: self.life_span_handler.clone(),
            context_menu_handler: self.context_menu_handler.clone(),
        }
    }
}

impl cef::rc::Rc for SampleClient {
    fn as_base(&self) -> &cef::sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplClient for SampleClient {
    fn get_raw(&self) -> *mut cef::sys::_cef_client_t {
        self.object.cast()
    }

    fn context_menu_handler(&self) -> Option<cef::ContextMenuHandler> {
        Some(self.context_menu_handler.clone())
    }

    fn life_span_handler(&self) -> Option<cef::LifeSpanHandler> {
        Some(self.life_span_handler.clone())
    }
}
