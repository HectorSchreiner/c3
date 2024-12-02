use std::time::Instant;
use uuid::Uuid;

use super::Client;

#[derive(Debug)]
pub struct CommandEntry {
    pub clients: Vec<Client>,
    pub command: C2Command,
    pub timestamp: Instant,
    pub result: CommandResult
}

impl CommandEntry {
    pub fn new(clients: Vec<Client>, command: C2Command, timestamp: Instant, result: CommandResult) -> Self {
        Self { clients, command, timestamp, result }
    }

    pub fn execute(&mut self) {
        todo!()
    }
}

#[derive(Debug)]
pub enum CommandResult {
    Succeded,
    Failed(String),
    Pending,
}

#[derive(Debug)]
pub enum C2Command {
    ListClients,
    ExecuteRemote(String),
    Info(Uuid),
}