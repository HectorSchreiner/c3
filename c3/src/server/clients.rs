use std::time::Instant;
use time::{OffsetDateTime, PrimitiveDateTime};
use tokio::net::TcpStream;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Client {
    pub id: Uuid,
    pub hostname: String,
    pub ip_address: String,
    pub last_seen: PrimitiveDateTime,
    pub status: ClientStatus,
}

impl Client {
    pub fn new(hostname: String, ip_address: String) -> Self {
        let last_seen = PrimitiveDateTime::new(
            OffsetDateTime::now_utc().date(),
            OffsetDateTime::now_utc().time(),
        );

        let status = ClientStatus::Connected;

        Self {
            id: Uuid::new_v4(),
            hostname,
            ip_address,
            last_seen,
            status,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ClientStatus {
    Connected,
    Disconnected,
    Idle,
    Busy,
}
