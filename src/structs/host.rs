use core::fmt;

use super::user::UserTrait;

pub struct Host {}

impl Host {
    pub fn new() -> Host {
        Host {}
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
