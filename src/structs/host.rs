use core::fmt;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    task,
    select,
    time::interval,
    sync::{Mutex, mpsc, watch},
    net::{TcpListener, TcpStream, UdpSocket},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

use crate::global::helper::quit_task_handler;

use super::{command::CommandType, user::UserTrait};

type HostClientMap = Arc<Mutex<HashMap<SocketAddr, mpsc::Sender<Message>>>>;

struct Message {
    producer: String,
    message: String,
}

pub struct Host {
    clients: HostClientMap,
}

impl Host {
    pub fn new() -> Host {
        Host {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start_chat(&self, host: &String, client_port: &u16, host_port: &u16) {
        let addr = format!("{}:{}", host, host_port);
        let listener = TcpListener::bind(addr).await.unwrap();

        // Setup shutdown signal
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

        // Spawn task to read stdin and look for 'q'
        let quit_task = quit_task_handler(shutdown_tx).await;

        // TCP start chat server
        let tcp_chat_server_task = {
            let client_port = client_port.clone();
            let client_map = self.clients.clone();
            task::spawn(async move {
                println!("Starting TCP server...");
                loop {
                    select! {
                        _ = shutdown_rx.changed() => {
                            if *shutdown_rx.borrow() {
                                break;
                            }
                        }
                        Ok((socket,addr)) = listener.accept() => {
                            // if addr.port() != client_port {
                            //     continue;
                            // }
                            println!("Client connected: {}", addr);
                            task::spawn(handle_client(socket, addr, client_map.clone()));
                        }
                    }
                }
            })
        };

        // Wait for either task to finish
        tokio::select! {
            _ = quit_task => {}
            _ = tcp_chat_server_task => {}
        }
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
        let quit_task = quit_task_handler(shutdown_tx).await;

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
    async fn execute_command(
        &mut self,
        cmd: &super::command::Command,
        host: &String,
        client_port: &u16,
        host_port: &u16,
    ) -> Result<(), String> {
        match &cmd.command_type {
            Some(ty) => match ty {
                CommandType::Start => {
                    self.start_chat(host, client_port, host_port).await;
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

async fn handle_client(mut socket: TcpStream, addr: SocketAddr, client_map: HostClientMap) {
    let sender;
    let mut receiver;
    {
        let mut clients = client_map.lock().await;
        let (tx, rx) = mpsc::channel::<Message>(10);
        clients.insert(addr, tx.clone());
        sender = tx;
        receiver = rx;
    }

    let (reader, mut writer) = socket.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        select! {
            // Read from socket
            read = buf_reader.read_line(&mut line) => {
                match read {
                    Ok(0) => {
                        let mut clients = client_map.lock().await;
                        clients.remove(&addr);
                        println!("Client disconnected");
                        break;
                    }
                    Ok(_) => {
                        println!("Received from client: {}", line.trim_end());
                        let clients = client_map.lock().await;

                        for (c_addr, tx) in clients.iter() {
                            if *c_addr != addr {
                                // skip sender if you want
                                let m = Message {
                                    producer: format!("{}:{}",addr.ip(), addr.port()),
                                    message: line.clone()
                                };
                                if let Err(e) = tx.send(m).await {
                                    eprintln!("Failed to send to {}: {:?}", addr, e);
                                }
                            }
                        }
                        line.clear(); // important to reuse buffer!
                    }
                    Err(e) => {
                        eprintln!("Read error: {:?}", e);
                        break;
                    }
                }
            }

            // Receive a message from another task
            msg = receiver.recv() => {
                match msg {
                    Some(text) => {
                        if let Err(e) = writer.write_all(text.message.as_bytes()).await {
                            eprintln!("Write error: {:?}", e);
                            break;
                        }
                    }
                    None => {
                        // All senders dropped
                        println!("Message channel closed");
                        break;
                    }
                }
            }
        }
    }
}
