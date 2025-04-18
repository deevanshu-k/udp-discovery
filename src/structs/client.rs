use core::fmt;
use std::collections::HashSet;

use super::user::UserTrait;

pub struct Client {
    pub hosts: HashSet<String>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            hosts: HashSet::new(),
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Client")
    }
}

impl UserTrait for Client {
    fn execute_command(&mut self, c: &super::command::Command) -> Result<(), String> {
        println!("Executing client cmd");
        Ok(())
    }
}
