#[derive(Debug)]
pub enum CommandType {
    Help,         // For listing all commands
    Exit,         // Exit the program
    Clear,        // Clear the screen
    BecomeHost,   // Start sending UDP discovery packets
    BecomeClient, // Bind to default port for host discovery, and save host details in memory
    ListHosts,    // List all hosts in memory
    Start,        // Start TCP server for accepting connection from client
    Connect,      // Connect to a host
    Disconnect,   // Disconnect from a host
    _Send,         // Start message sending session
    _Receive,      // Start message receiving session
}

#[derive(Debug)]
pub struct Command {
    pub command_type: Option<CommandType>,
    pub _args: Vec<String>,
}

impl Command {
    pub fn marshal(&mut self, s: &String) -> Result<(), String> {
        let args = s.split_whitespace().collect::<Vec<&str>>();
        if args.is_empty() {
            return Err(String::from("No command provided"));
        }

        self.command_type = match args[0].to_uppercase().as_str() {
            "HELP" => Some(CommandType::Help),
            "EXIT" => Some(CommandType::Exit),
            "CLEAR" => Some(CommandType::Clear),
            "BECOME" => {
                if args.len() < 2 {
                    Some(CommandType::Help)
                } else {
                    match args[1].to_uppercase().as_str() {
                        "HOST" => Some(CommandType::BecomeHost),
                        "CLIENT" => Some(CommandType::BecomeClient),
                        _ => Some(CommandType::Help),
                    }
                }
            }
            "LIST" => {
                if args.len() < 2 {
                    Some(CommandType::Help)
                } else {
                    match args[1].to_uppercase().as_str() {
                        "HOSTS" => Some(CommandType::ListHosts),
                        _ => Some(CommandType::Help),
                    }
                }
            }
            "CONNECT" => Some(CommandType::Connect),
            "DISCONNECT" => Some(CommandType::Disconnect),
            "START" => {
                if args.len() < 2 {
                    Some(CommandType::Help)
                } else {
                    Some(CommandType::Start)
                }
            }
            _ => Some(CommandType::Help),
        };

        Ok(())
    }
}

pub fn new() -> Command {
    Command {
        command_type: None,
        _args: Vec::new(),
    }
}
