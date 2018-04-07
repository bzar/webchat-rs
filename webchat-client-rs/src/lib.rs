#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate webchat_rs;

use wasm_bindgen::prelude::*;
use webchat_rs::*;

#[wasm_bindgen(module = "./webchat_client")]
extern {
    fn send(msg: &[u8]);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    send(&serialize(Message::Ping));
    send(&serialize(Message::Chat("Hello World!".to_owned())));
}

#[wasm_bindgen]
pub fn recv(buffer: &[u8]) {
    let msg = deserialize(buffer);
    match msg {
        Message::Ping => send(&serialize(Message::Pong)),
        Message::Pong => log("got Pong!"),
        Message::Chat(content) => log(&content)
    };
}
