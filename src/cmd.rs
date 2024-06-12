use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "temproject")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    /// Init a project from a local template
    New { name: String },

    /// List all available local templates
    List,
    #[command(arg_required_else_help = true)]
    /// Add a new template to local
    Add { name: String },

    #[command(arg_required_else_help = true)]
    /// Remove a new template from local
    Remove { name: String },

    #[command(arg_required_else_help = true)]
    /// Access the templates from Internet
    Net {
        #[command(subcommand)]
        command: NetCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum NetCommands {
    #[command(arg_required_else_help = true)]
    /// Init a project from a local template
    New { name: String },

    /// List all available local templates
    List,
    #[command(arg_required_else_help = true)]
    /// Add a new template to local
    Add { name: String },

    #[command(arg_required_else_help = true)]
    /// Remove a new template from local
    Remove { name: String },
}
