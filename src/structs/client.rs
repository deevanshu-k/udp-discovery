use core::fmt;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpSocket, UdpSocket},
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

    pub async fn start_chat(&self, host: [u8; 4], client_port: u16, host_port: u16) {
        let client_addr: SocketAddr =
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), client_port);
        let host_addr: SocketAddr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(host[0], host[1], host[2], host[3])),
            host_port,
        );
        let socket = TcpSocket::new_v4().unwrap();
        socket.bind(client_addr).unwrap();
        let stream = match socket.connect(host_addr).await {
            Ok(s) => s,
            Err(e) => {
                println!(
                    "Failed in connecting to host {}.{}.{}.{}:{host_port}, Error: {e}",
                    host[0], host[1], host[2], host[3]
                );
                return;
            }
        };

        let (readstream, _writestream) = stream.into_split();
        let mut buf_reader = BufReader::new(readstream);

        // Setup shutdown signal
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

        // Spawn task to read stdin and look for 'q'
        let quit_task = quit_task_handler(shutdown_tx).await;

        let tcp_socket = {
            let _name = self.name.clone();
            let mut line = String::new();
            task::spawn(async move {
                loop {
                    select! {
                        _ = shutdown_rx.changed() => {
                            if *shutdown_rx.borrow() {
                                break;
                            }
                        }
                        read = buf_reader.read_line(&mut line) => {
                            match read {
                                Ok(0) => {
                                    println!("Host disconnected!");
                                    break;
                                }
                                Ok(_) => {
                                    line = line.trim().to_string();
                                    println!("> {}",line);
                                    line.clear();
                                }
                                Err(e) => {
                                    eprintln!("Read error: {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
            })
        };

        println!("> Enter q then ENTER for exit discovering");

        // Wait for either task to finish
        tokio::select! {
            _ = quit_task => {}
            _ = tcp_socket => {}
        }
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
        _host: &String,
        client_port: &u16,
        host_port: &u16,
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
                    if cmd.args.len() == 0 {
                        println!("Host server address required => START <ip>")
                    } else {
                        let ip: Vec<u8> = cmd.args[0]
                            .clone()
                            .split('.')
                            .filter_map(|s| s.parse::<u8>().ok())
                            .collect();
                        if ip.len() != 4 {
                            println!("Wrong ip address provided");
                        }
                        self.start_chat(
                            [ip[0], ip[1], ip[2], ip[3]],
                            client_port.clone(),
                            host_port.clone(),
                        )
                        .await;
                    }
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
