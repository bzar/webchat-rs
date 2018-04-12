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

fn send_message(msg: Message) {
    // Message serialization should ALWAYS succeed
    send(&serialize(msg).unwrap());
}
#[wasm_bindgen]
pub fn main() {
    send_message(Message::Ping);
    send_message(Message::Chat("Hello World!".to_owned()));
}

#[wasm_bindgen]
pub fn recv(buffer: &[u8]) {
    if let Ok(msg) = deserialize(buffer) {
        match msg {
            Message::Ping => send_message(Message::Pong),
            Message::Pong => log("got Pong!"),
            Message::Chat(content) | Message::Me(content) => add_message(&content),
            Message::Nick(nick) => {
                let mut msg = "You are now known as ".to_owned();
                msg.push_str(&nick);
                add_message(&msg);
            }
        };
    } else {
        log("Received invalid message!");
    }
}

#[wasm_bindgen]
pub fn input(msg: &str) {
    if msg.starts_with("/nick") {
        if let Some(nick) = msg.splitn(2, " ").skip(1).next() {
            send_message(Message::Nick(nick.to_owned()));
        } else {
            add_message("Usage: /nick <nick>");
        }
    } else if msg.starts_with("/me") {
        if let Some(content) = msg.splitn(2, " ").skip(1).next() {
            send_message(Message::Me(content.to_owned()));
        } else {
            add_message("Usage: /me <action>");
        }
    } else if msg.starts_with("/") {
        add_message("Unknown command")
    } else {
        send_message(Message::Chat(msg.to_owned()));
    }
}
