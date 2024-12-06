use std::time::Instant;
use uuid::Uuid;
use time::Time;
use crate::log::*;

use super::Client;

#[derive(Debug)]
pub struct CommandEntry {
    pub clients: Vec<Client>,
    pub command: C2Command,
    pub timestamp: Time,
    pub result: CommandResult
}

impl CommandEntry {
    pub fn new(clients: Vec<Client>, command: C2Command, timestamp: Time, result: CommandResult) -> Self {

        Self { clients, command, timestamp, result }
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