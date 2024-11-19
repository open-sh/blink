use tui::BlinkRenderer;
use anyhow::Result;

/// Main state of the application.
pub struct BlinkState {
    pub renderer: BlinkRenderer
}

impl BlinkState {
    pub fn new() -> Self {
        let message = String::from("Hello, Blink");

        Self {
            renderer: BlinkRenderer {
                message,
                should_quit: false,
            }
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let terminal = self.renderer.init();
        self.renderer.draw(terminal)?;
        self.renderer.restore();
        Ok(())
    }
}
