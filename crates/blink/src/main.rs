use state::BlinkState;
use anyhow::Result;
use utils::init_logging;

fn main() -> Result<()> {
    init_logging()?;

    let mut blink_state = BlinkState::new();
    blink_state.run()
}
