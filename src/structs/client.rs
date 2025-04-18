use core::fmt;
use std::{collections::HashMap, sync::Arc};

use tokio::{
    io::{self, AsyncReadExt},
    net::UdpSocket,
    select,
    sync::{RwLock, watch},
    task,
};

use super::{command, discovery::DiscoveryMessage, user::UserTrait};

pub struct Client {
    pub hosts: Arc<RwLock<HashMap<String, DiscoveryMessage>>>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            hosts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn search_for_hosts(&mut self, host: String, port: u16) {
        let socket = UdpSocket::bind(format!("{}:{}", host, port)).await.unwrap();
        let mut buf = [0; 1024];

        // Setup shutdown signal
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

        // Spawn task to read stdin and look for 'q'
        let quit_task = task::spawn(async move {
            let mut stdin = io::stdin();
            let mut input = [0u8; 2];

            loop {
                if let Ok(n) = stdin.read_exact(&mut input).await {
                    if n == 2 && input[0] == b'q' && input[1] == b'\n' {
                        let _ = shutdown_tx.send(true);
                        break;
                    }
                }
            }
        });

        // UDP Listening loop
        let udp_task = {
            let hosts = Arc::clone(&self.hosts);
            task::spawn(async move {
                loop {
                    select! {
                        _ = shutdown_rx.changed() => {
                            if *shutdown_rx.borrow() {
                                break;
                            }
                        }
                        result = socket.recv_from(&mut buf) => {
                            match result {
                                Ok((_, addr)) => {
                                    let s: String = format!("{}:{}", addr.ip(), addr.port());
                                    let mut hosts = hosts.write().await;
                                    if !hosts.contains_key(&s) {
                                        let dm = DiscoveryMessage::new(&addr);
                                        hosts.insert(s.clone(), dm);
                                        println!("> found host {}:{}", addr.ip(), addr.port());
                                    }
                                }
                                Err(e) => {
                                    eprintln!("UDP recv error: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
            })
        };

        println!("> Enter q then ENTER for exit discovering");
        println!("> Searching...");

        // Wait for either task to finish
        tokio::select! {
            _ = quit_task => {}
            _ = udp_task => {}
        }

        // Ok(())
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
