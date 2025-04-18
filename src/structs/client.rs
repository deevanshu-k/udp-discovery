use core::fmt;
use std::collections::HashSet;

use super::user::UserTrait;

pub struct Client {
    pub hosts: HashSet<String>,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Client")
    }
}

impl Client {
    pub fn new() -> Client {
        println!("New client creater");
        Client {
            hosts: HashSet::new(),
        }
    }
}

impl UserTrait for Client {
    fn execute_command(&mut self, c: &super::command::Command) -> Result<(), String> {
        Ok(())
    }
}
