use clap::Parser;                              
use anyhow::Result;
use log::{info, warn, error};
use bark::{Bark, BarkConfig};
use brk::Chain;

    
    
#[derive(Parser)]
#[clap(name = "my_cli", about = "An example CLI application")]
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

