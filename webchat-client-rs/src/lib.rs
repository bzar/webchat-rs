#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate webchat_rs;

use wasm_bindgen::prelude::*;
use webchat_rs::*;

#[wasm_bindgen(module = "./webchat_client")]
extern {
    fn send(msg: &[u8]);
    #[wasm_bindgen(js_name = addMessage)]
    fn add_message(msg: &str);
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
        Message::Chat(content) | Message::Me(content) => add_message(&content),
        Message::Nick(nick) => add_message(&format!("You are now known as {}", nick))
    };
}

#[wasm_bindgen]
pub fn input(msg: &str) {
    if msg.starts_with("/nick ") {
        if let Some(nick) = msg.splitn(2, " ").skip(1).next() {
            send(&serialize(Message::Nick(nick.to_owned())));
        } else {
            add_message("Usage: /nick <nick>");
        }
    } else if msg.starts_with("/me") {
        if let Some(content) = msg.splitn(2, " ").skip(1).next() {
            send(&serialize(Message::Me(content.to_owned())));
        } else {
            add_message("Usage: /me <action>");
        }
    } else if msg.starts_with("/") {
        add_message("Unknown command")
    } else {
        send(&serialize(Message::Chat(msg.to_owned())));
    }

}
