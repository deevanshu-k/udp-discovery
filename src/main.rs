mod cmd;
mod global;
mod structs;

use clap::Parser;

#[derive(Parser)]
struct  Cli {
    #[arg(short = 'i', long = "ip")]
    host: String,
    #[arg(short = 'p', long = "port")]
    port: u16,
}
fn main() {
    let args = Cli::parse();
    cmd::read_commands(&args.host, &args.port);
}
