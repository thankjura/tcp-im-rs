use std::net::{SocketAddr, TcpStream};

pub struct Client {
    pub stream: TcpStream,
    pub addr: SocketAddr,
    pub name: String,
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            stream,
            addr,
            name: addr.to_string(),
        }
    }
}