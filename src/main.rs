

use clap::{Parser, Subcommand};
use log::info;
use anyhow::Result; // Ensure anyhow is in your dependencies


/// Ark CLI Wallet: A CLI wallet for managing transactions and wallets.
#[derive(Parser)]
#[clap(
    name = "Ark CLI Wallet",
    about = "Ark wallet CLI written in Rust",
    version = "1.0",
    author = "Ark wallet"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Executes Command1 functionality.
    Command1,
    /// Executes Command2 with an argument.
    Command2 { argument: String },
}

fn main() -> Result<()> {
    // Initialize environment logger
    env_logger::init();

    // Parse CLI commands
    let cli = Cli::parse();

    // Create a client (example URL for demonstration)
    let client = Client::new("http://localhost:4003");

    // Match user commands and handle accordingly
    match &cli.command {
        Commands::Command1 => {
            info!("Executing Command1...");
            // Placeholder for real Command1 logic
            execute_command1(&client)?;
        }
        Commands::Command2 { argument } => {
            info!("Executing Command2 with argument: {}", argument);
            // Placeholder for real Command2 logic
            execute_command2(&client, argument)?;
        }
    }

    Ok(())
}

fn execute_command1(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    info!("Client URL: {}", client.url());
    // Implement Command1 functionality here (example placeholder)
    println!("Command1 logic executed successfully!");
    Ok(())
}
fn execute_command2(client: &Client, argument: &String) -> Result<()> {
    info!("Client URL: {}", client.url());
    info!("Argument passed: {}", argument);
    // Implement Command2 functionality here (example placeholder)
    println!("Command2 executed with argument: {}", argument);
    Ok(())
}