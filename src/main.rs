#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::window;

fn start_app() {
    //let document = window()
    //    .and_then(|win| win.document())
    //    .expect("Could not access document");
    //let body = document.body().expect("Could not access document.body");
    //let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    //body.append_child(text_node.as_ref())
    //    .expect("Failed to append text");
}

#[wasm_bindgen]
pub fn wasm_hello() {
    web_sys::console::log_1(&"Hello from WASM!".into());
}

#[wasm_bindgen]
pub fn process_file_as_string(f: String)
{
    web_sys::console::log_1(&"WASM got this!".into());
    web_sys::console::log_1(&f.into());
}

fn main() {
    set_panic_hook();
    start_app();
}