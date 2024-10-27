use std::env;
use std::error::Error;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(required = true)]
        id: u32,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::new()
        .filter_level(
            env::var("RUST_LOG")
                .unwrap_or(String::from("info"))
                .parse()
                .unwrap(),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { id }) => lib::new::new(id)?,

        _ => {}
    }

    Ok(())
}
