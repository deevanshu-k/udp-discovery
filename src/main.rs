mod cmd;
mod global;
mod structs;

use clap::Parser;
use structs::user::User;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'n', long = "name")]
    name: String,
    #[arg(short = 'i', long = "ip")]
    host: String,
    #[arg(long = "host-port")]
    hport: u16,
    #[arg(long = "client-port")]
    cport: u16,
}
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let mut user: Option<User> = None;
    cmd::read_commands(&args.name, &args.host, &args.cport, &args.hport, &mut user).await;
}
