use core::fmt;

use super::user::UserTrait;

pub struct Host {}

impl Host {
    pub fn new() -> Host {
        println!("New host creater");
        Host {}
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Host")
    }
}

impl UserTrait for Host {
    fn execute_command(&mut self, c: &super::command::Command) -> Result<(), String> {
        Ok(())
    }
}
