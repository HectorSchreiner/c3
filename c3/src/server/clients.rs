use std::net::TcpStream;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    pub stream: TcpStream,
    pub hostname: String,
    pub ip_address: String,
    pub last_seen: Instant,
    pub status: ClientStatus,
}

#[derive(Debug, PartialEq)]
pub enum ClientStatus {
    Connected,
    Disconnected,
    Idle,
    Busy,
}