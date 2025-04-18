use super::{client::Client, command::Command, host::Host};

pub enum User {
    Client(Client),
    Host(Host),
}
pub trait UserTrait {
    fn execute_command(&mut self, c: &Command) -> Result<(), String>;
}
