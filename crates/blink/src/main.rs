// TODO: Add Clap for the cli of this project.
use anyhow::Result;
use clap::Parser;
use cli::CLI;
use config::BlinkConfig;
use state::BlinkState;
use utils::init_logging;

use networks::protocols::http::HttpClient;
use networks::NetworkManager;

mod cli;

// Mock only
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MyRequest {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    let cli = CLI::parse();

    match &cli.commands {
        Some(cli::Commands::Test) => {
            println!("gotcha bitch");
        }
        None => {
            let config = BlinkConfig::load()?;

            let mut blink_state = BlinkState::new(config)?;
            blink_state.run()?;
        }
    }

    // WARNING
    // TEMP Mock
    let mut client = HttpClient::new("http://example.com".to_string());

    // Initialize the client (no-op in this case)
    client.initialize();

    let request = String::new();
    let response = client.call_procedure("mock_rpc", &request).await;
    println!("Received response: {:?}", response);

    client.close_connection();

    Ok(())
}
