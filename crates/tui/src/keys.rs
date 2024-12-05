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
            // Verifica todos os bindings para essa tecla e retorna o primeiro que bater com o modo atual.
            // Aqui a lógica é simples: se encontrar um correspondente ao modo exato ou Any, retorna.
            // Se quiser priorizar certos modos, basta alterar a ordem ou lógica.
            for (cmd, binding_mode) in bindings {
                match binding_mode {
                    VimMode::Any => return Some(*cmd),
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

        map.add_binding(input!(Key::Char('b'), true, false, false), BlinkCommand::MoveCursorLeft, VimMode::Any);
        map.add_binding(input!(Key::Char('b'), true, false, true), BlinkCommand::MoveCursorLeftSelecting, VimMode::Any);
        map.add_binding(input!(Key::Char('b'), false, true, false), BlinkCommand::MoveCursorLeftByWord, VimMode::Any);

        map.add_binding(input!(Key::Char('f'), true, false, false), BlinkCommand::MoveCursorRight, VimMode::Any);
        map.add_binding(input!(Key::Char('f'), false, true, false), BlinkCommand::MoveCursorRightByWord, VimMode::Any);

        map.add_binding(input!(Key::Char('q'), true, false, false), BlinkCommand::Quit, VimMode::Any);
        map.add_binding(input!(Key::Tab), BlinkCommand::ToggleFocus, VimMode::Any);

        map.add_binding(input!(Key::Up), BlinkCommand::MoveCursorUp, VimMode::Any);
        map.add_binding(input!(Key::Char('p'), true, false, false), BlinkCommand::MoveCursorUp, VimMode::Any);

        map.add_binding(input!(Key::Down), BlinkCommand::MoveCursorDown, VimMode::Any);
        map.add_binding(input!(Key::Char('n'), true, false, false), BlinkCommand::MoveCursorDown, VimMode::Any);

        map.add_binding(input!(Key::Left), BlinkCommand::MoveCursorLeft, VimMode::Any);
        map.add_binding(input!(Key::Left, false, false, true), BlinkCommand::MoveCursorLeftSelecting, VimMode::Any);
        map.add_binding(input!(Key::Left, true, false, false), BlinkCommand::MoveCursorLeftByWord, VimMode::Any);

        map.add_binding(input!(Key::Right), BlinkCommand::MoveCursorRight, VimMode::Any);
        map.add_binding(input!(Key::Right, false, false, true), BlinkCommand::MoveCursorRightSelecting, VimMode::Any);
        map.add_binding(input!(Key::Right, true, false, false), BlinkCommand::MoveCursorRightByWord, VimMode::Any);

        map.add_binding(input!(Key::Backspace), BlinkCommand::DeleteBackward, VimMode::Any);
        map.add_binding(input!(Key::Backspace, false, true, false), BlinkCommand::DeleteWord, VimMode::Any);
        map.add_binding(input!(Key::Delete), BlinkCommand::DeleteForward, VimMode::Normal);

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

        //
        // Insert mode bindings.
        //

        map.add_binding(input![Key::Esc], BlinkCommand::EnterNormalMode, VimMode::Insert);

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
        _ => VimMode::Any,
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

fn parse_blink_command(cmd: &str) -> Result<BlinkCommand> {
    match cmd.to_lowercase().as_str() {
        "quit" => Ok(BlinkCommand::Quit),
        "togglefocus" => Ok(BlinkCommand::ToggleFocus),
        "movecursorup" => Ok(BlinkCommand::MoveCursorUp),
        "movecursordown" => Ok(BlinkCommand::MoveCursorDown),
        "movecursorleft" => Ok(BlinkCommand::MoveCursorLeft),
        "movecursorleftselecting" => Ok(BlinkCommand::MoveCursorLeftSelecting),
        "movecursorright" => Ok(BlinkCommand::MoveCursorRight),
        "deletebackward" => Ok(BlinkCommand::DeleteBackward),
        "deleteforward" => Ok(BlinkCommand::DeleteForward),
        "enterinsertmode" => Ok(BlinkCommand::EnterInsertMode),
        "enternormalmode" => Ok(BlinkCommand::EnterNormalMode),
        "movecursorleftbyword" => Ok(BlinkCommand::MoveCursorLeftByWord),
        "movecursorrightbyword" => Ok(BlinkCommand::MoveCursorRightByWord),
        _ => Err(anyhow!("Unknown BlinkCommand: {}", cmd)),
    }
}
