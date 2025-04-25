use core::fmt;
use std::{collections::HashMap, sync::Arc};

use tokio::{
    net::UdpSocket,
    select,
    sync::{RwLock, watch},
    task,
};

use crate::global::helper::quit_task_handler;

use super::{
    command::{Command, CommandType},
    discovery::DiscoveryMessage,
    user::UserTrait,
};

pub struct Client {
    name: String,
    hosts: Arc<RwLock<HashMap<String, DiscoveryMessage>>>,
}

impl Client {
    pub fn new(name: String) -> Client {
        Client {
            name,
            hosts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_chat(&self) {
        println!("Starting chating...");
    }

    pub async fn search_for_hosts(&mut self, host: String, client_port: u16) {
        let socket = UdpSocket::bind(format!("{}:{}", host, client_port))
            .await
            .unwrap();
        let mut buf = [0; 1024];

        // Setup shutdown signal
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

        // Spawn task to read stdin and look for 'q'
        let quit_task = quit_task_handler(shutdown_tx).await;

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
    async fn execute_command(
        &mut self,
        cmd: &Command,
        _: &String,
        _: &u16,
        _: &u16,
    ) -> Result<(), String> {
        match &cmd.command_type {
            Some(ty) => match ty {
                CommandType::ListHosts => {
                    let hosts = self.hosts.read().await;
                    if hosts.is_empty() {
                        println!("No host found!")
                    } else {
                        let mut i = 1;
                        for (host, _) in hosts.iter() {
                            println!("[{}] {}", i, host);
                            i = i + 1;
                        }
                    }
                }
                CommandType::Start => {
                    self.start_chat().await;
                }
                _ => {
                    println!("Invalid command!")
                }
            },
            None => {
                println!("Invalid command!")
            }
        }
        Ok(())
    }
}
