use clap::Parser;
use cmd::{Cli, Commands, NetCommands};

mod cmd;
mod template;

#[tokio::main]
async fn main() {
    let arg = Cli::parse();
    match arg.command {
        Commands::New { name } => todo!(),
        Commands::List => todo!(),
        Commands::Add { name } => todo!(),
        Commands::Remove { name } => todo!(),
        Commands::Net { command } => match command {
            NetCommands::New { name } => todo!(),
            NetCommands::List => todo!(),
            NetCommands::Add { name } => todo!(),
            NetCommands::Remove { name } => todo!(),
        },
    }
}
