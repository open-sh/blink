use std::{collections::HashMap, hash::Hash};

use anyhow::{anyhow, Context, Result};
use config::KeybindingConfig;
use crossterm::event::{KeyCode, KeyModifiers};
use utils::VimMode;

use crate::events::BlinkCommand;

/// Represents a combination of a key with it's modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyCombination {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Hash for KeyCombination {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.modifiers.bits().hash(state);
    }
}

impl KeyCombination {
    pub fn new(key: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { key, modifiers }
    }
}

/// Maps `KeyCombination` to a `BlinkCommand`.
pub struct KeybindingMap {
    bindings: HashMap<KeyCombination, (BlinkCommand, VimMode)>,
}

impl KeybindingMap {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn add_binding(&mut self, key_comb: KeyCombination, command: BlinkCommand, mode: VimMode) {
        self.bindings.insert(key_comb, (command, mode));
    }

    pub fn get_command(&self, key_comb: KeyCombination, current_mode: VimMode) -> Option<BlinkCommand> {
        if let Some((cmd, binding_mode)) = self.bindings.get(&key_comb) {
            match binding_mode {
                VimMode::Any => {
                    Some(*cmd)
                }
                VimMode::Normal => {
                    if current_mode == VimMode::Normal {
                        Some(*cmd)
                    } else {
                        None
                    }
                }
                VimMode::Insert => {
                    if current_mode == VimMode::Insert {
                        Some(*cmd)
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn default_keybindings() -> Self {
        let mut map = KeybindingMap::new();

        map.add_binding(
            KeyCombination::new(KeyCode::Char('b'), KeyModifiers::CONTROL),
            BlinkCommand::MoveCursorLeft,
            VimMode::Any
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('f'), KeyModifiers::CONTROL),
            BlinkCommand::MoveCursorRight,
            VimMode::Any
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('q'), KeyModifiers::CONTROL),
            BlinkCommand::Quit,
            VimMode::Any
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Tab, KeyModifiers::NONE),
            BlinkCommand::ToggleFocus,
            VimMode::Any
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Up, KeyModifiers::NONE),
            BlinkCommand::MoveCursorUp,
            VimMode::Any
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Down, KeyModifiers::NONE),
            BlinkCommand::MoveCursorDown,
            VimMode::Any
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Left, KeyModifiers::NONE),
            BlinkCommand::MoveCursorLeft,
            VimMode::Any
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Right, KeyModifiers::NONE),
            BlinkCommand::MoveCursorRight,
            VimMode::Any
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Backspace, KeyModifiers::NONE),
            BlinkCommand::DeleteBackward,
            VimMode::Any
        );

        //
        // Normal mode keybindings.
        //

        map.add_binding(
            KeyCombination::new(KeyCode::Char('k'), KeyModifiers::NONE),
            BlinkCommand::MoveCursorUp,
            VimMode::Normal
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('j'), KeyModifiers::NONE),
            BlinkCommand::MoveCursorDown,
            VimMode::Normal
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('i'), KeyModifiers::NONE),
            BlinkCommand::EnterInsertMode,
            VimMode::Normal
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('h'), KeyModifiers::NONE),
            BlinkCommand::MoveCursorLeft,
            VimMode::Normal
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('j'), KeyModifiers::NONE),
            BlinkCommand::MoveCursorDown,
            VimMode::Normal
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('k'), KeyModifiers::NONE),
            BlinkCommand::MoveCursorUp,
            VimMode::Normal
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('l'), KeyModifiers::NONE),
            BlinkCommand::MoveCursorRight,
            VimMode::Normal
        );

        map.add_binding(
            KeyCombination::new(KeyCode::Char('x'), KeyModifiers::NONE),
            BlinkCommand::DeleteForward,
            VimMode::Normal
        );

        return map;
    }

    /// Add keybindings from the config.
    /// These bindings will overwrite the default ones if there is any conflict.
    pub fn add_bindings_from_config(&mut self, config_bindings: &[KeybindingConfig]) -> Result<()> {
        for binding in config_bindings {
            let mut key = parse_key_code(&binding.key)
                .with_context(|| format!("Invalid key code: {}", binding.key))?;
            let modifiers = parse_key_modifiers(&binding.modifiers)
                .with_context(|| format!("Invalid modifiers: {:?}", binding.modifiers))?;
            let command = parse_blink_command(&binding.command)
                .with_context(|| format!("Invalid command: {}", binding.command))?;
            let mode = parse_mode(&binding.mode);

            // If shift is pressed and there is a char, map it to upper.
            if modifiers.contains(KeyModifiers::SHIFT) {
                if let KeyCode::Char(c) = key {
                    if c.is_ascii_lowercase() {
                        key = KeyCode::Char(c.to_ascii_uppercase());
                    }
                }
            }

            let key_comb = KeyCombination::new(key, modifiers);
            self.add_binding(key_comb, command, mode);
        }
        Ok(())
    }
}

pub fn parse_mode(s: &str) -> VimMode {
    match s.to_lowercase().as_str() {
        "normal" => VimMode::Normal,
        "insert" => VimMode::Insert,
        _ => VimMode::Any
    }
}

pub fn parse_key_code(key: &str) -> Result<KeyCode> {
    match key {
        "a" => Ok(KeyCode::Char('a')),
        "b" => Ok(KeyCode::Char('b')),
        "c" => Ok(KeyCode::Char('c')),
        "d" => Ok(KeyCode::Char('d')),
        "e" => Ok(KeyCode::Char('e')),
        "f" => Ok(KeyCode::Char('f')),
        "g" => Ok(KeyCode::Char('g')),
        "h" => Ok(KeyCode::Char('h')),
        "i" => Ok(KeyCode::Char('i')),
        "j" => Ok(KeyCode::Char('j')),
        "k" => Ok(KeyCode::Char('k')),
        "l" => Ok(KeyCode::Char('l')),
        "m" => Ok(KeyCode::Char('m')),
        "n" => Ok(KeyCode::Char('n')),
        "o" => Ok(KeyCode::Char('o')),
        "p" => Ok(KeyCode::Char('p')),
        "q" => Ok(KeyCode::Char('q')),
        "r" => Ok(KeyCode::Char('r')),
        "s" => Ok(KeyCode::Char('s')),
        "t" => Ok(KeyCode::Char('t')),
        "u" => Ok(KeyCode::Char('u')),
        "v" => Ok(KeyCode::Char('v')),
        "w" => Ok(KeyCode::Char('w')),
        "x" => Ok(KeyCode::Char('x')),
        "y" => Ok(KeyCode::Char('y')),
        "z" => Ok(KeyCode::Char('z')),
        "enter" => Ok(KeyCode::Enter),
        "backspace" => Ok(KeyCode::Backspace),
        "up" => Ok(KeyCode::Up),
        "down" => Ok(KeyCode::Down),
        "left" => Ok(KeyCode::Left),
        "right" => Ok(KeyCode::Right),
        "tab" => Ok(KeyCode::Tab),
        "esc" | "escape" => Ok(KeyCode::Esc),
        _ => Err(anyhow!("Unknown key code: {}", key)),
    }
}

pub fn parse_key_modifiers(mods: &[String]) -> Result<KeyModifiers> {
    let mut modifiers = KeyModifiers::NONE;
    for m in mods {
        match m.to_lowercase().as_str() {
            "control" | "ctrl" => modifiers |= KeyModifiers::CONTROL,
            "alt" => modifiers |= KeyModifiers::ALT,
            "shift" => modifiers |= KeyModifiers::SHIFT,
            _ => return Err(anyhow!("Unknown key modifier: {}", m)),
        }
    }
    Ok(modifiers)
}

pub fn parse_blink_command(cmd: &str) -> Result<BlinkCommand> {
    match cmd.to_lowercase().as_str() {
        "quit" => Ok(BlinkCommand::Quit),
        "togglefocus" => Ok(BlinkCommand::ToggleFocus),
        "movecursorup" => Ok(BlinkCommand::MoveCursorUp),
        "movecursordown" => Ok(BlinkCommand::MoveCursorDown),
        "movecursorleft" => Ok(BlinkCommand::MoveCursorLeft),
        "movecursorright" => Ok(BlinkCommand::MoveCursorRight),
        "deletebackward" => Ok(BlinkCommand::DeleteBackward),
        "deleteforward" => Ok(BlinkCommand::DeleteForward),
        "enterinsertmode" => Ok(BlinkCommand::EnterInsertMode),
        "enternormalmode" => Ok(BlinkCommand::EnterNormalMode),
        _ => Err(anyhow!("Unknown BlinkCommand: {}", cmd)),
    }
}
