mod shared;
mod client;
mod command;

use std::io::Read;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use argparse::{ArgumentParser, Store};
use common::Options;
use crate::client::Client;
use crate::command::Command;
use crate::shared::Shared;


fn main() -> std::io::Result<()> {
    let mut options = Options::new();

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Im server app");
        parser.refer(&mut options.port).add_option(&["--port"], Store, "Listen port");
        parser.refer(&mut options.host).add_option(&["--host"], Store, "Listen host");

        parser.parse_args_or_exit();
    }

    let listener = TcpListener::bind(&options.addr())?;
    let shared = Arc::new(Mutex::new(Shared::default()));

    loop {
        if let Ok((stream, addr)) = listener.accept() {
            let shared = shared.clone();
            thread::spawn(move || {
                process(stream, addr, shared);
            });
        }
    }
}

fn process(mut stream: TcpStream, addr: SocketAddr, shared: Arc<Mutex<Shared>>) {
    let client = Client::new(stream.try_clone().unwrap(), addr);
    shared.lock().unwrap().add_client(client);
    let mut buffer = [0 as u8; 1024];
    loop {
        if let Ok(size) = stream.read(&mut buffer) {
            if size == 0 {
                break;
            }
            if let Ok(text) = String::from_utf8(buffer[..size].to_vec()) {
                let text = text.trim().to_string();
                if let Some(command) = Command::from(&text) {
                    match command {
                        Command::Rename(name) => {
                            shared.lock().unwrap().set_name(&addr, name);
                        }
                        _ => {}
                    }
                } else {
                    shared.lock().unwrap().msg_text(&addr, &text);
                }
            } else {
                eprintln!("can't read string from client: {:#?}", addr);
            }
        } else {
            break;
        }
    }
    stream.shutdown(Shutdown::Both).unwrap();
    shared.lock().unwrap().remove_client(&addr);
    println!("Client {} disconnected", addr);
}
