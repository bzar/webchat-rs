extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;

use serde::{Serialize, Deserialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Message {
    Ping, Pong, Chat(String)
}

pub fn serialize(message: Message) -> Vec<u8> {
    let mut buffer = Vec::new();
    message.serialize(&mut Serializer::new(&mut buffer)).unwrap();
    buffer
}

pub fn deserialize(buffer: &[u8]) -> Message {
    Deserialize::deserialize(&mut Deserializer::new(buffer)).unwrap()
}


#[cfg(test)]
mod test {
    use ::*;
    #[test]
    fn serde_ping() {
        let ping = Message::Ping;
        let serialized = serialize(ping);
        let deserialized = deserialize(&serialized);
        assert!(deserialized == Message::Ping);
    }
}
