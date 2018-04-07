extern crate webchat_rs;
extern crate ws;

use ws::listen;
use webchat_rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let senders: &RefCell<Vec<Rc<RefCell<ws::Sender>>>> = &RefCell::new(Vec::new());

    listen("127.0.0.1:8081", |out| {
        let out = Rc::new(RefCell::new(out));
        senders.borrow_mut().push(Rc::clone(&out));
        println!("new connection, sending ping");
        out.borrow_mut().send(ws::Message::Binary(serialize(Message::Ping))).unwrap();

        move |msg| {
            if let ws::Message::Binary(buffer) = msg {
                let msg = deserialize(&buffer);
                match msg {
                    Message::Ping => {
                        println!("got ping, sending pong");
                        out.borrow_mut().send(ws::Message::Binary(serialize(Message::Pong))).unwrap()
                    },
                    Message::Pong => println!("got pong!"),
                    Message::Chat(content) => {
                        let msg = serialize(Message::Chat(content));
                        senders.borrow().iter().for_each(|out| {
                            out.borrow_mut().send(ws::Message::Binary(msg.clone())).unwrap();
                        });
                    }
                };
            }
            Ok(())
        }
    }).unwrap();
}
