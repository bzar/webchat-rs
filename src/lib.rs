extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;

use rmp_serde::encode::Error as EncodeError;
use rmp_serde::decode::Error as DecodeError;

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Message {
    Ping, Pong, Chat(String), Nick(String), Me(String)
}

pub fn serialize(message: Message) -> Result<Vec<u8>, EncodeError> {
    rmp_serde::to_vec(&message)
}

pub fn deserialize(buffer: &[u8]) -> Result<Message, DecodeError> {
    rmp_serde::from_slice(buffer)
}


#[cfg(test)]
mod test {
    use ::*;
    #[test]
    fn serde_ping() {
        let ping = Message::Ping;
        let serialized = serialize(ping).unwrap();
        let deserialized = deserialize(&serialized).unwrap();
        assert!(deserialized == Message::Ping);
    }
}
