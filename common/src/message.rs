use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct MessageAuthor {
    pub name: String
}

#[derive(Serialize,Deserialize)]
pub enum Message {
    Msg(MessageAuthor, String),
    ClientConnected(MessageAuthor),
    ClientDisconnected(MessageAuthor),
}

impl Message {
    pub fn msg(name: &str, text: &str) -> Self {
        Self::Msg(
            MessageAuthor {
                name: name.to_string(),
            },
            text.to_string()
        )
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        bincode::deserialize(bytes).ok()
    }
}