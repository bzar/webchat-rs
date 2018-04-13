extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{deserialize as bin_de, serialize as bin_ser, Error};

#[derive(Serialize, Deserialize,Debug,PartialEq)]
pub enum Message {
    Ping, Pong, Chat(String), Nick(String), Me(String)
}

pub fn serialize(message: Message) -> Result<Vec<u8>, Error> {
    bin_ser(&message)
}

pub fn deserialize(buffer: &[u8]) -> Result<Message, Error> {
    bin_de(buffer)
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
