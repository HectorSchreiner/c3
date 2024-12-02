use std::{collections::HashMap, sync::{Arc, Mutex}};

use uuid::Uuid;

use super::{clients, Client, CommandEntry};

pub struct C2 {
    pub clients: Arc<Mutex<Vec<Client>>>,
    pub max_clients: usize,
    pub command_queue: Vec<CommandEntry>,
}

impl C2 {
    pub fn new() -> Self {
        C2 {
            clients: Arc::new(Mutex::new(Vec::new())),
            max_clients: 100, // Default max clients
            command_queue: Vec::new(),
        }
    }

    pub fn add_client(&mut self, client: Client) -> Result<(), ()>{
        if self.clients.lock().unwrap().len() < self.max_clients {
            self.clients.lock().unwrap().push(client);
        }
        Ok(())
    }

    pub fn add_command_to_queue(&mut self, command_entry: CommandEntry) {
        self.command_queue.push(command_entry);
    }

    
}
