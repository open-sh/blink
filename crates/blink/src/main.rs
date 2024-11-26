// TODO: Add Clap for the cli of this project.

use anyhow::Result;
use config::BlinkConfig;
use state::BlinkState;
use utils::init_logging;

fn main() -> Result<()> {
    init_logging()?;

    let config = BlinkConfig::load()?;

    let mut blink_state = BlinkState::new(config)?;
    blink_state.run()
}
