use clap::Parser;                              
use anyhow::Result;
use log::info;

use ark_lib;    
use ark_lib::address::Address;
use ark_lib::multisig::Multisig;
use ark_lib::transaction::Transaction;
use ark_lib::Wallet;
use bark_client::client::Client;
#[derive(Parser)]
#[clap(name = "ark_cli_wallet", about = "Your Ark wallet CLI written in Rust")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    
}

#[derive(Parser)]
enum Commands {
    Command1,
    Command2 { argument: String },
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Command1 => {
            info!("Running Command1");
            // Implement Command1 logic here
        }
        Commands::Command2 { argument } => {
            info!("Running Command2 with argument: {}", argument);
            // Implement Command2 logic here
        }
    }

    Ok(())
}

