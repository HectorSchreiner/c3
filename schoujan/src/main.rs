use std::{
    net::{TcpListener, TcpStream},
    thread,
    sync::{Arc, Mutex}
};

mod server;
mod interface;
mod commands;

use interface::{Interface};
use crate::server::*;

fn main() -> std::io::Result<()> {
    let mut interface = Interface::new();
    let test_command = C2Command::new(vec!["whoami".to_string()]);
    
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let c2 = Arc::new(Mutex::new(C2::new(vec![test_command])));
    interface.display_menu()?;
    println!("escaped");

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
    }

}
