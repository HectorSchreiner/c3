use c3::interface::Interface;
use c3::log::{C2Log, LogHandler, LogLevel, LogStorage};

mod interface;
mod log;
mod server;
mod utils;

pub fn main() {
    let mut storage = LogStorage::new();

    Interface::init(&mut storage).map_err(|err| {
        storage.add_log(C2Log::new(
            LogLevel::Error,
            format!("Failure with tui: {}", err),
        ));
    });

    for log in storage.logs.iter() {
        LogHandler::print_log(log);
    }
}
