use anyhow::{Context, Result};
use config::BlinkConfig;
use notify::{Event as NotifyEvent, RecommendedWatcher, Result as NotifyResult, Watcher};
use tui::url_input::tui_textarea::TextArea;
use std::io::Write;
use std::process::Command;
use std::{
    path::Path,
    sync::mpsc::{channel, Receiver},
};
use tempfile::NamedTempFile;
use tui::{
    events::{handle_event, poll_events, BlinkCommand},
    keys::KeybindingMap,
    BlinkRenderer, FocusArea,
};
use utils::{error, info, VimMode};

/// Main state of the application.
pub struct BlinkState<'a> {
    pub renderer: BlinkRenderer<'a>,
    pub config: BlinkConfig,
    // Receiver to receive watcher events of configuration
    config_watcher_rx: Receiver<NotifyResult<NotifyEvent>>,
    should_quit: bool,
    key_bindings: KeybindingMap,
}

impl<'a> BlinkState<'a> {
    /// The states gets initialized only after the config is loaded.
    /// This happens so that we can inject properties (if they exist)
    /// from the `BlinkConfig` (global and/or local) into the `BlinkState`.
    pub fn new(config: BlinkConfig) -> Result<Self> {
        // Renderer variables.
        let requests = config.local_requests.requests.clone();
        let vim_mode = config.vim_mode;

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
            renderer: BlinkRenderer::new(requests, vim_mode),
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
            let event_mode = match self.renderer.focus_area {
                FocusArea::Editor => self.renderer.editor.mode,
                FocusArea::URLInput => self.renderer.url_input.mode,
                FocusArea::SidePanel => self.renderer.side_panel.mode,
            };

            let commands = handle_event(event, &self.key_bindings, &event_mode);
            for command in commands {
                match command {
                    BlinkCommand::Quit => self.should_quit = true,
                    BlinkCommand::ToggleFocus => self.toggle_focus(),
                    BlinkCommand::EnterInsertMode => self.enter_insert_mode(),
                    BlinkCommand::EnterVisualMode => self.enter_visual_mode(),
                    BlinkCommand::EnterNormalMode => self.enter_normal_mode(),
                    BlinkCommand::Enter => self.enter(),

                    // Movement.
                    BlinkCommand::MoveCursorUp => self.move_cursor_up(),
                    BlinkCommand::MoveCursorUpParagraph => self.move_cursor_up_paragraph(),
                    BlinkCommand::MoveCursorUpSelecting => self.move_cursor_up_selecting(),
                    BlinkCommand::MoveCursorUpParagraphSelecting => self.move_cursor_up_paragraph_selecting(),

                    BlinkCommand::MoveCursorDown => self.move_cursor_down(),
                    BlinkCommand::MoveCursorDownParagraph => self.move_cursor_down_paragraph(),
                    BlinkCommand::MoveCursorDownSelecting => self.move_cursor_down_selecting(),
                    BlinkCommand::MoveCursorDownParagraphSelecting => self.move_cursor_down_paragraph_selecting(),

                    BlinkCommand::MoveCursorLeft => self.move_cursor_left(),
                    BlinkCommand::MoveCursorLeftSelecting => self.move_cursor_left_selecting(),
                    BlinkCommand::MoveCursorLeftByWord => self.move_cursor_left_by_word(),
                    BlinkCommand::MoveCursorLeftByWordSelecting => self.move_cursor_left_by_word_selecting(),
                    BlinkCommand::MoveCursorLeftByWordParagraph => self.move_cursor_left_by_word_paragraph(),

                    BlinkCommand::MoveCursorRight => self.move_cursor_right(),
                    BlinkCommand::MoveCursorRightSelecting => self.move_cursor_right_selecting(),
                    BlinkCommand::MoveCursorRightByWord => self.move_cursor_right_by_word(),
                    BlinkCommand::MoveCursorRightByWordSelecting => self.move_cursor_right_by_word_selecting(),
                    BlinkCommand::MoveCursorRightByWordParagraph => self.move_cursor_right_by_word_paragraph(),
                    BlinkCommand::MoveCursorRightByWordEnd => self.move_cursor_right_by_word_end(),

                    BlinkCommand::MoveCusorBOL => self.move_cursor_bol(),
                    BlinkCommand::MoveCusorBOLIntoInsertMode => self.move_cursor_bol_into_insert_mode(),
                    BlinkCommand::MoveCusorBOLSelecting => self.move_cursor_bol_selecting(),
                    BlinkCommand::MoveCusorEOL => self.move_cursor_eol(),
                    BlinkCommand::MoveCusorEOLIntoInsertMode => self.move_cursor_eol_into_insert_mode(),
                    BlinkCommand::MoveCusorEOLSelecting => self.move_cursor_eol_selecting(),

                    // Editing
                    BlinkCommand::InsertChar(c) => self.insert_char(c),

                    BlinkCommand::DeleteBackward => self.backspace(),
                    BlinkCommand::DeleteForward => self.delete_char(),
                    BlinkCommand::DeleteWordBack => self.delete_word_back(),
                    BlinkCommand::DeleteWordForward => self.delete_word_forward(),
                    BlinkCommand::DeleteUntilEOL => self.delete_until_eol(),
                    BlinkCommand::DeleteUntilEOLIntoInsertMode => self.delete_until_eol_into_insert_mode(),
                    BlinkCommand::DeleteUntilHOL => self.delete_until_hol(),

                    BlinkCommand::Newline => self.newline(),
                    BlinkCommand::NewlineUp => self.newline_up(),

                    BlinkCommand::Undo => self.undo(),
                    BlinkCommand::Redo => self.redo(),

                    BlinkCommand::Copy => self.copy(),
                    BlinkCommand::Paste => self.paste(),
                    BlinkCommand::Cut => self.cut(),
                    BlinkCommand::CutIntoInsertMode => self.cut_into_insert_mode(),

                    BlinkCommand::OpenInEditor => {
                        if self.renderer.focus_area == FocusArea::Editor {
                            self.open_in_editor()
                        }
                    },
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

        // Update vim_mode.
        let new_vim_mode = self.config.vim_mode.clone();
        self.renderer.update_vim_mode(new_vim_mode);

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

    pub fn open_in_editor(&mut self) {
        // 1 - Restore the terminal before leaving the TUI.
        self.renderer.restore();

        // Get the current content the `editor`.
        let lines = self.renderer.editor.text_area.lines();
        let full_text = lines.join("\n");

        // Create a temp file.
        let mut tmp = NamedTempFile::new().expect("Failed to create temp file");
        write!(tmp, "{}", full_text).expect("Failed to write to temp file");
        let tmp_path = tmp.path().to_path_buf(); // Cloning the path here to use after we drop `tmp`.

        // Close explicitly to free the reference to the `editor`.
        drop(tmp);

        // TOOD: Put this into config or something.
        let editor = "vim".to_string();

        let status = Command::new(&editor).arg(&tmp_path).status().expect("Failed to spawn editor");
        assert!(status.success(), "Editor exited with non-zero status. Keeping old content.");

        let new_body = std::fs::read_to_string(&tmp_path).expect("Failed to read edited file");
        // TODO?: remove_file(&tmp_path).ok() // Remove temp file.

        // Update the state of the `editor`.
        self.renderer.editor.text_area = TextArea::default();

        for (i, line) in new_body.lines().enumerate() {
            if i > 0 {
                self.renderer.editor.text_area.insert_newline();
            }
            self.renderer.editor.text_area.insert_str(line);
        }

        // Initialize the TUI again.
        let mut terminal = self.renderer.init();
        self.renderer.draw(&mut terminal).expect("Could not initialize the terminal again after opening the editor");
    }

    //
    // Movement.
    //

    fn move_cursor_up(&mut self) {
        match self.renderer.focus_area {
            FocusArea::SidePanel => self.renderer.side_panel.move_cursor_up(),
            FocusArea::Editor => self.renderer.editor.move_cursor_up(),
            _ => {}
        }
    }

    fn move_cursor_up_paragraph(&mut self) {
        match self.renderer.focus_area {
            FocusArea::Editor => self.renderer.editor.move_cursor_up_paragraph(),
            _ => {}
        }
    }

    fn move_cursor_up_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::Editor => self.renderer.editor.move_cursor_up_selecting(),
            _ => {}
        }
    }

    fn move_cursor_up_paragraph_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::Editor => self.renderer.editor.move_cursor_up_paragraph_selecting(),
            _ => {}
        }
    }

    fn move_cursor_down(&mut self) {
        match self.renderer.focus_area {
            FocusArea::SidePanel => self.renderer.side_panel.move_cursor_down(),
            FocusArea::Editor => self.renderer.editor.move_cursor_down(),
            _ => {}
        }
    }

    fn move_cursor_down_paragraph(&mut self) {
        match self.renderer.focus_area {
            FocusArea::Editor => self.renderer.editor.move_cursor_down_paragraph(),
            _ => {}
        }
    }

    fn move_cursor_down_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::Editor => self.renderer.editor.move_cursor_down_selecting(),
            _ => {}
        }
    }

    fn move_cursor_down_paragraph_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::Editor => self.renderer.editor.move_cursor_down_paragraph_selecting(),
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

    fn move_cursor_left_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_left_selecting(),
            FocusArea::Editor => self.renderer.editor.move_cursor_left_selecting(),
            _ => {}
        }
    }

    fn move_cursor_left_by_word(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_left_by_word(),
            FocusArea::Editor => self.renderer.editor.move_cursor_left_by_word(),
            _ => {}
        }
    }

    fn move_cursor_left_by_word_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_left_by_word_selecting(),
            FocusArea::Editor => self.renderer.editor.move_cursor_left_by_word_selecting(),
            _ => {}
        }
    }

    fn move_cursor_left_by_word_paragraph(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_left_by_word_paragraph(),
            FocusArea::Editor => self.renderer.editor.move_cursor_left_by_word_paragraph(),
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

    fn move_cursor_right_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_right_selecting(),
            FocusArea::Editor => self.renderer.editor.move_cursor_right_selecting(),
            _ => {}
        }
    }

    fn move_cursor_right_by_word(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_right_by_word(),
            FocusArea::Editor => self.renderer.editor.move_cursor_right_by_word(),
            _ => {}
        }
    }

    fn move_cursor_right_by_word_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self
                .renderer
                .url_input
                .move_cursor_right_by_word_selecting(),
            FocusArea::Editor => self.renderer.editor.move_cursor_right_by_word_selecting(),
            _ => {}
        }
    }

    fn move_cursor_right_by_word_paragraph(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self
                .renderer
                .url_input
                .move_cursor_right_by_word_paragraph(),
            FocusArea::Editor => self.renderer.editor.move_cursor_right_by_word_paragraph(),
            _ => {}
        }
    }

    fn move_cursor_right_by_word_end(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_right_by_word_end(),
            FocusArea::Editor => self.renderer.editor.move_cursor_right_by_word_end(),
            _ => {}
        }
    }

    fn move_cursor_bol(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_bol(),
            FocusArea::Editor => self.renderer.editor.move_cursor_bol(),
            _ => {}
        }
    }

    fn move_cursor_bol_into_insert_mode(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => {
                self.renderer.url_input.move_cursor_bol();
                self.enter_insert_mode()
            }
            FocusArea::Editor => {
                self.renderer.editor.move_cursor_bol();
                self.enter_insert_mode()
            }
            _ => {}
        }
    }

    fn move_cursor_bol_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_bol_selecting(),
            FocusArea::Editor => self.renderer.editor.move_cursor_bol_selecting(),
            _ => {}
        }
    }

    fn move_cursor_eol(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_eol(),
            FocusArea::Editor => self.renderer.editor.move_cursor_eol(),
            _ => {}
        }
    }

    fn move_cursor_eol_into_insert_mode(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => {
                self.renderer.url_input.move_cursor_eol();
                self.enter_insert_mode()
            }
            FocusArea::Editor => {
                self.renderer.editor.move_cursor_eol();
                self.enter_insert_mode()
            }
            _ => {}
        }
    }

    fn move_cursor_eol_selecting(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.move_cursor_eol_selecting(),
            FocusArea::Editor => self.renderer.editor.move_cursor_eol_selecting(),
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

    fn delete_char(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.delete_char(),
            FocusArea::Editor => self.renderer.editor.delete_char(),
            _ => {}
        }
    }

    fn delete_word_back(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.delete_word_back(),
            FocusArea::Editor => self.renderer.editor.delete_word_back(),
            _ => {}
        }
    }

    fn delete_word_forward(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.delete_word_forward(),
            FocusArea::Editor => self.renderer.editor.delete_word_forward(),
            _ => {}
        }
    }

    fn delete_until_eol(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.delete_until_eol(),
            FocusArea::Editor => self.renderer.editor.delete_until_eol(),
            _ => {}
        }
    }

    fn delete_until_eol_into_insert_mode(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => {
                self.renderer.url_input.delete_until_eol();
                self.enter_insert_mode();
            }
            FocusArea::Editor => {
                self.renderer.editor.delete_until_eol();
                self.enter_insert_mode();
            }
            _ => {}
        }
    }

    fn delete_until_hol(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.delete_until_hol(),
            FocusArea::Editor => self.renderer.editor.delete_until_hol(),
            _ => {}
        }
    }

    fn undo(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.undo(),
            FocusArea::Editor => self.renderer.editor.undo(),
            _ => {}
        }
    }

    fn redo(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.redo(),
            FocusArea::Editor => self.renderer.editor.redo(),
            _ => {}
        }
    }

    fn copy(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.copy(),
            FocusArea::Editor => self.renderer.editor.copy(),
            _ => {}
        }
    }

    fn paste(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.paste(),
            FocusArea::Editor => self.renderer.editor.paste(),
            _ => {}
        }
    }

    fn cut(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.cut(),
            FocusArea::Editor => self.renderer.editor.cut(),
            _ => {}
        }
    }

    fn cut_into_insert_mode(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.cut_into_insert_mode(),
            FocusArea::Editor => self.renderer.editor.cut_into_insert_mode(),
            _ => {}
        }
    }

    fn newline(&mut self) {
        match self.renderer.focus_area {
            FocusArea::Editor => self.renderer.editor.newline(),
            _ => {}
        }
    }

    fn newline_up(&mut self) {
        match self.renderer.focus_area {
            FocusArea::Editor => self.renderer.editor.newline_up(),
            _ => {}
        }
    }

    //
    // State handling.
    //

    fn enter(&mut self) {
        match self.renderer.focus_area {
            FocusArea::SidePanel => {
                let index = self.renderer.side_panel.selected_request;
                if let Some(req) = self.renderer.side_panel.requests.clone().get(index) {
                    self.renderer.load_request(req);
                    self.renderer.focus_area = FocusArea::Editor;
                }
            }
            FocusArea::URLInput => { /* TODO */ }
            FocusArea::Editor => {
                if self.renderer.editor.mode == VimMode::Insert {
                    self.renderer.editor.insert_char('\n')
                }
            }
        }
    }

    fn enter_insert_mode(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.enter_insert_mode(),
            FocusArea::Editor => self.renderer.editor.enter_insert_mode(),
            _ => {}
        }
    }

    fn enter_normal_mode(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.enter_normal_mode(),
            FocusArea::Editor => self.renderer.editor.enter_normal_mode(),
            _ => {}
        }
    }

    fn enter_visual_mode(&mut self) {
        match self.renderer.focus_area {
            FocusArea::URLInput => self.renderer.url_input.enter_visual_mode(),
            FocusArea::Editor => self.renderer.editor.enter_visual_mode(),
            _ => {}
        }
    }
}
