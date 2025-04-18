use core::fmt;
use std::{collections::HashSet, net::UdpSocket};

use super::{command, discovery::DiscoveryMessage, user::UserTrait};

pub struct Client {
    pub hosts: HashSet<DiscoveryMessage>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            hosts: HashSet::new(),
        }
    }

    pub async fn search_for_hosts(&mut self, host: String, port: u16) {
        let socket = UdpSocket::bind(format!("{}:{}", host, port)).expect("could'nt bind to port");
        let mut buf = [0; 1024];
        loop {
            let (len, addr) = socket.recv_from(&mut buf).unwrap();
            let _ = String::from_utf8(buf[..len].to_vec())
                .unwrap_or_else(|_| String::from("<invalid UTF-8>"));
            let discovery_message = DiscoveryMessage::new(&addr);

            self.hosts.insert(discovery_message);
            println!("Discovered Hosts: {:?}", self.hosts);
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Client")
    }
}

impl UserTrait for Client {
    fn execute_command(&mut self, _: &command::Command) -> Result<(), String> {
        println!("Executing client cmd");
        Ok(())
    }
}
