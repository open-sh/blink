use std::{collections::HashMap, hash::Hash};

use crossterm::event::{KeyCode, KeyModifiers};

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
    bindings: HashMap<KeyCombination, BlinkCommand>
}

impl KeybindingMap {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn add_binding(&mut self, key_comb: KeyCombination, command: BlinkCommand) {
        self.bindings.insert(key_comb, command);
    }

    pub fn get_command(&self, key_comb: KeyCombination) -> Option<&BlinkCommand> {
        self.bindings.get(&key_comb)
    }
}
