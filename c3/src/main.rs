use c3::log::{C2Log, LogHandler, LogLevel, LogStorage};

mod server;
mod utils;
mod log;
mod interface;

pub fn main() {
    let mut storage = LogStorage::new(vec![
        C2Log::new(LogLevel::Info, "This is an Informational Log".to_string()),
        C2Log::new(LogLevel::Info, "another one".to_string()),
        C2Log::new(LogLevel::Critical, "Woop Woop Very bad".to_string()),
    ]
    );

    let log_handler = LogHandler;

    for log in storage.logs.iter() {
        LogHandler::print_log(log);
    }
}

