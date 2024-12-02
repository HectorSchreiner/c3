use std::fmt;

use time::Time;

pub struct LogStorage {
    pub logs: Vec<Log>,
}

pub struct Log {
    pub timestamp: Time,
    pub category: LogCategory
}

impl Log {
    pub fn string_output(&mut self) -> String {
        format!("Date: {}, Category: {}", self.timestamp.to_string(), self.category)
    }

    pub fn get_date() -> String {
        todo!()
    }

    pub fn get_title() -> String {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LogCategory {
    Command,
    Client,
}

impl fmt::Display for LogCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command => write!(f, "Command"),
            Self::Client => write!(f, "Client"),
        }
    }
}

