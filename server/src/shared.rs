use std::io::Write;
use std::net::SocketAddr;
use common::{Message, MessageAuthor};
use crate::client::Client;

#[derive(Default)]
pub struct Shared {
    clients: Vec<Client>
}

impl Shared {
    pub fn add_client(&mut self, client: Client) {
        for c in &self.clients {
            if c.addr == client.addr {
                return;
            }
        }

        self.clients.push(client);
    }

    pub fn set_name(&mut self, addr: &SocketAddr, name: String) {
        let index = self.clients.iter().position(|c| c.addr == *addr);
        if let Some(index) = index {
            self.clients[index].name = name.clone();
            self.msg(Some(addr), &Message::ClientConnected(MessageAuthor { name }))
        }
    }

    pub fn remove_client(&mut self, addr: &SocketAddr) {
        let index = self.clients.iter().position(|c| c.addr == *addr);
        if let Some(index) = index {
            let client = self.clients.remove(index);
            let msg = Message::ClientDisconnected(MessageAuthor { name: client.name });
            self.msg(None, &msg);
        }
    }

    pub fn msg(&mut self, addr: Option<&SocketAddr>, msg: &Message) {
        for client in &mut self.clients {
            if addr.is_some() && *addr.unwrap() == client.addr {
                continue;
            }
            let _result = client.stream.write(msg.as_bytes().as_slice());
        }
    }

    pub fn msg_text(&mut self, addr: &SocketAddr, text: &str) {
        println!("broadcast: {}", text);
        for client in &mut self.clients {
            if client.addr == *addr {
                let message = Message::msg(&client.name, text);
                self.msg(Some(addr), &message);
                break;
            }
        }
    }
}