use super::{client::Client, command::Command, host::Host};

pub enum User {
    Client(Client),
    Host(Host),
}
pub trait UserTrait {
    async fn execute_command(
        &mut self,
        c: &Command,
        h: &String,
        cp: &u16,
        hp: &u16,
    ) -> Result<(), String>;
}
