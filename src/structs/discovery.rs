use std::net::SocketAddr;

#[derive(Hash ,PartialEq, Eq, Debug)]
pub struct DiscoveryMessage {
    pub ip: String,
    pub port: u16,
}

impl DiscoveryMessage {
    pub fn new(addr: &SocketAddr) -> DiscoveryMessage {
        DiscoveryMessage {
            ip: addr.ip().to_string(),
            port: addr.port(),
        }
    }
}
