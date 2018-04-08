extern crate webchat_rs;
extern crate ws;

use ws::listen;
use webchat_rs::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::env;

fn send(sender: &mut ws::Sender, message: Message) -> ws::Result<()> {
    sender.send(ws::Message::Binary(serialize(message)))
}
fn send_binary(sender: &mut ws::Sender, buffer: Vec<u8>) -> ws::Result<()> {
    sender.send(ws::Message::Binary(buffer))
}
struct Senders(Vec<Rc<RefCell<ws::Sender>>>);

impl Senders {
    fn new() -> Senders {
        Senders(Vec::new())
    }
    fn add(&mut self, sender: Rc<RefCell<ws::Sender>>) {
        self.0.push(sender);
    }
    fn broadcast(&mut self, message: Message) {
        let msg = serialize(message);
        self.0.iter().for_each(|out| {
            send_binary(&mut out.borrow_mut(), msg.clone()).unwrap();
        });
    }
}
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <serve address>", args[0]);
        return;
    }
    let senders: &RefCell<Senders> = &RefCell::new(Senders::new());

    listen(&args[1], |out| {
        let out = Rc::new(RefCell::new(out));
        senders.borrow_mut().add(Rc::clone(&out));
        println!("new connection, sending ping");
        send(&mut out.borrow_mut(), Message::Ping).unwrap();
        let nick = RefCell::new("Anonymous".to_owned());

        move |msg| {
            if let ws::Message::Binary(buffer) = msg {
                let msg = deserialize(&buffer);
                match msg {
                    Message::Ping => {
                        println!("got ping, sending pong");
                        send(&mut out.borrow_mut(), Message::Pong).unwrap();
                    },
                    Message::Pong => println!("got pong!"),
                    Message::Chat(content) => {
                        let msg = Message::Chat(format!("{}: {}", nick.borrow(), content));
                        senders.borrow_mut().broadcast(msg);
                    },
                    Message::Nick(value) => {
                        nick.replace(value.clone());
                        send(&mut out.borrow_mut(), Message::Nick(value)).unwrap();
                    },
                    Message::Me(content) => {
                        let msg = Message::Me(format!("{} {}", nick.borrow(), content));
                        senders.borrow_mut().broadcast(msg);
                    },
                };
            }
            Ok(())
        }
    }).unwrap();
}
