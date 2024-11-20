use std::time::Instant;
use uuid::Uuid;

#[derive(Debug)]
pub struct CommandEntry {
    pub client_id: Uuid,
    pub command: C2Command,
    pub timestamp: Instant,
    pub result: CommandResult
}

impl CommandEntry {
    pub fn new(client_id: Uuid, command: C2Command, timestamp: Instant, result: CommandResult) -> Self {
        Self { client_id, command, timestamp, result }
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