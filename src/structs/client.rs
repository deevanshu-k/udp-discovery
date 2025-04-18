use core::fmt;
use std::collections::HashSet;

use tokio::{
    io::{self, AsyncReadExt},
    net::UdpSocket,
    select,
    sync::watch,
    task,
};

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
                        println!("Quit command received.");
                        let _ = shutdown_tx.send(true);
                        break;
                    }
                }
            }
        });

        // UDP Listening loop
        let udp_task = {
            task::spawn(async move {
                loop {
                    select! {
                        _ = shutdown_rx.changed() => {
                            if *shutdown_rx.borrow() {
                                println!("UDP task: Shutdown signal received.");
                                break;
                            }
                        }
                        result = socket.recv_from(&mut buf) => {
                            match result {
                                Ok((len, src)) => {
                                    let msg = String::from_utf8_lossy(&buf[..len]);
                                    println!("Received from {}: {}", src, msg);
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

        // Wait for either task to finish
        tokio::select! {
            _ = quit_task => {
                println!("Exiting due to user input.");
            }
            _ = udp_task => {
                println!("UDP task ended.");
            }
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
