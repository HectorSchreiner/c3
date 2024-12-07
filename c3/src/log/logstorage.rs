use std::fmt;
use time::{OffsetDateTime, PrimitiveDateTime};

pub struct LogStorage {
    pub logs: Vec<C2Log>,
}

impl LogStorage {
    pub fn new(logs: Vec<C2Log>) -> Self {
        Self { logs }
    }

    pub fn add_log(&mut self, log: C2Log) {
        self.logs.push(log);
    }
}

pub struct C2Log {
    pub timestamp: PrimitiveDateTime,
    pub level: LogLevel,
    pub message: String
}

impl C2Log {
    pub fn new(level: LogLevel, message: String) -> Self {
        let now = OffsetDateTime::now_utc(); // Get current UTC time
        let date = now.date();
        let time = now.time();
        let timestamp = PrimitiveDateTime::new(date, time);

        Self {
            timestamp,
            level,
            message,
        }
    }
}

impl From<T> for C2Log {
    pub fn from(value: String) -> Self {
        
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Critical
}

