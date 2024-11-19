use state::BlinkState;
use anyhow::Result;

fn main() -> Result<()> {
    let mut blink_state = BlinkState::new();
    blink_state.run()
}
