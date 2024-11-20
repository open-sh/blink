use config::BlinkConfig;
use tui::BlinkRenderer;
use anyhow::Result;

/// Main state of the application.
pub struct BlinkState {
    pub renderer: BlinkRenderer,
    pub config: BlinkConfig
}

impl BlinkState {
    /// The states gets initialized only after the config is loaded.
    /// This happens so that we can inject properties (if they exist)
    /// from the `BlinkConfig` (global and/or local) into the `BlinkState`.
    pub fn new(config: BlinkConfig) -> Self {
        let message = config.mock.clone().unwrap_or_else(|| "Hello not from the config".to_string());

        Self {
            renderer: BlinkRenderer::new(message),
            config,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let terminal = self.renderer.init();
        self.renderer.draw(terminal)?;
        self.renderer.restore();
        Ok(())
    }
}
