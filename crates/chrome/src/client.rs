use cef::{
    Client, ImplClient, RenderHandler, WrapClient,
    rc::{Rc as _, RcImpl},
};

pub struct SampleClient {
    object: *mut RcImpl<cef::sys::_cef_client_t, Self>,
    render_handler: RenderHandler,
}

impl SampleClient {
    pub fn new_client(render_handler: RenderHandler) -> Client {
        Client::new(Self {
            object: std::ptr::null_mut(),
            render_handler,
        })
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
            render_handler: self.render_handler.clone(),
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

    fn render_handler(&self) -> Option<RenderHandler> {
        // softbufferを使ってウィンドウに描画する`RenderHandler`を使ってもらう。
        Some(self.render_handler.clone())
    }
}
