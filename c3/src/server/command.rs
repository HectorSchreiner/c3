use crate::log::*;
use std::time::Instant;
use time::Time;
use uuid::Uuid;

use super::Client;

#[derive(Debug)]
pub struct CommandEntry {
    pub id: Uuid,
    pub clients: Vec<Client>,
    pub command: C2Command,
    pub timestamp: Time,
    pub result: CommandResult,
}

impl CommandEntry {
    pub fn new(
        id: Uuid,
        clients: Vec<Client>,
        command: C2Command,
        timestamp: Time,
        result: CommandResult,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            clients,
            command,
            timestamp,
            result,
        }
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
