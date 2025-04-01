use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = crate::APP_NAME)]
#[command(version = crate::APP_VERSION)]
#[command(about = crate::APP_DESCRIPTION, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a file
    #[command(visible_aliases = &["r"])] // Short alias for "run"
    Run {
        /// Path to the file (positional argument)
        input_file: String,
    },
}