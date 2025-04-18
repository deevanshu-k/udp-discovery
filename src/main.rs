mod cmd;
mod global;
mod structs;

use clap::Parser;
use structs::user::User;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'i', long = "ip")]
    host: String,
    #[arg(short = 'p', long = "port")]
    port: u16,
}
fn main() {
    let args = Cli::parse();
    let mut user: Option<User> = None;
    cmd::read_commands(&args.host, &args.port,&mut user);
}
