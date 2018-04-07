extern crate webchat_rs;
extern crate ws;

use ws::listen;
use webchat_rs::*;

fn main() {
    listen("127.0.0.1:8081", |out| {
        println!("new connection, sending ping");
        out.send(ws::Message::Binary(serialize(Message::Ping))).unwrap();

        move |msg| {
            if let ws::Message::Binary(buffer) = msg {
                let msg = deserialize(&buffer);
                match msg {
                    Message::Ping => {
                        println!("got ping, sending pong");
                        out.send(ws::Message::Binary(serialize(Message::Pong))).unwrap()
                    },
                    Message::Pong => println!("got pong!")
                };
            }
            Ok(())
        }
    }).unwrap();
}
