use std::{collections::HashMap, sync::{Arc, Mutex}};

use uuid::Uuid;

use super::{Client, CommandEntry};

pub struct C2 {
    pub clients: Arc<Mutex<HashMap<Uuid, Client>>>,
    pub max_clients: usize,
    pub command_history: Vec<CommandEntry>,
}
