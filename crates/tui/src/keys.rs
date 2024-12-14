use std::collections::HashMap;

use anyhow::{anyhow, Result};
use config::KeybindingConfig;
use tui_textarea::{Input, Key};
use utils::VimMode;

use crate::events::BlinkCommand;

/// Helper macro to create an `Input`.
macro_rules! input {
    ($key:expr, $ctrl:expr, $alt:expr, $shift:expr) => {
        Input {
            key: $key,
            ctrl: $ctrl,
            alt: $alt,
            shift: $shift,
        }
    };
    ($key:expr) => {
        Input {
            key: $key,
            ctrl: false,
            alt: false,
            shift: false,
        }
    };
}

/// Maps `KeyCombination` to a `BlinkCommand`.
pub struct KeybindingMap {
    bindings: HashMap<Input, Vec<(BlinkCommand, VimMode)>>,
}

impl KeybindingMap {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn add_binding(&mut self, input: Input, command: BlinkCommand, mode: VimMode) {
        self.bindings
            .entry(input)
            .or_insert_with(Vec::new)
            .push((command, mode));
    }

    pub fn get_command(&self, input: Input, current_mode: VimMode) -> Option<BlinkCommand> {
        if let Some(bindings) = self.bindings.get(&input) {
            // Check all the bindings for the key and returns the first that matches the current mode.
            // The logic here is quite simple: if we found a corresponding or `Insert`, return it.
            for (cmd, binding_mode) in bindings {
                match binding_mode {
                    VimMode::Normal if current_mode == VimMode::Normal => return Some(*cmd),
                    VimMode::Visual if current_mode == VimMode::Visual => return Some(*cmd),
                    VimMode::Insert if current_mode == VimMode::Insert => return Some(*cmd),
                    _ => {}
                }
            }
        }
        None
    }

    pub fn default_keybindings() -> Self {
        let mut map = KeybindingMap::new();

        //
        // Insert mode bindings.
        // Also valid as to when vim_mode is false.
        //

        map.add_binding(input!(Key::Char('b'), true, false, false), BlinkCommand::MoveCursorLeft, VimMode::Insert);
        map.add_binding(input!(Key::Char('b'), false, true, false), BlinkCommand::MoveCursorLeftByWord, VimMode::Insert);
        map.add_binding(input!(Key::Char('B'), false, true, true), BlinkCommand::MoveCursorLeftByWordSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Char('f'), true, false, false), BlinkCommand::MoveCursorRight, VimMode::Insert);
        map.add_binding(input!(Key::Char('f'), false, true, false), BlinkCommand::MoveCursorRightByWord, VimMode::Insert);
        map.add_binding(input!(Key::Char('F'), false, true, true), BlinkCommand::MoveCursorRightByWordSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Char('q'), true, false, false), BlinkCommand::Quit, VimMode::Insert);
        map.add_binding(input!(Key::Tab), BlinkCommand::ToggleFocus, VimMode::Insert);
        map.add_binding(input!(Key::Tab), BlinkCommand::ToggleFocus, VimMode::Normal);

        map.add_binding(input!(Key::Up), BlinkCommand::MoveCursorUp, VimMode::Insert);
        map.add_binding(input!(Key::Char('p'), true, false, false), BlinkCommand::MoveCursorUp, VimMode::Insert);
        map.add_binding(input!(Key::Char('p'), true, false, true), BlinkCommand::MoveCursorUpSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Char('p'), false, true, false), BlinkCommand::MoveCursorUpParagraph, VimMode::Insert);
        map.add_binding(input!(Key::Char('P'), false, true, true), BlinkCommand::MoveCursorUpParagraphSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Down), BlinkCommand::MoveCursorDown, VimMode::Insert);
        map.add_binding(input!(Key::Char('n'), true, false, false), BlinkCommand::MoveCursorDown, VimMode::Insert);
        map.add_binding(input!(Key::Char('n'), true, false, true), BlinkCommand::MoveCursorDownSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Char('n'), false, true, false), BlinkCommand::MoveCursorDownParagraph, VimMode::Insert);
        map.add_binding(input!(Key::Char('N'), false, true, true), BlinkCommand::MoveCursorDownParagraphSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Left), BlinkCommand::MoveCursorLeft, VimMode::Insert);
        map.add_binding(input!(Key::Left, false, false, true), BlinkCommand::MoveCursorLeftSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Left, true, false, false), BlinkCommand::MoveCursorLeftByWord, VimMode::Insert);
        map.add_binding(input!(Key::Left, false, true, false), BlinkCommand::MoveCursorLeftByWord, VimMode::Insert);
        map.add_binding(input!(Key::Left, true, false, true), BlinkCommand::MoveCursorLeftByWordSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Left, false, true, true), BlinkCommand::MoveCursorLeftByWordSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Right), BlinkCommand::MoveCursorRight, VimMode::Insert);
        map.add_binding(input!(Key::Right, false, false, true), BlinkCommand::MoveCursorRightSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Right, true, false, false), BlinkCommand::MoveCursorRightByWord, VimMode::Insert);
        map.add_binding(input!(Key::Right, false, true, false), BlinkCommand::MoveCursorRightByWord, VimMode::Insert);
        map.add_binding(input!(Key::Right, true, false, true), BlinkCommand::MoveCursorRightByWordSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Right, false, true, true), BlinkCommand::MoveCursorRightByWordSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Up), BlinkCommand::MoveCursorUp, VimMode::Insert);
        map.add_binding(input!(Key::Up, false, false, true), BlinkCommand::MoveCursorUpSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Up, true, false, false), BlinkCommand::MoveCursorUpParagraph, VimMode::Insert);
        map.add_binding(input!(Key::Up, true, false, true), BlinkCommand::MoveCursorUpParagraphSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Down), BlinkCommand::MoveCursorDown, VimMode::Insert);
        map.add_binding(input!(Key::Down, false, false, true), BlinkCommand::MoveCursorDownSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Down, true, false, false), BlinkCommand::MoveCursorDownParagraph, VimMode::Insert);
        map.add_binding(input!(Key::Down, true, false, true), BlinkCommand::MoveCursorDownParagraphSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Backspace), BlinkCommand::DeleteBackward, VimMode::Insert);
        map.add_binding(input!(Key::Backspace, false, true, false), BlinkCommand::DeleteWordBack, VimMode::Insert);

        map.add_binding(input!(Key::Delete), BlinkCommand::DeleteForward, VimMode::Insert);
        map.add_binding(input!(Key::Delete, false, true, false), BlinkCommand::DeleteWordForward, VimMode::Insert);

        map.add_binding(input!(Key::Char('k'), true, false, false), BlinkCommand::DeleteUntilEOL, VimMode::Insert);
        map.add_binding(input!(Key::Char('j'), true, false, false), BlinkCommand::DeleteUntilHOL, VimMode::Insert);

        map.add_binding(input!(Key::Char('u'), true, false, false), BlinkCommand::Undo, VimMode::Insert);
        map.add_binding(input!(Key::Char('r'), true, false, false), BlinkCommand::Redo, VimMode::Insert);

        map.add_binding(input!(Key::Char('c'), true, false, false), BlinkCommand::Copy, VimMode::Insert);
        map.add_binding(input!(Key::Char('v'), true, false, false), BlinkCommand::Paste, VimMode::Insert);
        map.add_binding(input!(Key::Char('x'), true, false, false), BlinkCommand::Cut, VimMode::Insert);

        map.add_binding(input!(Key::Char('a'), true, false, false), BlinkCommand::MoveCusorBOL, VimMode::Insert);
        map.add_binding(input!(Key::Char('A'), true, false, true), BlinkCommand::MoveCusorBOLSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Home), BlinkCommand::MoveCusorBOL, VimMode::Insert);
        map.add_binding(input!(Key::Home, false, false, true), BlinkCommand::MoveCusorBOLSelecting, VimMode::Insert);
        map.add_binding(input!(Key::Char('e'), true, false, false), BlinkCommand::MoveCusorEOL, VimMode::Insert);
        map.add_binding(input!(Key::Char('E'), true, false, true), BlinkCommand::MoveCusorEOLSelecting, VimMode::Insert);
        map.add_binding(input!(Key::End), BlinkCommand::MoveCusorEOL, VimMode::Insert);
        map.add_binding(input!(Key::End, false, false, true), BlinkCommand::MoveCusorEOLSelecting, VimMode::Insert);

        map.add_binding(input!(Key::Enter), BlinkCommand::Enter, VimMode::Insert);

        map.add_binding(input![Key::Esc], BlinkCommand::EnterNormalMode, VimMode::Insert);

        //
        // Normal mode bindings.
        //

        map.add_binding(input!(Key::Char('k')), BlinkCommand::MoveCursorUp, VimMode::Normal);
        map.add_binding(input!(Key::Char('h')), BlinkCommand::MoveCursorLeft, VimMode::Normal);
        map.add_binding(input!(Key::Char('j')), BlinkCommand::MoveCursorDown, VimMode::Normal);
        map.add_binding(input!(Key::Char('l')), BlinkCommand::MoveCursorRight, VimMode::Normal);

        map.add_binding(input!(Key::Char('b')), BlinkCommand::MoveCursorLeftByWord, VimMode::Normal);
        map.add_binding(input!(Key::Char('w')), BlinkCommand::MoveCursorRightByWord, VimMode::Normal);
        map.add_binding(input!(Key::Char('e')), BlinkCommand::MoveCursorRightByWordEnd, VimMode::Normal);

        map.add_binding(input!(Key::Char('b'), false, false, true), BlinkCommand::MoveCursorLeftByWordParagraph, VimMode::Normal);
        map.add_binding(input!(Key::Char('w'), false, false, true), BlinkCommand::MoveCursorRightByWordParagraph, VimMode::Normal);

        map.add_binding(input!(Key::Char('i')), BlinkCommand::EnterInsertMode, VimMode::Normal);
        map.add_binding(input!(Key::Char('v')), BlinkCommand::EnterVisualMode, VimMode::Normal);

        map.add_binding(input!(Key::Char('x')), BlinkCommand::DeleteForward, VimMode::Normal);

        map.add_binding(input!(Key::Char('$')), BlinkCommand::MoveCusorEOL, VimMode::Normal);
        map.add_binding(input!(Key::Char('A'), false, false, true), BlinkCommand::MoveCusorEOLIntoInsertMode, VimMode::Normal);
        map.add_binding(input!(Key::Char('0')), BlinkCommand::MoveCusorBOL, VimMode::Normal);
        map.add_binding(input!(Key::Char('I'), false, false, true), BlinkCommand::MoveCusorBOLIntoInsertMode, VimMode::Normal);

        map.add_binding(input!(Key::Char('D'), false, false, true), BlinkCommand::DeleteUntilEOL, VimMode::Normal);
        map.add_binding(input!(Key::Char('C'), false, false, true), BlinkCommand::DeleteUntilEOLIntoInsertMode, VimMode::Normal);

        map.add_binding(input!(Key::Char('p')), BlinkCommand::Paste, VimMode::Normal);

        map.add_binding(input!(Key::Char('u')), BlinkCommand::Undo, VimMode::Normal);
        map.add_binding(input!(Key::Char('U'), false, false, true), BlinkCommand::Redo, VimMode::Normal);

        map.add_binding(input!(Key::Char('o')), BlinkCommand::Newline, VimMode::Normal);
        map.add_binding(input!(Key::Char('O'), false, false, true), BlinkCommand::NewlineUp, VimMode::Normal);

        map.add_binding(input!(Key::Char('q')), BlinkCommand::Quit, VimMode::Normal);

        map.add_binding(input!(Key::Enter), BlinkCommand::Enter, VimMode::Normal);
        map.add_binding(input!(Key::Enter, false, true, false), BlinkCommand::OpenInEditor, VimMode::Normal);

        //
        // Visual mode bindings.
        //

        map.add_binding(input!(Key::Char('k')), BlinkCommand::MoveCursorUp, VimMode::Visual);
        map.add_binding(input!(Key::Char('h')), BlinkCommand::MoveCursorLeft, VimMode::Visual);
        map.add_binding(input!(Key::Char('j')), BlinkCommand::MoveCursorDown, VimMode::Visual);
        map.add_binding(input!(Key::Char('l')), BlinkCommand::MoveCursorRight, VimMode::Visual);

        map.add_binding(input!(Key::Char('b')), BlinkCommand::MoveCursorLeftByWord, VimMode::Visual);
        map.add_binding(input!(Key::Char('w')), BlinkCommand::MoveCursorRightByWord, VimMode::Visual);
        map.add_binding(input!(Key::Char('e')), BlinkCommand::MoveCursorRightByWordEnd, VimMode::Visual);

        map.add_binding(input!(Key::Char('b'), false, false, true), BlinkCommand::MoveCursorLeftByWordParagraph, VimMode::Visual);
        map.add_binding(input!(Key::Char('w'), false, false, true), BlinkCommand::MoveCursorRightByWordParagraph, VimMode::Visual);

        map.add_binding(input!(Key::Char('$')), BlinkCommand::MoveCusorEOL, VimMode::Visual);
        map.add_binding(input!(Key::Char('0')), BlinkCommand::MoveCusorBOL, VimMode::Visual);

        map.add_binding(input!(Key::Char('y')), BlinkCommand::Copy, VimMode::Visual);
        map.add_binding(input!(Key::Char('d')), BlinkCommand::Cut, VimMode::Visual);
        map.add_binding(input!(Key::Char('c')), BlinkCommand::CutIntoInsertMode, VimMode::Visual);
        map.add_binding(input!(Key::Char('s')), BlinkCommand::CutIntoInsertMode, VimMode::Visual);
        map.add_binding(input!(Key::Char('p')), BlinkCommand::CutIntoInsertMode, VimMode::Visual);

        map.add_binding(input!(Key::Char('q')), BlinkCommand::Quit, VimMode::Visual);

        map.add_binding(input![Key::Esc], BlinkCommand::EnterNormalMode, VimMode::Visual);

        return map;
    }

    /// Add keybindings from the config.
    /// These bindings will overwrite the default ones if there is any conflict.
    pub fn add_bindings_from_config(&mut self, config_bindings: &[KeybindingConfig]) -> Result<()> {
        for binding in config_bindings {
            let key = parse_key(binding)?;
            let mode = parse_mode(&binding.mode);
            let command = parse_blink_command(&binding.command)?;

            self.add_binding(key, command, mode);
        }
        Ok(())
    }
}

pub fn parse_mode(s: &str) -> VimMode {
    match s.to_lowercase().as_str() {
        "normal" => VimMode::Normal,
        "insert" => VimMode::Insert,
        _ => VimMode::Insert,
    }
}

fn parse_key(binding: &KeybindingConfig) -> Result<Input> {
    let key = match binding.key.to_lowercase().as_str() {
        "enter" => Key::Enter,
        "backspace" => Key::Backspace,
        "tab" => Key::Tab,
        "esc" | "escape" => Key::Esc,
        "up" => Key::Up,
        "down" => Key::Down,
        "left" => Key::Left,
        "right" => Key::Right,
        k if k.len() == 1 => Key::Char(k.chars().next().unwrap()),
        _ => return Err(anyhow!("Unknown key code: {}", binding.key)),
    };

    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;

    for m in &binding.modifiers {
        match m.to_lowercase().as_str() {
            "control" | "ctrl" => ctrl = true,
            "alt" => alt = true,
            "shift" => shift = true,
            _ => return Err(anyhow!("Unknown key modifier: {}", m)),
        }
    }

    Ok(Input {
        key,
        ctrl,
        alt,
        shift,
    })
}

/// NOTE: Maybe automate this?
fn parse_blink_command(cmd: &str) -> Result<BlinkCommand> {
    match cmd.to_lowercase().as_str() {
        "quit" => Ok(BlinkCommand::Quit),
        "togglefocus" => Ok(BlinkCommand::ToggleFocus),
        "enterinsertmode" => Ok(BlinkCommand::EnterInsertMode),
        "entervisualmode" => Ok(BlinkCommand::EnterVisualMode),
        "enternormalmode" => Ok(BlinkCommand::EnterNormalMode),

        "movecursorup" => Ok(BlinkCommand::MoveCursorUp),
        "movecursordown" => Ok(BlinkCommand::MoveCursorDown),

        "movecursorleft" => Ok(BlinkCommand::MoveCursorLeft),
        "movecursorleftselecting" => Ok(BlinkCommand::MoveCursorLeftSelecting),
        "movecursorleftbyword" => Ok(BlinkCommand::MoveCursorLeftByWord),
        "movecursorleftbywordselecting" => Ok(BlinkCommand::MoveCursorLeftByWordSelecting),
        "movecursorleftbywordparagraph" => Ok(BlinkCommand::MoveCursorLeftByWordParagraph),

        "movecursorright" => Ok(BlinkCommand::MoveCursorRight),
        "movecursorrightselecting" => Ok(BlinkCommand::MoveCursorRightSelecting),
        "movecursorrightbyword" => Ok(BlinkCommand::MoveCursorRightByWord),
        "movecursorrightbywordselecting" => Ok(BlinkCommand::MoveCursorRightByWordSelecting),
        "movecursorrightbywordend" => Ok(BlinkCommand::MoveCursorRightByWordEnd),
        "movecursorrightbywordparagraph" => Ok(BlinkCommand::MoveCursorRightByWordParagraph),

        "deletebackward" => Ok(BlinkCommand::DeleteBackward),
        "deleteforward" => Ok(BlinkCommand::DeleteForward),
        "deleteword" => Ok(BlinkCommand::DeleteWordBack),
        _ => Err(anyhow!("Unknown BlinkCommand: {}", cmd)),
    }
}
