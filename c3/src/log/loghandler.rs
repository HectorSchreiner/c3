use std::fmt;
use super::{C2Log, LogLevel};
use time::{error::Format, Time};

use crossterm::{style::{Color, Print, ResetColor, SetForegroundColor}, ExecutableCommand};
use std::io::{stdout, Write};
use time::{OffsetDateTime, PrimitiveDateTime};

use super::LogStorage;

pub struct LogHandler;

impl LogHandler {
    pub fn print_log(log: &C2Log) {
        let mut stdout = stdout();

        let color = match log.level {
            LogLevel::Info => Color::Green,
            LogLevel::Warning => Color::Yellow,
            LogLevel::Error => Color::Red,
            LogLevel::Critical => Color::Magenta,
        };

        // Print log
        stdout
            .execute(SetForegroundColor(color))
            .unwrap()
            .execute(Print(format!(
                "[{}] [{:?}] {}\n",
                log.timestamp, log.level, log.message
            )))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
    }
}