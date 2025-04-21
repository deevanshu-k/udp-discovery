use core::fmt;
use std::{net::SocketAddr, time::Duration};

use tokio::{
    io::{self, AsyncReadExt},
    net::UdpSocket,
    select,
    sync::watch,
    task,
    time::interval,
};

use super::user::UserTrait;

pub struct Host {}

impl Host {
    pub fn new() -> Host {
        Host {}
    }

    pub async fn broadcast_discovery_message(
        &mut self,
        host: String,
        client_port: u16,
        host_port: u16,
    ) {
        let socket = UdpSocket::bind(format!("{}:{}", host, host_port))
            .await
            .unwrap();

        // Enable broadcast mode
        socket.set_broadcast(true).unwrap();

        // Setup shutdown signal
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

        // Bradcasting address
        let target_addr: SocketAddr = format!("255.255.255.255:{}", client_port).parse().unwrap();

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

        // UDP host discovery messages
        let udp_task = task::spawn(async move {
            let mut ticker = interval(Duration::from_secs(1));
            loop {
                select! {
                    _ = shutdown_rx.changed() => {
                        if *shutdown_rx.borrow() {
                            break;
                        }
                    }
                    _ = ticker.tick() => {
                        let msg: String = format!("Hello from cli broadcaster!");
                        match socket.send_to(msg.as_bytes(), &target_addr).await {
                            Ok(_) => {},
                            Err(e) => {
                                eprintln!("Failed to send: {}", e);
                                break;
                            },
                        }
                    }
                }
            }
        });

        println!("> Enter q then ENTER for exit discovering");
        println!("> Sending...");

        // Wait for either task to finish
        tokio::select! {
            _ = quit_task => {}
            _ = udp_task => {}
        }

        // Ok(())
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Host")
    }
}

impl UserTrait for Host {
    fn execute_command(&mut self, _c: &super::command::Command) -> Result<(), String> {
        println!("Executing host cmd");
        Ok(())
    }
}
