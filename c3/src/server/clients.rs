use std::net::TcpStream;
use std::time::Instant;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    pub stream: TcpStream,
    pub hostname: String,
    pub ip_address: String,
    pub last_seen: PrimitiveDateTime,
    pub status: ClientStatus,
}

impl Client {
    pub fn new(
        id: Uuid,
        stream: TcpStream,
        hostname: String,
        ip_address: String,
    ) -> Self 
    {
        let now = OffsetDateTime::now_utc();
        let date = now.date();
        let time = now.time();
        let last_seen = PrimitiveDateTime::new(date, time);
        let status = ClientStatus::Connected;
        Self { id, stream, hostname, ip_address, last_seen, status }
    }
}

#[derive(Debug, PartialEq)]
pub enum ClientStatus {
    Connected,
    Disconnected,
    Idle,
    Busy,
}