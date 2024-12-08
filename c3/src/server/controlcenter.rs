use serde::de::value::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;

use std::{collections::HashMap, sync::{Arc}};
use uuid::Uuid;
use crate::log::LogHandler;

use super::{clients, Client, CommandEntry, C2Command};
use crate::log::*;

#[derive(Debug)]
pub struct C2 {
    pub clients: Arc<RwLock<Vec<Client>>>,
    pub max_clients: usize,
    pub command_queue: Vec<CommandEntry>,
}

impl C2 {
    pub fn new() -> Self {
        C2 {
            clients: Arc::new(RwLock::new(Vec::new())),
            max_clients: 100, // Default max clients
            command_queue: Vec::new(),
        }
    }

    pub async fn start_listener(&mut self, logstorage: &mut LogStorage) -> tokio::io::Result<()> {
        let server_address = "127.0.0.1:1414";
        let listener: TcpListener = TcpListener::bind(server_address).await?;
        logstorage.add_log(C2Log::new(LogLevel::Info, format!("Created new TcpListener on address: {server_address}")));
        
        let clients = self.clients.to_owned();
        let max_clients = self.max_clients;

        loop {
            match listener.accept().await {
                Ok((socket, client_address)) => {
                    logstorage.add_log(C2Log::new(LogLevel::Info, format!("Accepted incoming connection from client address: {}", client_address)));
                    self.save_client(Client::new( socket, "hostname".to_owned(), client_address.to_string()), logstorage).await;
                }
                Err(e) => {
                    logstorage.add_log(C2Log::new(LogLevel::Error, format!("Failed to accept incoming connection: {e}")));
                }
            }
        }
    }

    async fn save_client(&mut self, client: Client, logstorage: &mut LogStorage) {
        let mut clients = self.clients.write().await;

        if clients.len()+1 < self.max_clients {
            clients.push(client);
            logstorage.add_log(C2Log::new(LogLevel::Info, format!("Saved Client to Storage!")));
        } else {
            logstorage.add_log(C2Log::new(LogLevel::Error, format!("Number of clients has exeeded max client limit!")));
            return;
        }
    }

    pub fn add_command_to_queue(&mut self, command_entry: CommandEntry, logstorage: &mut LogStorage) {
        self.command_queue.push(command_entry);
        logstorage.add_log(C2Log::new(LogLevel::Info, format!("Added command to command queue!")));
    }

    pub fn iter_queue(&mut self) {
        for command_entry in self.command_queue.iter() {

        }
    }
}
