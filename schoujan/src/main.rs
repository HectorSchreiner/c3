use std::{
    net::{TcpListener, TcpStream},
    thread,
    sync::{Arc, Mutex}
};

mod server;
mod interface;
mod commands;

use interface::{Interface, Menu, Item};
use crate::server::*;

fn main() -> std::io::Result<()> {
    let mut interface = Interface::new();
    let test_command = C2Command::new(vec!["whoami".to_string()]);

    let mut menu = Menu::new("Command and Control Center".to_string());
    menu.add_item(&Item::new("Send Whoami to clients".to_string(), test_command.clone()));
    
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let c2 = Arc::new(Mutex::new(C2::new(vec![test_command])));

    let c2_clone = Arc::clone(&c2);
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Add client to C2
                    let mut c2 = c2_clone.lock().unwrap();
                    c2.add_client(Client::new(stream));
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }        
        }
    });

    loop {
        let c2_lock = Arc::clone(&c2);
        let mut c2 = c2_lock.lock().unwrap();

        interface.display_menu(&menu)?;
        interface.handle_input(&menu, &mut c2)?;
    }
}
