use std::io::{Write};
use std::net::{TcpStream};
use std::process::Command;

pub struct C2 {
    pub clients: Vec<Client>,
    command: Vec<String>
}

impl C2 {
    pub fn new(command: Vec<String>) -> Self {
        let clients = vec![];
        Self { clients, command }
    }

    pub fn send_commands(&mut self) {
        for client in self.clients.iter_mut() {
            for command in self.command.iter() {
                if let Err(e) = client.stream.write_all(command.as_bytes()) {
                    println!("Failed to send command {}", e);
                } 
            }
        }
    }
}

pub struct Client {
    stream: TcpStream, 
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
}






