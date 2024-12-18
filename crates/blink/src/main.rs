use anyhow::Result;
use clap::Parser;
use cli::CLI;
use config::BlinkConfig;
use state::BlinkState;
use utils::init_logging;

mod cli;

fn main() -> Result<()> {
    init_logging()?;

    let cli = CLI::parse();

    match &cli.commands {
        Some(cli::Commands::Test) => println!("gotcha bitch"),
        None => {
            let config = BlinkConfig::load()?;

            let mut blink_state = BlinkState::new(config)?;
            blink_state.run()?;
        }
    }

    Ok(())
}
