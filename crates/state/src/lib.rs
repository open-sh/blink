use anyhow::{Context, Result};
use config::BlinkConfig;
use notify::{Event as NotifyEvent, RecommendedWatcher, Result as NotifyResult, Watcher};
use std::{
    path::Path,
    sync::mpsc::{channel, Receiver},
};
use tui::{
    events::{handle_event, poll_events, BlinkCommand},
    BlinkRenderer,
};
use utils::{error, info};

/// Main state of the application.
pub struct BlinkState {
    pub renderer: BlinkRenderer,
    pub config: BlinkConfig,
    // Receiver to receive watcher events of configuration
    config_watcher_rx: Receiver<NotifyResult<NotifyEvent>>,
    should_quit: bool,
}

impl BlinkState {
    /// The states gets initialized only after the config is loaded.
    /// This happens so that we can inject properties (if they exist)
    /// from the `BlinkConfig` (global and/or local) into the `BlinkState`.
    pub fn new(config: BlinkConfig) -> Result<Self> {
        let message = config
            .mock
            .clone()
            .unwrap_or_else(|| "Hello not from the config".to_string());

        let (config_watcher_tx, config_watcher_rx) = channel();

        // Clone of sender channel to move to thread.
        let watcher_tx = config_watcher_tx.clone();
        let config_path = Path::new(".");

        std::thread::spawn(move || {
            assert!(config_path.is_dir(), "The current path '.' is not a valid directory");

            let mut watcher: RecommendedWatcher =
                match Watcher::new(watcher_tx, notify::Config::default()) {
                    Ok(w) => w,
                    Err(e) => {
                        error!("ERROR: Initializing watcher configuration: {:?}", e);
                        return;
                    }
                };

            // NOTE: Watching the current path instead of the actual `blink.toml` file because some editors
            // like Neovim have weird writing patterns. To avoid conflicts with any kind of editors, it's easier
            // watch over the current path.
            if let Err(e) = watcher.watch(config_path, notify::RecursiveMode::NonRecursive) {
                error!("ERROR: Watching config file: {:?}", e)
            }

            loop {}
        });

        Ok(Self {
            renderer: BlinkRenderer::new(message),
            config,
            config_watcher_rx,
            should_quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        assert!(!self.should_quit, "The `should_quit` property should start as `false`");
        let mut terminal = self.renderer.init();

        loop {
            self.renderer.draw(&mut terminal)?;
            self.handle_events().context("ERROR: Handling events")?;
            self.check_config_updates()
                .context("ERROR: Checking config updates")?;

            if self.should_quit {
                break;
            }
        }

        self.renderer.restore();
        Ok(())
    }

    // Event handling.
    fn handle_events(&mut self) -> Result<()> {
        let events = poll_events().context("ERROR: polling events.")?;
        for event in events {
            let commands = handle_event(event);
            for command in commands {
                match command {
                    BlinkCommand::Quit => self.should_quit = true,
                }
            }
        }

        Ok(())
    }

    /// Check updates in the config file.
    fn check_config_updates(&mut self) -> Result<()> {
        use notify::event::{EventKind, ModifyKind};
        use std::sync::mpsc::TryRecvError;

        // Try to receive all non-blocking available events.
        loop {
            match self.config_watcher_rx.try_recv() {
                Ok(Ok(event)) => {
                    for path in &event.paths {
                        if path.ends_with("blink.toml") {
                            match event.kind {
                                EventKind::Modify(ModifyKind::Data(_))
                                | EventKind::Modify(ModifyKind::Any) => {
                                    self.reload_config()?;
                                }
                                _ => {
                                    // We don't really care about other kind of events.
                                }
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    error!("ERROR: watching config file: {:?}", e);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    error!("Config watcher channel disconnected.");
                    break;
                }
            }
        }
        Ok(())
    }

    /// Reload config from `blink.toml`.
    fn reload_config(&mut self) -> Result<()> {
        info!("Reloading config...");
        self.config = BlinkConfig::load().context("ERROR: Loading configuration during reload")?;

        let new_message = self
            .config
            .mock
            .clone()
            .unwrap_or_else(|| "Hello not from the config".to_string());
        self.renderer.update_message(new_message);

        info!("Config reloaded.");

        Ok(())
    }
}
