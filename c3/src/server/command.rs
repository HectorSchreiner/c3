use std::time::Instant;
use uuid::Uuid;

#[derive(Debug)]
pub struct CommandEntry {
    pub client_id: Uuid,
    pub command: C2Command,
    pub timestamp: Instant,
    pub result: CommandResult
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