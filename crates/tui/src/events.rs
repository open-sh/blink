use std::time::Duration;

use anyhow::Result;
pub use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use tui_textarea::{Input, Key};
use utils::VimMode;

use crate::keys::KeybindingMap;

/// Event is any type of terminal event that Blink can compute.
pub enum Event {
    KeyPress(KeyEvent),
    _Mock,
    // TODO: Mouse events
}

/// Any sort of high-level command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlinkCommand {
    Quit,
    ToggleFocus, // REFACTOR: Directional focus?
    EnterInsertMode,
    EnterVisualMode,
    EnterNormalMode,

    // Movement.
    MoveCursorUp,
    MoveCursorDown,

    MoveCursorLeft,
    MoveCursorLeftSelecting,
    MoveCursorLeftByWord,
    MoveCursorLeftByWordSelecting,
    MoveCursorLeftByWordParagraph,

    MoveCursorRight,
    MoveCursorRightSelecting,
    MoveCursorRightByWord,
    MoveCursorRightByWordSelecting,
    MoveCursorRightByWordEnd,
    MoveCursorRightByWordParagraph,

    // Editing.
    InsertChar(char),
    DeleteBackward,
    DeleteWord,
    DeleteForward,
}

/// Capture events from the terminal and return them into a Vector.
pub fn poll_events() -> Result<Vec<Event>> {
    let mut events = Vec::new();

    // We use `event::poll` here with a timeout of 0 to make it non-blocking.
    if event::poll(Duration::from_millis(0))? {
        // c_event is a crossterm event.
        if let Ok(c_event) = event::read() {
            match c_event {
                CrosstermEvent::Key(key_event) => events.push(Event::KeyPress(key_event)),
                _ => {}
            }
        }
    }

    Ok(events)
}

/// Maps `CrosstermEvent` to a Vec<BlinkCommand>.
pub fn handle_event(
    event: Event,
    bindings: &KeybindingMap,
    editor_mode: &VimMode,
) -> Vec<BlinkCommand> {
    let mut commands = Vec::new();

    match event {
        Event::KeyPress(key_event) => {
            let input: Input = key_event.into();

            if let Some(command) = bindings.get_command(input.clone(), *editor_mode) {
                // Found a command whose mode is matched with the current one.
                commands.push(command);
            } else {
                // If we haven't found anything, and we are in `Insert`/`Any`, we can
                // handle it directly.
                if *editor_mode == VimMode::Insert || *editor_mode == VimMode::Any {
                    match input.key {
                        Key::Char(c) => commands.push(BlinkCommand::InsertChar(c)),
                        Key::Esc => commands.push(BlinkCommand::EnterNormalMode),
                        _ => {}
                    }
                }
            }
        }
        Event::_Mock => {}
    }

    return commands;
}
