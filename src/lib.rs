extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;

use serde::{Serialize, Deserialize};
use rmp_serde::{Deserializer, Serializer};
use rmp_serde::encode::Error as EncodeError;
use rmp_serde::decode::Error as DecodeError;

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Message {
    Ping, Pong, Chat(String), Nick(String), Me(String)
}

pub fn serialize(message: Message) -> Result<Vec<u8>, EncodeError> {
    let mut buffer = Vec::new();
    message.serialize(&mut Serializer::new(&mut buffer)).map(|_| buffer)
}

pub fn deserialize(buffer: &[u8]) -> Result<Message, DecodeError> {
    Deserialize::deserialize(&mut Deserializer::new(buffer))
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
