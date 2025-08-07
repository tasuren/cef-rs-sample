use cef::{ImplBrowser, ImplFrame};
use serde::Deserialize;
use wry_cmd::command;

use crate::browser::get_browser;

#[derive(Deserialize)]
struct LoadUrlArgs {
    url: String,
}

#[command]
fn load_url(args: LoadUrlArgs) -> bool {
    println!("aaa");
    if let Some(frame) = get_browser().main_frame() {
        println!("bbb");
        frame.load_url(Some(&args.url.as_str().into()));

        true
    } else {
        false
    }
}
