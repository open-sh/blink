use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};

use crate::FocusArea;

/// Event is any type of terminal event that Blink can compute.
pub enum Event {
    KeyPress(KeyEvent),
    _Mock,
    // TODO: Mouse events
}

/// Any sort of high-level command.
pub enum BlinkCommand {
    Quit,
    ToggleFocus, // REFACTOR: Directional focus?
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorRight,
    InsertChar(char),
    DeleteBackward,
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
///
/// TODO: Add Mode into the parameters of this function.
/// TODO: Add FocusArea so that we can have specific keybindings dependeing upon
/// where the user is currently focused.
pub fn handle_event(event: Event, focus_area: FocusArea) -> Vec<BlinkCommand> {
    let mut commands = Vec::new();

    match event {
        Event::KeyPress(key_event) => match key_event.code {
            // TODO: Send `key_event` into `handle_key_event()` so that I can match the mode.
            KeyCode::Char('q') => commands.push(BlinkCommand::Quit),
            KeyCode::Tab => commands.push(BlinkCommand::ToggleFocus),
            KeyCode::Up | KeyCode::Char('k') => {
                if focus_area == FocusArea::SidePanel {
                    commands.push(BlinkCommand::MoveCursorUp)
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if focus_area == FocusArea::SidePanel {
                    commands.push(BlinkCommand::MoveCursorDown)
                }
            }
            KeyCode::Left => {
                if focus_area == FocusArea::URLInput {
                    commands.push(BlinkCommand::MoveCursorLeft)
                }
            }
            KeyCode::Right => {
                if focus_area == FocusArea::URLInput {
                    commands.push(BlinkCommand::MoveCursorRight)
                }
            }
            KeyCode::Backspace => {
                if focus_area == FocusArea::URLInput {
                    commands.push(BlinkCommand::DeleteBackward)
                }
            }
            KeyCode::Char(c) => {
                if focus_area == FocusArea::URLInput {
                    commands.push(BlinkCommand::InsertChar(c))
                }
            }
            _ => {}
        },
        Event::_Mock => {}
    }

    return commands;
}
