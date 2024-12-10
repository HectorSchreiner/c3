use crate::log::*;
use std::time::Instant;
use time::PrimitiveDateTime;
use uuid::Uuid;

use super::Client;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CommandEntry {
    pub id: Uuid,
    pub clients: Vec<Client>,
    pub command: C2Command,
    pub timestamp: PrimitiveDateTime,
    pub result: CommandResult,
}

impl CommandEntry {
    pub fn new(
        clients: Vec<Client>,
        command: C2Command,
        timestamp: PrimitiveDateTime,
        result: CommandResult,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            clients,
            command,
            timestamp: timestamp,
            result,
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        match self.command {
            C2Command::ListClients => {
                Ok(());
            }
            _ => {
                Ok(());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CommandResult {
    Succeded,
    Failed(String),
    Pending,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum C2Command {
    ListClients,
    ExecuteRemote(String),
    Info(Uuid),
}
