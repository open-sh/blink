use std::time::Duration;

use anyhow::Result;
use crossterm::event::KeyModifiers;
pub use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};

use crate::{
    editor::EditorMode,
    keys::{KeyCombination, KeybindingMap},
    FocusArea,
};

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
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorRight,
    InsertChar(char),
    DeleteBackward,
    DeleteForward,
    EnterInsertMode,
    EnterNormalMode,
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
    focus_area: FocusArea,
    bindings: &KeybindingMap,
    editor_mode: &EditorMode,
) -> Vec<BlinkCommand> {
    let mut commands = Vec::new();

    match event {
        Event::KeyPress(key_event) => {
            let key_comb = KeyCombination::new(key_event.code, key_event.modifiers);

            if focus_area == FocusArea::Editor {
                match editor_mode {
                    EditorMode::Normal => {
                        // Handle normal mode keybindings.
                        if let Some(command) = bindings.get_command(key_comb) {
                            commands.push(*command);
                        }
                    }
                    EditorMode::Insert => match key_event.code {
                        KeyCode::Char(c) => commands.push(BlinkCommand::InsertChar(c)),
                        KeyCode::Esc => commands.push(BlinkCommand::EnterNormalMode),
                        KeyCode::Backspace => commands.push(BlinkCommand::DeleteBackward),
                        _ => {}
                    },
                }
            } else {
                if let Some(command) = bindings.get_command(key_comb) {
                    commands.push(*command);
                } else {
                    // Default behavior: Insert a char if the focus is URLInput without modifiers.
                    if focus_area == FocusArea::URLInput {
                        match key_event.code {
                            KeyCode::Char(c) if key_event.modifiers == KeyModifiers::NONE => {
                                commands.push(BlinkCommand::InsertChar(c));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Event::_Mock => {}
    }

    return commands;
}
