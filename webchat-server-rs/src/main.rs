extern crate webchat_rs;
extern crate ws;

use ws::listen;
use webchat_rs::*;
use std::cell::RefCell;
use std::env;

struct WsMsg(Message);
impl Into<ws::Message> for WsMsg {
    fn into(self) -> ws::Message {
        ws::Message::Binary(serialize(self.0))
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <serve address>", args[0]);
        return;
    }

    let server = listen(&args[1], |out| {
        match out.send(WsMsg(Message::Ping)) {
            Ok(_) => println!("Ping?"),
            Err(e) => eprintln!("Error sending initial ping: {:?}", e)
        };

        let nick = RefCell::new("Anonymous".to_owned());

        move |msg| {
            if let ws::Message::Binary(buffer) = msg {
                let msg = deserialize(&buffer);
                match msg {
                    Message::Ping => {
                        out.send(WsMsg(Message::Pong))
                    },
                    Message::Pong => { println!("Pong!"); Ok(()) }
                    Message::Chat(content) => {
                        let msg = Message::Chat(format!("{}: {}", nick.borrow(), content));
                        out.broadcast(WsMsg(msg))
                    },
                    Message::Nick(value) => {
                        nick.replace(value.clone());
                        out.send(WsMsg(Message::Nick(value)))
                    },
                    Message::Me(content) => {
                        let msg = Message::Me(format!("{} {}", nick.borrow(), content));
                        out.broadcast(WsMsg(msg))
                    },
                }
            } else {
                Ok(())
            }
        }
    });

    match server {
        Ok(_) => println!("Exited successfully"),
        Err(e) => eprintln!("Server error: {:?}", e)
    };
}
