use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::PathBuf};

use anyhow::{Context, Result};

/// This represents a single http request.
///
/// TODO: Put all this network related stuff in the `networks` crate.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct HTTPRequest {
    pub name: String,
    pub url: String,
    pub body: String,

    pub method: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct LocalRequests {
    pub requests: Vec<HTTPRequest>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct KeybindingConfig {
    pub key: String,            // Example: "b".
    pub modifiers: Vec<String>, // Example: ["Control"].
    pub command: String,        // Example: "MoveCursorLeft".
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_mode() -> String {
    "any".to_string()
}

/// This struct represents all configurations that Blink supports.
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct BlinkConfig {
    #[serde(default)]
    pub local_requests: LocalRequests,
    #[serde(default)]
    pub keybindings: Vec<KeybindingConfig>,
    #[serde(default = "default_true")]
    pub vim_mode: bool,
}

fn default_true() -> bool {
    true
}

impl BlinkConfig {
    /// Loads and merges global and local configurations.
    pub fn load() -> Result<Self> {
        let local_config = Self::load_local_config()?;

        Ok(local_config)
    }

    /// Loads the local configuration from './blink.toml'.
    fn load_local_config() -> Result<Self> {
        let config_path = PathBuf::from("blink.toml");

        if config_path.exists() {
            let content = read_to_string(&config_path)
                .with_context(|| format!("ERROR: Reading local config at: {:?}", config_path))?;

            let config: BlinkConfig =
                toml::from_str(&content).with_context(|| format!("ERROR: Parsing config."))?;

            Ok(config)
        } else {
            Ok(BlinkConfig::default())
        }
    }
}
