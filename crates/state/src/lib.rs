use anyhow::{Context, Result};
use config::BlinkConfig;
use notify::{Event as NotifyEvent, RecommendedWatcher, Result as NotifyResult, Watcher};
use std::{
    path::Path,
    sync::mpsc::{channel, Receiver},
};
use tui::{
    events::{handle_event, poll_events, BlinkCommand},
    keys::KeybindingMap,
    BlinkRenderer, FocusArea,
};
use utils::{error, info};

/// Main state of the application.
pub struct BlinkState {
    pub renderer: BlinkRenderer,
    pub config: BlinkConfig,
    // Receiver to receive watcher events of configuration
    config_watcher_rx: Receiver<NotifyResult<NotifyEvent>>,
    should_quit: bool,
    key_bindings: KeybindingMap,
}

impl BlinkState {
    /// The states gets initialized only after the config is loaded.
    /// This happens so that we can inject properties (if they exist)
    /// from the `BlinkConfig` (global and/or local) into the `BlinkState`.
    pub fn new(config: BlinkConfig) -> Result<Self> {
        // Renderer variables.
        let requests = config.local_requests.requests.clone();

        let (config_watcher_tx, config_watcher_rx) = channel();

        // Clone of sender channel to move to thread.
        let watcher_tx = config_watcher_tx.clone();
        let config_path = Path::new(".");

        std::thread::spawn(move || {
            assert!(
                config_path.is_dir(),
                "The current path '.' is not a valid directory"
            );

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

        // Initialize with default bindings.
        let mut key_bindings = KeybindingMap::default_keybindings();

        // Add keybindings from the config if there are any.
        if !config.keybindings.is_empty() {
            key_bindings
                .add_bindings_from_config(&config.keybindings)
                .context("ERROR: Adding keybindings from config")?;
        }

        Ok(Self {
            renderer: BlinkRenderer::new(requests),
            config,
            config_watcher_rx,
            should_quit: false,
            key_bindings,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        assert!(
            !self.should_quit,
            "The `should_quit` property should start as `false`"
        );
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
            let commands = handle_event(event, self.renderer.focus_area, &self.key_bindings, &self.renderer.editor.mode);
            for command in commands {
                match command {
                    BlinkCommand::Quit => self.should_quit = true,
                    BlinkCommand::ToggleFocus => self.toggle_focus(),
                    BlinkCommand::MoveCursorUp => self.move_cursor_up(),
                    BlinkCommand::MoveCursorDown => self.move_cursor_down(),
                    BlinkCommand::InsertChar(c) => self.insert_char(c),
                    BlinkCommand::DeleteBackward => self.backspace(),
                    BlinkCommand::MoveCursorLeft => self.move_cursor_left(),
                    BlinkCommand::MoveCursorRight => self.move_cursor_right(),
                    BlinkCommand::DeleteForward => self.renderer.editor.delete_char(),
                    BlinkCommand::EnterInsertMode => self.enter_insert_mode(),
                    BlinkCommand::EnterNormalMode => self.enter_normal_mode(),
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

    /// Reload config from `blink.toml` (local).
    fn reload_config(&mut self) -> Result<()> {
        info!("Reloading config...");
        self.config = BlinkConfig::load().context("ERROR: Loading configuration during reload")?;

        // Update requests.
        let new_requests = self.config.local_requests.requests.clone();
        self.renderer.update_requests(new_requests);

        // Update keybindings.
        self.key_bindings = KeybindingMap::default_keybindings();
        if !self.config.keybindings.is_empty() {
            self.key_bindings
                .add_bindings_from_config(&self.config.keybindings)
                .context("ERROR: Adding keybindings from reloaded config")?;
        }

        info!("Config reloaded.");
        Ok(())
    }

    /// Toggle focus between different areas of the TUI.
    pub fn toggle_focus(&mut self) {
        self.renderer.focus_area = match self.renderer.focus_area {
            FocusArea::SidePanel => FocusArea::URLInput,
            FocusArea::URLInput => FocusArea::Editor,
            FocusArea::Editor => FocusArea::SidePanel,
        }
    }

    //
    // Movement.
    //

    fn move_cursor_up(&mut self) {
        match self.renderer.focus_area {
            FocusArea::SidePanel => {
                if self.renderer.selected_request > 0 {
                    self.renderer.selected_request -= 1;
                }
            }
            FocusArea::Editor => self.renderer.editor.move_cursor_up(),
            _ => {}
        }
    }

    fn move_cursor_down(&mut self) {
        match self.renderer.focus_area {
            FocusArea::SidePanel => {
                if self.renderer.selected_request + 1 < self.renderer.requests.len() {
                    self.renderer.selected_request += 1;
                }
            }
            FocusArea::Editor => self.renderer.editor.move_cursor_down(),
            _ => {}
        }
    }

    fn move_cursor_left(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_left(),
            FocusArea::Editor => self.renderer.editor.move_cursor_left(),
            _ => {}
        }
    }

    fn move_cursor_right(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_right(),
            FocusArea::Editor => self.renderer.editor.move_cursor_right(),
            _ => {}
        }
    }

    //
    // Editing.
    //

    fn insert_char(&mut self, c: char) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.insert_char(c),
            FocusArea::Editor => self.renderer.editor.insert_char(c),
            _ => {}
        }
    }

    fn backspace(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.backspace(),
            FocusArea::Editor => self.renderer.editor.backspace(),
            _ => {}
        }
    }

    //
    // State handling.
    //

    fn enter_insert_mode(&mut self) {
        if self.renderer.focus_area == FocusArea::Editor {
            self.renderer.editor.enter_insert_mode();
        }
    }

    fn enter_normal_mode(&mut self) {
        if self.renderer.focus_area == FocusArea::Editor {
            self.renderer.editor.enter_normal_mode();
        }
    }
}
