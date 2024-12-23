use serde::de::value::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

use time::{OffsetDateTime, PrimitiveDateTime};

use crate::log::LogHandler;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use super::{clients, C2Command, Client, CommandEntry};
use crate::interface::Interface;
use crate::log::*;

#[derive(Debug)]
pub struct C2 {
    pub clients: Arc<RwLock<HashMap<Client, TcpStream>>>,
    pub max_clients: usize,
    command_queue: Arc<RwLock<Vec<CommandEntry>>>,
    command_history: Arc<RwLock<Vec<CommandEntry>>>,
}

impl C2 {
    pub fn new() -> Self {
        C2 {
            clients: Arc::new(RwLock::new(HashMap::new())),
            max_clients: 100, // Default max clients
            command_queue: Arc::new(RwLock::new(Vec::new())),
            command_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start_listener(&mut self, logstorage: &mut LogStorage) -> tokio::io::Result<()> {
        let server_address = "127.0.0.1:1414";
        let listener: TcpListener = TcpListener::bind(server_address).await?;
        logstorage.add_log(C2Log::new(
            LogLevel::Info,
            format!("Created new TcpListener on address: {server_address}"),
        ));

        let clients = self.clients.to_owned();
        let max_clients = self.max_clients;

        loop {
            match listener.accept().await {
                Ok((mut socket, client_address)) => {
                    // check if theres a duplicate client
                    if self.clients.read().await.keys().find(|client| client.ip_address == client_address.to_string()).is_none() {
                        logstorage.add_log(C2Log::new(
                            LogLevel::Info,
                            format!(
                                "Accepted incoming connection from client address: {}",
                                client_address
                            ),
                        ));
                        self.save_client(
                            Client::new("hostname".to_owned(), client_address.to_string()),
                            socket,
                            logstorage,
                        )
                        .await;
                    // if no duplicate client
                    } else {
                        logstorage.add_log(C2Log::new(
                            LogLevel::Error,
                            format!(
                                "Duplicate client detected from address: {}, connection rejected.",
                                client_address
                            ),
                        ));
                        let _ = socket.shutdown().await;
                    }
                }
                Err(e) => {
                    logstorage.add_log(C2Log::new(
                        LogLevel::Error,
                        format!("Failed to accept incoming connection: {e}"),
                    ));
                }
            }
            
        }
    }

    async fn save_client(
        &mut self,
        client: Client,
        stream: TcpStream,
        logstorage: &mut LogStorage,
    ) {
        let mut clients = self.clients.write().await;

        if clients.len() + 1 < self.max_clients {
            clients.insert(client, stream);
            logstorage.add_log(C2Log::new(
                LogLevel::Info,
                format!("Saved Client to Storage!"),
            ));
        } else {
            logstorage.add_log(C2Log::new(
                LogLevel::Error,
                format!("Number of clients has exeeded max client limit!"),
            ));
            return;
        }
    }

    pub async fn add_command_to_queue(
        &mut self,
        command_entry: CommandEntry,
        logstorage: &mut LogStorage,
    ) {
        self.command_queue.write().await.push(command_entry);
        logstorage.add_log(C2Log::new(
            LogLevel::Info,
            format!("Added command to command queue!"),
        ));
    }

    pub async fn add_command_to_history(
        &mut self,
        command_entry: CommandEntry,
        logstorage: &mut LogStorage,
    ) {
        self.command_history.write().await.push(command_entry);
        logstorage.add_log(C2Log::new(
            LogLevel::Info,
            format!("Added command to command queue!"),
        ));
    }

    async fn get_client_from_uuid(&self, uuid: Uuid) -> Option<Client> {
        self.clients
            .read()
            .await
            .keys()
            .find(|client| client.id == uuid)
            .map(|client| client.clone())
    }

    async fn get_command_queue(&self) -> Vec<CommandEntry> {
        self.command_queue.read().await.to_vec()
    }

    async fn remove_command_from_queue(&self, id: Uuid) {
        self.command_queue
            .write()
            .await
            .retain(|client| client.id != id);
    }

    async fn send_command_entry(&self, command_data: String, clients: Vec<Client>, logstorage: &mut LogStorage) -> Result<String, std::io::Error> {
        let clients = &clients;
        
        let command_data = match command_data {
            Ok(data) => data,
            Err(error) => {
                command_entry.result = CommandResult::Failed(format!("Serialization error: {}", error));
                logstorage.add_log(C2Log::new(
                    LogLevel::Error,
                    format!("Failed to serialize command: {}", error),
                ));
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Serialization failed",
                ));
            }
        }
        Ok(())
    }

    async fn execute_command(
        command_entry: &CommandEntry,
        logstorage: &mut LogStorage,
    ) -> Result<(), std::io::Error> {
        let command = &command_entry.command;
        let clients = &command_entry.clients;

        clients.iter().for_each(|client| match &command {
            C2Command::ListClients => {
                logstorage.add_log(C2Log::new(
                    LogLevel::Info,
                    format!("Client {} is {:?}", client.hostname, client.status),
                ));
            }
            _ => {
                logstorage.add_log(C2Log::new(LogLevel::Error, format!("Command not found!")));
            }
        });

        Ok(())
    }

    async fn run_command_queue(
        &mut self,
        logstorage: &mut LogStorage,
        interface: &mut Interface,
    ) -> Result<(), std::io::Error> {
        if let Some(command) = self.command_queue.read().await.first() {
            Self::execute_command(command, logstorage).await?;
            Self::remove_command_from_queue(&self, command.id).await;
        }
        Ok(())
    }
}
