mod message;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

pub use message::Message;
pub use message::MessageAuthor;


#[derive(Debug)]
pub struct Options {
    pub host: IpAddr,
    pub port: u16,
}

impl Options {
    pub fn new() -> Self {
        Self {
            host: IpAddr::from_str("0.0.0.0").unwrap(),
            port: 8088,
        }
    }

    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}