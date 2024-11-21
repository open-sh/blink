use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};

use anyhow::{Context, Result};

/// This struct represents all configurations that Blink supports.
///
/// NOTE: All config properties should probably be an `Option<>`.
#[derive(Default, Deserialize, Debug)]
pub struct BlinkConfig {
    pub mock: Option<String>,
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
