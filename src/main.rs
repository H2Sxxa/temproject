use clap::Parser;
use cmd::Cli;

mod cmd;
mod typing;

#[tokio::main]
async fn main() {
    let _arg = Cli::parse();
}
