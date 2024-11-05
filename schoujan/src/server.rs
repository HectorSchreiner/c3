use std::io::{Write};
use std::net::{TcpStream};
use std::process::Command;

pub struct C2 {
    pub clients: Vec<Client>,
    pub commands: Vec<C2Command>
}

impl C2 {
    pub fn new(commands: Vec<C2Command>) -> Self {
        let clients = vec![];
        Self { clients, commands }
    }

    pub fn send_commands(&mut self) {
        for client in self.clients.iter_mut() {
            for command in self.commands.iter() {
                for item in command.item.iter() {
                    if let Err(e) = client.stream.write_all(item.as_bytes()) {
                        println!("Failed to send command {}", e);
                    } 
                }
            }
        }
    }

    pub fn add_command(&mut self, command: &C2Command) {
        self.commands.push(command.clone());
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.push(client);
    }
}

pub struct Client {
    pub stream: TcpStream, 
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
}

#[derive(Debug, Clone)]
pub struct C2Command {
    pub item: Vec<String>
}

impl C2Command {
    pub fn new(item: Vec<String>) -> Self {
        Self { item }
    }
}






