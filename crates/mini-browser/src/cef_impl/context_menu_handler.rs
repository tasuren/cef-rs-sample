use std::{cell::RefCell, os::raw, rc::Rc};

use cef::{
    ContextMenuHandler, ImplBrowser, ImplBrowserHost, ImplContextMenuHandler,
    ImplContextMenuParams, ImplMenuModel, WrapContextMenuHandler,
    rc::{Rc as _, RcImpl},
    sys::{cef_context_menu_handler_t, cef_menu_id_t},
};

pub struct ContextMenuHandlerService {
    internal: *mut RcImpl<cef_context_menu_handler_t, Self>,
    client: Rc<RefCell<Option<cef::Client>>>,
}

impl ContextMenuHandlerService {
    pub fn create(client: Rc<RefCell<Option<cef::Client>>>) -> ContextMenuHandler {
        ContextMenuHandler::new(Self {
            internal: std::ptr::null_mut(),
            client,
        })
    }
}

impl WrapContextMenuHandler for ContextMenuHandlerService {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_context_menu_handler_t, Self>) {
        self.internal = object;
    }
}

impl cef::rc::Rc for ContextMenuHandlerService {
    fn as_base(&self) -> &cef::sys::cef_base_ref_counted_t {
        unsafe {
            let object = &*self.internal;
            std::mem::transmute(&object.cef_object)
        }
    }
}

impl Clone for ContextMenuHandlerService {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.internal;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            internal: object,
            client: Rc::clone(&self.client),
        }
    }
}

pub const MENU_ID_INSPECT_ELEMENT: raw::c_int = cef_menu_id_t::MENU_ID_USER_FIRST as raw::c_int + 1;

impl ImplContextMenuHandler for ContextMenuHandlerService {
    fn get_raw(&self) -> *mut cef_context_menu_handler_t {
        self.internal.cast()
    }

    fn on_before_context_menu(
        &self,
        browser: Option<&mut cef::Browser>,
        frame: Option<&mut cef::Frame>,
        params: Option<&mut cef::ContextMenuParams>,
        model: Option<&mut cef::MenuModel>,
    ) {
        if let Some(model) = model {
            model.insert_item_at(5, MENU_ID_INSPECT_ELEMENT, Some(&"Inspect Element".into()));
        }
    }

    fn on_context_menu_command(
        &self,
        browser: Option<&mut cef::Browser>,
        _frame: Option<&mut cef::Frame>,
        params: Option<&mut cef::ContextMenuParams>,
        command_id: ::std::os::raw::c_int,
        _event_flags: cef::EventFlags,
    ) -> ::std::os::raw::c_int {
        if cef::currently_on(cef::ThreadId::default()) == 0 {
            println!("not ui thread");
            return true as _;
        }

        match command_id {
            MENU_ID_INSPECT_ELEMENT => {
                let Some(browser_host) = browser.and_then(|browser| browser.host()) else {
                    return true as _;
                };
                let Some(params) = params else {
                    return true as _;
                };

                browser_host.show_dev_tools(
                    None,
                    None::<&mut cef::Client>,
                    None,
                    Some(&cef::Point {
                        x: params.xcoord(),
                        y: params.ycoord(),
                    }),
                );
            }
            _ => {}
        }

        true as _
    }
}
