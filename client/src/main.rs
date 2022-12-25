use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::{io, thread};
use argparse::{ArgumentParser, Store};
use colored::Colorize;
use common::{Message, Options};

fn main() {
    let mut options = Options::new();

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Im client app");
        parser.refer(&mut options.port).add_option(&["--port"], Store, "Listen port");
        parser.refer(&mut options.host).add_option(&["--host"], Store, "Listen host");

        parser.parse_args_or_exit();
    }

    if let Ok(mut stream) = TcpStream::connect(&options.addr()) {
        let stream_clone = stream.try_clone().unwrap();

        let stdin = io::stdin();

        print!("{}", "Введите ваше имя: ".yellow());
        io::stdout().flush().unwrap();
        loop {
            if let Some(Ok(line)) = stdin.lock().lines().next() {
                thread::spawn(move || {
                    process_read(stream_clone, line);
                });
                break;
            }
        }

        loop {
            if let Some(Ok(line)) = stdin.lock().lines().next() {
                let _result = stream.write(line.as_bytes());
            }
        }
    } else {
        println!("Not connected");
    }
}

fn process_read(mut stream: TcpStream, name: String) {
    let _result = stream.write(format!("/name {}", name).as_bytes());

    loop {
        let mut buffer = [0 as u8; 1024];
        if let Ok(size) = stream.read(&mut buffer) {
            if let Some(message) = Message::from_bytes(&buffer[..size].to_vec()) {
                match message {
                    Message::Msg(author, message) => {
                        println!("{}: {}", author.name.green(), message)
                    }
                    Message::ClientConnected(user) => {
                        println!("*** {} {} {} ***", "User".yellow(), user.name.green(), "connected".yellow());
                    }
                    Message::ClientDisconnected(user) => {
                        println!("*** {} {} {} ***", "User".red(), user.name.green(), "disconnected".red());
                    }
                }
            }
        }
    }
}
