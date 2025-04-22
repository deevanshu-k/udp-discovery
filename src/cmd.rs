use crate::structs::{
    self,
    client::Client,
    command::CommandType::{BecomeClient, BecomeHost},
    host::Host,
    user::{User, UserTrait},
};
use colored::Colorize;
use std::io::{self, BufRead, Write};

pub async fn read_commands(
    host: &String,
    client_port: &u16,
    host_port: &u16,
    user: &mut Option<User>,
) {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);
    let mut reader = io::BufReader::new(stdin);

    let mut cmd_str = String::from("");
    update_prompt_str(
        &mut cmd_str,
        &host,
        client_port,
        host_port,
        String::from("Cli"),
    );

    loop {
        // Show prompt
        write!(&mut writer, "{}", cmd_str).unwrap();
        writer.flush().unwrap();

        // Read command
        let mut buf: Vec<u8> = Vec::new();
        reader
            .read_until(b'\n', &mut buf)
            .expect("Failed to read command");

        // Marshal the command
        let mut c = structs::command::new();
        if let Err(_e) = c.marshal(&String::from_utf8(buf).expect("Invalid UTF-8 sequence")) {
            continue;
        }

        // Create or switch user
        match c.command_type.as_ref().unwrap() {
            BecomeClient => {
                *user = Some(User::Client(Client::new()));
                update_prompt_str(
                    &mut cmd_str,
                    &host,
                    client_port,
                    host_port,
                    String::from("Client"),
                );
                if let User::Client(u) = user.as_mut().unwrap() {
                    u.search_for_hosts(host.clone(), client_port.clone()).await;
                }
                continue;
            }
            BecomeHost => {
                *user = Some(User::Host(Host::new()));
                update_prompt_str(
                    &mut cmd_str,
                    &host,
                    client_port,
                    host_port,
                    String::from("Host"),
                );
                if let User::Host(u) = user.as_mut().unwrap() {
                    u.broadcast_discovery_message(
                        host.clone(),
                        client_port.clone(),
                        host_port.clone(),
                    )
                    .await;
                }
                continue;
            }
            _ => {}
        }

        // Init user if not present
        if user.is_none() {
            println!("Select user type, command");
            println!("BECOME");
            println!("{:>10} -> for becomming host", "HOST");
            println!("{:>10} -> for becomming client", "CLIENT");
            continue;
        }

        // Execute command
        if let Some(u) = user {
            match u {
                User::Client(cl) => cl
                    .execute_command(&c, host, client_port, host_port)
                    .await
                    .unwrap(),
                User::Host(ho) => ho
                    .execute_command(&c, host, client_port, host_port)
                    .await
                    .unwrap(),
            }
        }
    }
}

fn update_prompt_str(
    s: &mut String,
    host: &String,
    client_port: &u16,
    host_port: &u16,
    client_type: String,
) {
    *s = format!(
        "{}:{}->{}[{}]$ ",
        host.green().bold(),
        client_port.to_string().green().bold(),
        host_port.to_string().green().bold(),
        client_type
    );
}
