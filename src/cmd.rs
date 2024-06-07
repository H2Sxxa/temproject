use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "temproject")]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    /// Create a project from a template
    New { name: String },

    /// List all available templates
    List,
    #[command(arg_required_else_help = true)]
    /// Add a new template
    Add { name: String },

    #[command(arg_required_else_help = true)]
    /// Remove a new template
    Remove { name: String },
}
