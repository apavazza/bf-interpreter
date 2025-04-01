use clap::Parser;

mod cli;
mod interpreter;
mod file;

// Constants for the application
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> std::io::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Run { input_file } => {
            let mut contents = file::load(&input_file).unwrap();
            let reader = file::convert(&mut contents).unwrap();
            interpreter::run(&reader).unwrap();
        }
    }

    Ok(())
}